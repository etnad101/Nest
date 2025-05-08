use std::{cell::RefCell, char::MAX, io::Read, rc::Rc};


pub const WINDOW_TITLE: &str = "Nest";
pub const WINDOW_WIDTH: usize = 256;
pub const WINDOW_HEIGHT: usize = 240;
const MAX_CYCLES_PER_FRAME: usize = cpu::CLOCK_SPEED / 60;

use crate::{
    bus::Bus,
    cartridge::{self, Cartridge},
    cpu::{self, Cpu},
};

pub struct EmulatorState {
    cpu_cycles: usize,
    cpu_r_a: u8,
    cpu_r_x: u8,
    cpu_r_y: u8,
    cpu_r_sp: u8,
    cpu_r_pc: u16,
    cpu_f_c: bool,
    cpu_f_z: bool,
    cpu_f_i: bool,
    cpu_f_d: bool,
    cpu_f_v: bool,
    cpu_f_n: bool,
}

#[derive(PartialEq, Eq)]
pub enum DebugMode {
    CPU,
    PPU,
    STEP
}

pub struct Emulator {
    running: bool,
    bus: Rc<RefCell<Bus>>,
    cpu: Cpu,
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
            cpu: Cpu::new(bus.clone()),
            cycles_this_frame: 0,
        }
    }

    pub fn set_debug_mode(&mut self, debug: Vec<DebugMode>) {
        self.debug = debug;
        self.cpu.set_debug_mode(false);

        for mode in &self.debug {
            match mode {
                DebugMode::CPU => self.cpu.set_debug_mode(true),
                DebugMode::PPU => (),
                DebugMode::STEP => (), 
            }
        }
    }

    pub fn load_cartridge(&mut self, cartridge: Cartridge) {
        self.bus.borrow_mut().load_cartridge(cartridge);
    }

    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.cpu.reset();
    }

    pub fn tick(&mut self) {
        self.cycles_this_frame += self.cpu.tick();
        if self.cycles_this_frame >= MAX_CYCLES_PER_FRAME {
            self.cycles_this_frame = 0;
            // Do some waiting to cap to 60fps
        }
    }

    pub fn run(&mut self) {
        self.cpu.reset();
        self.running = true;

        while self.running {
            if self.debug.contains(&DebugMode::STEP) {
                // TODO: make something pause until supposed to step
            }
            self.tick();
        }
    }
}
