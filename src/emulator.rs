use std::{cell::RefCell, rc::Rc};

use simple_graphics::display::Display;

use crate::{
    bus::Bus,
    cartridge::{self, Cartridge},
    cpu::{self, Cpu},
    ppu::Ppu
};

pub enum DebugMode {
    CPU,
    PPU,
}

pub struct Emulator<'a> {
    debug: Vec<DebugMode>,
    running: bool,
    display: &'a mut Display,
    bus: Rc<RefCell<Bus>>,
    cpu: Cpu,
    ppu: Ppu,
}

impl<'a> Emulator<'a> {
    pub fn new(display: &'a mut Display) -> Self {
        let bus = Rc::new(RefCell::new(Bus::new()));
        Self {
            debug: vec![],
            running: false,
            display,
            bus: bus.clone(),
            cpu: Cpu::new(bus.clone()),
            ppu: Ppu::new(bus.clone()),
        }
    }

    pub fn set_debug_mode(&mut self, debug: Vec<DebugMode>) {
        self.cpu.set_debug_mode(false);
        self.ppu.set_debug_mode(false);
        for mode in debug {
            match mode {
                DebugMode::CPU => self.cpu.set_debug_mode(true),
                DebugMode::PPU => self.ppu.set_debug_mode(true),
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
                self.display.render().unwrap();
                self.display.set_buffer(self.ppu.get_frame());
            }
        }
    }
}
