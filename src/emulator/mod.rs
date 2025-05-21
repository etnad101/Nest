mod apu;
mod bus;
pub mod cartridge;
pub mod cpu;
mod io;
mod ppu;

use crate::emulator::{bus::Bus, cartridge::Cartridge, cpu::Cpu};
use std::{cell::RefCell, rc::Rc};

pub const NES_WIDTH: usize = 256;
pub const NES_HEIGHT: usize = 240;
pub const PATTERN_TABLE_WIDTH: usize = 8 * 16;
pub const PATTERN_TABLE_HEIGHT: usize = 8 * 32;
pub const MAX_CYCLES_PER_FRAME: usize = cpu::CLOCK_SPEED / 60;

pub struct FrameBuffer {
    buf: Vec<u32>,
}

impl FrameBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            buf: vec![0; width * height],
        }
    }

    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }

    pub fn read(&self, index: usize) -> u32 {
        self.buf[index]
    }

    pub fn write(&mut self, index: usize, value: u32) {
        self.buf[index] = value;
    }

    pub fn raw(&self) -> &Vec<u32> {
        &self.buf
    }

    pub fn rgb(&self) -> Vec<u8> {
        let mut result = Vec::with_capacity(self.buf.len() * 3);
        for &pixel in &self.buf {
            let r = ((pixel & 0xFF0000) >> 16) as u8;
            let g = ((pixel & 0x00FF00) >> 8) as u8;
            let b = (pixel & 0x0000FF) as u8;

            // Push the RGB components to the result vector
            result.push(r);
            result.push(g);
            result.push(b);
        }
        result
    }
}

pub struct CpuState {
    pub cycles: usize,
    pub r_a: u8,
    pub r_x: u8,
    pub r_y: u8,
    pub r_sp: u8,
    pub r_pc: u16,
    pub f_c: bool,
    pub f_z: bool,
    pub f_i: bool,
    pub f_d: bool,
    pub f_v: bool,
    pub f_n: bool,
    pub p: u8,
}

#[derive(PartialEq, Eq)]
pub enum DebugFlag {
    Cpu,
    Ppu,
    Step,
    Json
}

pub struct Emulator {
    bus: Rc<RefCell<Bus>>,
    cpu: Cpu,

    running: bool,
    debug: Vec<DebugFlag>,
    cycles_this_frame: usize,
}

impl Emulator {
    pub fn new() -> Self {
        let bus = Rc::new(RefCell::new(Bus::new()));

        Self {
            debug: vec![],
            running: false,
            bus: bus.clone(),
            cpu: Cpu::new(bus),
            cycles_this_frame: 0,
        }
    }

    fn update_internal_debug_mode(&mut self) {
        self.bus.borrow_mut().json_test_mode = false;
        self.cpu.set_debug_mode(false);
        self.bus.borrow_mut().set_ppu_debug_mode(false);

        for mode in &self.debug {
            match mode {
                DebugFlag::Json => {
                    self.bus.borrow_mut().json_test_mode = true;
                    return
                }
                DebugFlag::Cpu => self.cpu.set_debug_mode(true),
                DebugFlag::Ppu => self.bus.borrow_mut().set_ppu_debug_mode(true),
                DebugFlag::Step => (),
            }
        }
    }

    pub fn set_debug_flags(&mut self, debug: Vec<DebugFlag>) {
        self.debug = debug;
        self.update_internal_debug_mode();
    }

    pub fn set_debug_flag(&mut self, flag: DebugFlag) {
        if !self.debug.contains(&flag) {
            self.debug.push(flag);
        }
        self.update_internal_debug_mode();
    }

    pub fn clear_debug_flag(&mut self, flag: &DebugFlag) {
        self.debug.retain(|x| x != flag);
        self.update_internal_debug_mode();
    }

    pub fn toggle_debug_flag(&mut self, flag: DebugFlag) {
        if self.debug.contains(&flag) {
            self.clear_debug_flag(&flag);
        } else {
            self.set_debug_flag(flag);
        }
    }

    pub fn running(&self) -> bool {
        self.running
    }

    pub fn stop(&mut self) {
        self.running = false;
    }

    pub fn get_logged_instr(&self) -> String {
        self.cpu.get_logged_instr()
    }

    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.cpu.reset();
    }


    pub fn load_cartridge(&mut self, cartridge: Cartridge) {
        self.bus.borrow_mut().load_cartridge(cartridge);
    }

    pub fn tick<T>(&mut self, handle_display: &mut T)
    where
        T: FnMut(&FrameBuffer, FrameBuffer),
    {
        let cycles = self.cpu.tick();
        self.cycles_this_frame += cycles;

        for _ in 0..cycles * 3 {
            self.bus.borrow_mut().tick_ppu();
        }

        if self.cycles_this_frame >= MAX_CYCLES_PER_FRAME {
            self.cycles_this_frame = 0;
            // Do some waiting to cap to 60 fps
            self.bus.borrow_mut().draw_nametable();

            handle_display(
                self.bus.borrow().get_frame(),
                self.bus.borrow().get_pattern_table(),
            );
        }
    }

    pub fn step_frame<T>(&mut self, handle_display: &mut T)
    where
        T: FnMut(&FrameBuffer, FrameBuffer),
    {
        while self.cycles_this_frame < MAX_CYCLES_PER_FRAME {
            let cycles = self.cpu.tick();
            self.cycles_this_frame += cycles;

            for _ in 0..cycles * 3 {
                self.bus.borrow_mut().tick_ppu();
            }
        }

        self.cycles_this_frame = 0;
        // Do some waiting to cap to 60 fps
        self.bus.borrow_mut().draw_nametable();

        handle_display(
            self.bus.borrow().get_frame(),
            self.bus.borrow().get_pattern_table(),
        );
    }

    pub fn run_with_callback<T>(&mut self, handle_display: &mut T)
    where
        T: FnMut(&FrameBuffer, FrameBuffer),
    {
        self.cpu.reset();
        self.running = true;

        while self.running {
            if self.debug.contains(&DebugFlag::Step) {
                // TODO: make something pause until supposed to step
            }
            self.tick(handle_display);
        }
    }

    /* Testing functions */
    pub fn get_state(&self) -> CpuState {
        self.cpu.get_state()
    }

    pub fn load_state(&mut self, state: CpuState) {
        self.cpu.load_state(state);
    }

    pub fn tick_cpu(&mut self) {
        self.cpu.tick();
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        self.bus.borrow_mut().write(addr, value);
    }

    pub fn read(&mut self, addr: u16) -> u8 {
        self.bus.borrow_mut().cpu_read(addr)
    }
}
