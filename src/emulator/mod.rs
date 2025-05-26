mod apu;
mod bus;
pub mod cartridge;
pub mod cpu;
pub mod debug;
mod io;
mod ppu;

use cpu::{BreakpointType, CpuStatus};
use debug::DebugContext;

use crate::emulator::{bus::Bus, cartridge::Cartridge, cpu::Cpu};
use crate::frame_buffer::FrameBuffer;
use std::{cell::RefCell, rc::Rc};

pub const NES_WIDTH: usize = 256;
pub const NES_HEIGHT: usize = 240;
pub const PATTERN_TABLE_WIDTH: usize = 8 * 16;
pub const PATTERN_TABLE_HEIGHT: usize = 8 * 32;
pub const MAX_CYCLES_PER_FRAME: usize = cpu::CLOCK_SPEED / 60;

// struct to cleanly pass atround cpu state
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

#[allow(unused)]
pub struct Emulator {
    cpu: Cpu,
    bus: Rc<RefCell<Bus>>,
    debug_ctx: Rc<RefCell<DebugContext>>,

    running: bool,
    cycles_this_frame: usize,
}

#[allow(unused)]
impl Emulator {
    pub fn new() -> Self {
        let debug_ctx = Rc::new(RefCell::new(DebugContext::new()));
        let bus = Rc::new(RefCell::new(Bus::new(debug_ctx.clone())));

        Self {
            cpu: Cpu::new(bus.clone(), debug_ctx.clone()),
            bus,
            debug_ctx,

            running: false,
            cycles_this_frame: 0,
        }
    }

    pub fn debug_ctx(&self) -> Rc<RefCell<DebugContext>> {
        self.debug_ctx.clone()
    }

    pub fn running(&self) -> bool {
        self.running
    }

    pub fn start(&mut self) {
        self.running = true;
    }

    pub fn stop(&mut self) {
        self.running = false;
    }

    // reset cpu
    pub fn reset(&mut self) {
        self.cpu.reset();
    }

    // load cartrtridge into bus
    pub fn load_cartridge(&mut self, cartridge: Cartridge) {
        self.bus.borrow_mut().load_cartridge(cartridge);
    }

    // tick emulator one instruction
    pub fn tick<T>(&mut self, handle_display: &mut T) -> Option<BreakpointType>
    where
        T: FnMut(&FrameBuffer, FrameBuffer),
    {
        let cycles = match self.cpu.tick() {
            CpuStatus::Normal(cycles) => cycles,
            CpuStatus::BreakpointHit(bp) => return Some(bp),
        };

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

        None
    }

    // tick emulator to the next frame
    pub fn step_frame<T>(&mut self, handle_display: &mut T) -> Option<BreakpointType>
    where
        T: FnMut(&FrameBuffer, FrameBuffer),
    {
        while self.cycles_this_frame < MAX_CYCLES_PER_FRAME {
            let cycles = match self.cpu.tick() {
                CpuStatus::Normal(cycles) => cycles,
                CpuStatus::BreakpointHit(bp) => return Some(bp),
            };
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
        None
    }

    // run while emulator is set to running
    // This should be used as its own event loop
    pub fn run_standalone<T>(&mut self, handle_display: &mut T) -> Option<BreakpointType>
    where
        T: FnMut(&FrameBuffer, FrameBuffer),
    {
        self.running = true;

        while self.running {
            if let Some(bp) = self.tick(handle_display) {
                self.running = false;
                return Some(bp);
            };
        }
        None
    }

    // to be run in another loop
    pub fn run_in_loop<T>(&mut self, handle_display: &mut T) -> Option<BreakpointType>
    where
        T: FnMut(&FrameBuffer, FrameBuffer),
    {
        if self.running {
            if self
                .debug_ctx
                .borrow()
                .flag_enabled(&debug::DebugFlag::Step(debug::StepMode::Frame))
            {
                self.running = false;
                if let Some(bp) = self.step_frame(handle_display) {
                    return Some(bp);
                };
                return Some(BreakpointType::AfterFrame);
            }

            if self
                .debug_ctx
                .borrow()
                .flag_enabled(&debug::DebugFlag::Step(debug::StepMode::Instruction))
            {
                self.running = false;
                if let Some(bp) = self.tick(handle_display) {
                    return Some(bp);
                };
                return Some(BreakpointType::AfterInstruction);
            }

            if let Some(bp) = self.step_frame(handle_display) {
                self.running = false;
                return Some(bp);
            };
        }
        None
    }

    /* functions to expose internal functions */
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
