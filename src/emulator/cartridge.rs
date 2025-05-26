use std::{error::Error, fs};

const PRG_ROM_SIZE_ADDR: usize = 4;
const CHR_ROM_SIZE_ADDR: usize = 5;

const PRG_ROM_CHUNK_SIZE: usize = 0x4000;
const CHR_ROM_CHUNK_SIZE: usize = 0x2000;

enum Mapper {}

enum CartridgeType {
    Ines,
    NES2,
}

pub struct Cartridge {
    prg_rom_size: usize,
    prg_ram_size: usize,
    chr_rom_size: usize,
    chr_ram_size: usize,

    // nametable_arrangement: bool,
    // persistant_memory: bool,
    // has_trainer: bool,
    // alt_nametable_layout: bool,

    // mapper: Mapper,
    // cartridge_type: CartridgeType,
    prg_rom: Vec<u8>,
    prg_ram: Vec<u8>,
    chr_rom: Vec<u8>,
    chr_ram: Vec<u8>,
}

impl Cartridge {
    // creates new cartridge from file
    pub fn new(rom_path: String) -> Result<Self, Box<dyn Error>> {
        let rom = fs::read(rom_path)?;

        // make sure file is actually a nes rom
        if rom[0] != 0x4e && rom[1] != 0x45 && rom[2] != 0x53 && rom[3] != 0x1a {
            return Err(
                "Invalid file type. Please provide a valid NES rom (INES or NES2.0)".into(),
            );
        }

        // extract program size and character rom size from header

        if rom[CHR_ROM_SIZE_ADDR] == 0 {
            panic!("Rom header says its using chr RAM, instrad of ROM")
        }

        let prg_rom_size = rom[PRG_ROM_SIZE_ADDR] as usize * PRG_ROM_CHUNK_SIZE;
        let chr_rom_size = rom[CHR_ROM_SIZE_ADDR] as usize * CHR_ROM_CHUNK_SIZE;

        // println!("prg rom size: {:06x}, chr rom size: {:06x}", prg_rom_size, chr_rom_size);

        // load program and character rom into vectors
        let prg_rom: Vec<u8> = (rom[0x10..0x10 + prg_rom_size]).to_owned();
        let chr_rom: Vec<u8> =
            (rom[0x10 + prg_rom_size..0x10 + chr_rom_size + prg_rom_size]).to_owned();

        let cartridge = Self {
            prg_rom_size,
            prg_ram_size: 0,
            chr_rom_size,
            chr_ram_size: 0,
            prg_rom,
            prg_ram: Vec::new(),
            chr_rom,
            chr_ram: Vec::new(),
        };

        Ok(cartridge)
    }

    // read from single value from program rom
    pub fn get_prg_rom(&self, addr: u16) -> u8 {
        let mut index = (addr - 0x8000) as usize;
        if self.prg_rom_size == PRG_ROM_CHUNK_SIZE {
            index &= 0x3FFF;
        }
        if index >= self.prg_rom_size {
            panic!("Attempted to read outside of prg_rom");
        }
        self.prg_rom[index]
    }

    // read single value from characger rom
    pub fn get_chr_rom(&self, addr: u16) -> u8 {
        let index = addr as usize;
        if index >= self.chr_rom_size {
            panic!("Attempted to read outside of chr_rom");
        }
        self.chr_rom[index]
    }
}
