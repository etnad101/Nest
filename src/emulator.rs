use std::{cell::RefCell, rc::Rc};


pub const WINDOW_TITLE: &str = "Nest";
pub const WINDOW_WIDTH: usize = 256;
pub const WINDOW_HEIGHT: usize = 240;

use crate::{
    bus::Bus,
    cartridge::{self, Cartridge},
    cpu::{self, Cpu},
};

pub enum DebugMode {
    CPU,
    PPU,
}

pub struct Emulator {
    // main_display: Display,
    running: bool,
    bus: Rc<RefCell<Bus>>,
    cpu: Cpu,
    debug: Vec<DebugMode>,
}

impl Emulator {
    pub fn new() -> Self {
        let bus = Rc::new(RefCell::new(Bus::new()));

        Self {
            debug: vec![],
            running: false,
            // main_display: display,
            bus: bus.clone(),
            cpu: Cpu::new(bus.clone()),
        }
    }

    pub fn set_debug_mode(&mut self, debug: Vec<DebugMode>) {
        self.debug = debug;
        self.cpu.set_debug_mode(false);

        for mode in &self.debug {
            match mode {
                DebugMode::CPU => self.cpu.set_debug_mode(true),
                DebugMode::PPU => ()
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

    pub fn run(&mut self) {
        self.cpu.reset();
        self.running = true;

        let mut cycles_this_frame: usize = 0;
        let max_cycles_per_frame = cpu::CLOCK_SPEED / 60;

        while self.running {
            cycles_this_frame += self.cpu.tick();
            if cycles_this_frame >= max_cycles_per_frame {
                cycles_this_frame = 0;
                // Do some waiting to cap to 60fps

            }
        }
    }
}
