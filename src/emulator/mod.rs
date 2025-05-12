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
const MAX_CYCLES_PER_FRAME: usize = cpu::CLOCK_SPEED / 60;

type FrameBuffer = Box<[u32; NES_WIDTH * NES_HEIGHT]>;

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

    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.cpu.reset();
    }

    pub fn load_cartridge(&mut self, cartridge: Cartridge) {
        self.bus.borrow_mut().load_cartridge(cartridge);
    }

    pub fn tick<T>(&mut self, handle_display: &mut T)
    where
        T: FnMut((&FrameBuffer, Vec<u32>)) -> (),
    {
        let cycles = self.cpu.tick();
        self.cycles_this_frame += cycles;

        for _ in 0..cycles * 3 {
            self.bus.borrow_mut().tick_ppu();
        }

        if self.cycles_this_frame >= MAX_CYCLES_PER_FRAME {
            self.cycles_this_frame = 0;
            // Do some waiting to cap to 60fps
            self.bus.borrow_mut().draw_nametable();

            handle_display(self.bus.borrow().get_frame_and_pattern_table())
        }
    }

    pub fn run_with_callback<T>(&mut self, handle_display: &mut T)
    where
        T: FnMut((&FrameBuffer, Vec<u32>)) -> (),
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
