use std::{fs, io::Read};

const PRG_ROM_SIZE_ADDR: usize = 4;
const CHR_ROM_SIZE_ADDR: usize = 4;

const PRG_ROM_CHUNK_SIZE: usize = 0x4000;
const CHR_ROM_CHUNK_SIZE: usize = 0x2000;

enum Mapper {

}

enum CartridgeType {
    INES,
    NES2
}

pub struct Cartridge {
    prg_rom_size: usize,
    chr_rom_size: usize,

    // nametable_arrangement: bool,
    // persistant_memory: bool,
    // has_trainer: bool,
    // alt_nametable_layout: bool,

    // mapper: Mapper,
    // cartridge_type: CartridgeType,

    prg_rom: Vec<u8>,
    chr_rom: Vec<u8>,
}

impl Cartridge {
    pub fn new(rom_path: String) -> Self {
        let rom = match fs::read(rom_path) {
            Ok(r) => r,
            Err(e) => panic!("Unable to read rom from specified path\n\n{}", e),
        };

        if rom[0] != 0x4e && rom[1] != 0x45 && rom[2] != 0x53 && rom[3] != 0x1a {
            panic!("Invalid file type. Please provide a valid NES rom (INES or NES2.0)");
        } 

        let prg_rom_size = rom[PRG_ROM_SIZE_ADDR] as usize * PRG_ROM_CHUNK_SIZE;
        let chr_rom_size = rom[CHR_ROM_SIZE_ADDR] as usize * CHR_ROM_CHUNK_SIZE;

        println!("prg rom size: {:06x}, chr rom size: {:06x}", prg_rom_size, chr_rom_size);

        let prg_rom: Vec<u8> = (&rom[0x10..0x10 + prg_rom_size]).to_owned();
        let chr_rom: Vec<u8> = (&rom[0x10 + prg_rom_size..0x10 + chr_rom_size + prg_rom_size]).to_owned();

        Self {
            prg_rom_size,
            chr_rom_size,
            prg_rom,
            chr_rom,
        }
    }

    pub fn get_prg_rom(&self, index: usize) -> u8 {
        let mut index = index;
        if self.prg_rom_size == PRG_ROM_CHUNK_SIZE {
            index &= 0x3FFF;
        }
        if index >= self.prg_rom_size {
            panic!("Attempted to read outside of prg_rom");
        }
        self.prg_rom[index]
    }

    pub fn get_chr_rom(&self, index: usize) -> u8 {
        if index >= self.chr_rom_size {
            panic!("Attempted to read outside of chr_rom");
        }
        self.prg_rom[index]
    }
}