mod apu;
mod bus;
pub mod cartridge;
mod cpu;
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
        for &pixel in self.buf.iter() {
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

pub struct EmulatorState {
    pub cpu_cycles: usize,
    pub cpu_r_a: u8,
    pub cpu_r_x: u8,
    pub cpu_r_y: u8,
    pub cpu_r_sp: u8,
    pub cpu_r_pc: u16,
    pub cpu_f_c: bool,
    pub cpu_f_z: bool,
    pub cpu_f_i: bool,
    pub cpu_f_d: bool,
    pub cpu_f_v: bool,
    pub cpu_f_n: bool,
}

#[derive(PartialEq, Eq)]
pub enum DebugMode {
    Cpu,
    Ppu,
    Step,
}

pub struct Emulator {
    bus: Rc<RefCell<Bus>>,
    cpu: Cpu,

    running: bool,
    debug: Vec<DebugMode>,
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

    pub fn set_debug_mode(&mut self, debug: Vec<DebugMode>) {
        self.debug = debug;
        self.cpu.set_debug_mode(false);
        self.bus.borrow_mut().set_ppu_debug_mode(false);

        for mode in &self.debug {
            match mode {
                DebugMode::Cpu => self.cpu.set_debug_mode(true),
                DebugMode::Ppu => self.bus.borrow_mut().set_ppu_debug_mode(true),
                DebugMode::Step => (),
            }
        }
    }

    pub fn running(&self) -> bool {
        self.running
    }

    pub fn stop(&mut self) {
        self.running = false;
    }

    pub fn get_state(&self) -> EmulatorState {
        self.cpu.get_state()
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
            )
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
        )
    }

    pub fn run_with_callback<T>(&mut self, handle_display: &mut T)
    where
        T: FnMut(&FrameBuffer, FrameBuffer),
    {
        self.cpu.reset();
        self.running = true;

        while self.running {
            if self.debug.contains(&DebugMode::Step) {
                // TODO: make something pause until supposed to step
            }
            self.tick(handle_display);
        }
    }
}
