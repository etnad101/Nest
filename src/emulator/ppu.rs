const VRAM_SIZE: usize = 0x800;

enum PpuRegs {
    PpuCtrl,
    PpuMask,
    PpuStatus,
    OamAddr,
    OamData,
    PpuScroll,
    PpuAddr,
    PpuData,
    OamDma,
}

impl PpuRegs {
    fn addr(&self) -> u16 {
        match self {
            PpuRegs::PpuCtrl => 0x2000,
            PpuRegs::PpuMask => 0x2001,
            PpuRegs::PpuStatus => 0x2002,
            PpuRegs::OamAddr => 0x2003,
            PpuRegs::OamData => 0x2004,
            PpuRegs::PpuScroll => 0x2005,
            PpuRegs::PpuAddr => 0x2006,
            PpuRegs::PpuData => 0x2007,
            PpuRegs::OamDma => 0x4014,
        }
    }
}

pub struct Ppu<'a> {
    vram: Box<[u8; VRAM_SIZE]>,
    chr_rom: Option<&'a mut Vec<u8>>,
    chr_ram: Option<&'a mut Vec<u8>>,
    ppu_ctrl: u8,
    ppu_mask: u8,
    ppu_status: u8,
    oam_addr: u8,
    oam_data: u8,
    ppu_scroll: u16, // This says its 16 bits but its written in 2 8 bit writes
    ppu_addr: u16,   // This says its 16 bits but its written in 2 8 bit writes
    ppu_data: u8,
    oam_dma: u8,
}

impl Ppu<'_> {
    pub fn new() -> Self {
        Self {
            vram: Box::new([0; VRAM_SIZE]),
            chr_rom: None,
            chr_ram: None,
            ppu_ctrl: 0,
            ppu_mask: 0,
            ppu_status: 0,
            oam_addr: 0,
            oam_data: 0,
            ppu_scroll: 0, // This says its 16 bits but its written in 2 8 bit writes
            ppu_addr: 0,   // This says its 16 bits but its written in 2 8 bit writes
            ppu_data: 0,
            oam_dma: 0,
        }
    }

    pub fn read_reg(&self, addr: u16) -> u8 {
        match addr {
            0x2000 => self.ppu_ctrl,
            0x2001 => self.ppu_mask,
            0x2002 => self.ppu_status,
            0x2003 => self.oam_addr,
            0x2004 => self.ppu_data,
            0x2005 => unimplemented!("PPU reg 0x2005 isn't implemented yet"),
            0x2006 => unimplemented!("PPU reg 0x2006 isn't implemented yet"),
            0x2007 => self.ppu_data,
            0x4014 => self.oam_data,
            _ => panic!("Address (${:04X}) is not a ppu register", addr),
        }
    }

    pub fn write_reg(&mut self, addr: u16, value: u8) {
        match addr {
            0x2000 => self.ppu_ctrl = value,
            0x2001 => self.ppu_mask = value,
            0x2002 => self.ppu_status = value,
            0x2003 => self.oam_addr = value,
            0x2004 => self.ppu_data = value,
            0x2005 => unimplemented!("PPU reg 0x2005 isn't implemented yet"),
            0x2006 => unimplemented!("PPU reg 0x2006 isn't implemented yet"),
            0x2007 => self.ppu_data = value,
            0x4014 => self.oam_data = value,
            _ => panic!("Address given is not a ppu regiter"),
        }
    }
}
