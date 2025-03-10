use crate::cartridge::Cartridge;

const RAM_SIZE: usize = 0x800;
const VRAM_SIZE: usize = 0x800;
const PPU_REGS_SIZE: usize = 8;
const APU_IO_REGS_SIZE: usize = 0x18;

pub struct Bus {
    ram: [u8; RAM_SIZE],
    ppu_regs: [u8; PPU_REGS_SIZE],
    apu_io_regs: [u8; APU_IO_REGS_SIZE],
    vram: [u8; VRAM_SIZE],
    cartridge_inserted: bool,
    cartridge: Option<Cartridge>,
}

impl Bus {
    pub fn new() -> Self {
        Self {
            ram: [0; RAM_SIZE],
            ppu_regs: [0; PPU_REGS_SIZE],
            apu_io_regs: [0; APU_IO_REGS_SIZE],
            vram: [0; VRAM_SIZE],
            cartridge_inserted: false,
            cartridge: None,
        }
    }

    pub fn loadCartridge(&mut self, cartridge: Cartridge) {
        self.cartridge = Some(cartridge);
        self.cartridge_inserted = true;
    }

    pub fn cpu_read(&self, addr: u16) -> u8 {
        if (addr & 0xE000) == 0 {
            return self.ram[(addr & 0x07FF) as usize];
        }
        if addr >= 0x2000 && addr <= 0x3FFF {
            return self.ppu_regs[(addr & 7) as usize];
        }
        if addr >= 0x4000 && addr <= 0x4017 {
            return self.apu_io_regs[(addr - 0x4000) as usize];
        }
        if addr >= 0x4018 && addr <= 0x401F {
            return 0; // APU and I/O functionality that is normally disabled, I think this should return 0
        }
        if addr >= 0x4020 && addr <= 0x5FFF {
            // TODO: figure out what to do here
            return 0;
        }
        if addr >= 0x6000 && addr <= 0x7FFF {
            // TODO: Make this return cartridge ram
            return 0;
        }
        if self.cartridge_inserted {
            let cartridge = self.cartridge.as_ref();
            return cartridge.unwrap().get_prg_rom((addr - 0x8000) as usize);
        }
        panic!("no cartridge inserted");
    }

    pub fn ppu_read(&self, addr: u16) -> u8 {
        todo!("ppu_read");
        // if (addr <= 0x1FFF) {
        //     return self.cartridge->getChrROM(addr);
        // }
        // if (addr <= 0x2FFF) {
        //     return self.vram[addr - 0x2FFF];
        // }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        if (addr & 0xE000) == 0 {
            self.ram[(addr & 0x07FF) as usize] = value;
            return;
        }
        if addr >= 0x2000 && addr <= 0x3FFF {
            self.ppu_regs[(addr & 7) as usize] = value;
            return;
        }
        if addr >= 0x4000 && addr <= 0x4017 {
            self.apu_io_regs[(addr - 0x4000) as usize] = value;
            return;
        }
        panic!("ERROR: Writing to something i haven't implemented yet")
    }
}
