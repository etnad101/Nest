use std::{cell::RefCell, rc::Rc};

use crate::{bus::Bus, cartridge::{self, Cartridge}, cpu::Cpu};

pub struct Emulator {
    debug: bool,
    running: bool,
    bus: Rc<RefCell<Bus>>,
    cpu: Cpu,
    // ppu: Ppu,
}

impl Emulator {
    pub fn new(debug: bool) -> Self {
        let bus = Rc::new(RefCell::new(Bus::new()));
        Self {
            debug,
            running: false,
            bus: bus.clone(),
            cpu: Cpu::new(bus.clone(), debug)
        } 
    }

    pub fn load_cartridge(&mut self, cartridge: Cartridge) {
        self.bus.borrow_mut().loadCartridge(cartridge);
    }

    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.cpu.reset();
    }

    pub fn run(&mut self) {
        self.cpu.reset();
        self.running = true;

        let mut cycles_this_frame: usize = 0;

        while self.running {
            cycles_this_frame += self.cpu.tick();
            if cycles_this_frame > 100 {
                self.running = false;
            }
        }
    }
}