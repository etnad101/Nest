use std::{cell::RefCell, rc::Rc};
use egui::Frame;
use super::{apu::Apu, cartridge::Cartridge, io::Io, ppu::Ppu, FrameBuffer, NES_HEIGHT, NES_WIDTH};

pub const RAM_SIZE: usize = 0x800;

pub struct Bus {
    wram: Box<[u8; RAM_SIZE]>,
    ppu: Ppu,
    apu: Apu,
    io: Io,
    cartridge_inserted: bool,
    cartridge: Option<Rc<RefCell<Cartridge>>>,
}

impl Bus {
    pub fn new() -> Self {
        Self {
            wram: Box::new([0; RAM_SIZE]),
            ppu: Ppu::new(),
            apu: Apu::new(),
            io: Io::new(),
            cartridge_inserted: false,
            cartridge: None,
        }
    }

    pub fn load_cartridge(&mut self, cartridge: Cartridge) {
        let cartridge = Rc::new(RefCell::new(cartridge));
        self.cartridge = Some(cartridge.clone());
        self.ppu.set_cartridge(Some(cartridge));
        self.cartridge_inserted = true;
    }

    pub fn cpu_read(&mut self, addr: u16) -> u8 {
        // RAM & Mirrors $0000-$1FFF
        if addr < 0x2000 {
            return self.wram[(addr & 0x07FF) as usize];
        }
        // PPU Registers & Mirrors $2000-$3FFF
        if addr < 0x4000 {
            return self.ppu.read_reg(addr);
        }
        // APU and IO Reisters $4000-$4017
        if addr < 0x4018 {
            return self.apu.read_reg(addr);
        }
        // APU and I/O functionality that is normally disabled, I think this should return 0 $4018-401F
        if addr < 0x4020 {
            return 0;
        }
        // TODO: figure out what to do here
        if (0x4020..=0x5FFF).contains(&addr) {
            return 0;
        }
        // TODO: Make this return cartridge ram
        if (0x6000..=0x7FFF).contains(&addr) {
            unimplemented!("Should return cartridge ram")
        }
        // Cartridge rom
        if self.cartridge_inserted {
            return self.cartridge.as_ref().unwrap().borrow().get_prg_rom(addr);
        }
        panic!("no cartridge inserted");
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        if addr < 0x2000 {
            self.wram[(addr & 0x07FF) as usize] = value;
            return;
        }
        if addr < 0x3FFF {
            self.ppu.write_reg(addr, value);
            return;
        }
        if addr < 0x4018 {
            self.apu.write_reg(addr, value);
            return;
        }
        panic!("ERROR: Writing to something i haven't implemented yet")
    }

    // Expose PPU functions to Emulator
    pub fn tick_ppu(&mut self) {
        self.ppu.tick();
    }

    pub fn draw_nametable(&mut self) {
        self.ppu.draw_nametable();
    }

    pub fn get_frame(&self) -> &FrameBuffer {
        self.ppu.frame()
    }

    pub fn get_pattern_table(&self) -> FrameBuffer {
        self.ppu.pattern_table()
    }

    pub fn set_ppu_debug_mode(&mut self, mode: bool) {
        self.ppu.set_debug_mode(mode)
    }
}
