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
const MAX_CYCLES_PER_FRAME: usize = cpu::CLOCK_SPEED / 60;

#[derive(PartialEq, Eq)]
pub enum DebugMode {
    Cpu,
    Ppu,
    Step,
}

pub struct Emulator<'a> {
    running: bool,
    bus: Rc<RefCell<Bus<'a>>>,
    cpu: Cpu<'a>,
    debug: Vec<DebugMode>,

    cycles_this_frame: usize,
}

impl Emulator<'_> {
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
                DebugMode::Cpu => self.cpu.set_debug_mode(true),
                DebugMode::Ppu => (),
                DebugMode::Step => (),
            }
        }
    }

    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.cpu.reset();
    }

    pub fn load_cartridge(&mut self, cartridge: Cartridge) {
        self.bus.borrow_mut().load_cartridge(cartridge);
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
            if self.debug.contains(&DebugMode::Step) {
                // TODO: make something pause until supposed to step
            }
            self.tick();
        }
    }
}
