use std::{cell::RefCell, rc::Rc};

use super::{cartridge::Cartridge, debug::DebugContext, FrameBuffer, NES_HEIGHT, NES_WIDTH};

const VRAM_SIZE: usize = 0x800;
pub const PATTERN_TABLE_WIDTH: usize = 8 * 16;
pub const PATTERN_TABLE_HEIGHT: usize = 8 * 32;

// enum PpuRegs {
//     PpuCtrl,
//     PpuMask,
//     PpuStatus,
//     OamAddr,
//     OamData,
//     PpuScroll,
//     PpuAddr,
//     PpuData,
//     OamDma,
// }

// impl PpuRegs {
//     fn addr(&self) -> u16 {
//         match self {
//             PpuRegs::PpuCtrl => 0x2000,
//             PpuRegs::PpuMask => 0x2001,
//             PpuRegs::PpuStatus => 0x2002,
//             PpuRegs::OamAddr => 0x2003,
//             PpuRegs::OamData => 0x2004,
//             PpuRegs::PpuScroll => 0x2005,
//             PpuRegs::PpuAddr => 0x2006,
//             PpuRegs::PpuData => 0x2007,
//             PpuRegs::OamDma => 0x4014,
//         }
//     }
// }

pub struct Ppu {
    vram: Box<[u8; VRAM_SIZE]>,
    cartridge: Option<Rc<RefCell<Cartridge>>>,
    ppu_ctrl: u8,
    ppu_mask: u8,
    ppu_status: u8,
    oam_addr: u8,
    oam_data: u8,
    ppu_data: u8,
    oam_dma: u8,

    r_v: u16,
    r_t: u16,
    r_x: u8,
    r_w: u8,

    scanline: usize,
    dot: usize,

    frame_buffer: FrameBuffer,
    debug_ctx: Rc<RefCell<DebugContext>>,
}

impl Ppu {
    pub fn new(debug_ctx: Rc<RefCell<DebugContext>>) -> Self {
        Self {
            vram: Box::new([0; VRAM_SIZE]),
            cartridge: None,
            ppu_ctrl: 0,
            ppu_mask: 0,
            ppu_status: 0,
            oam_addr: 0,
            oam_data: 0,
            ppu_data: 0,
            oam_dma: 0,

            r_v: 0,
            r_t: 0,
            r_x: 0,
            r_w: 0,

            scanline: 0,
            dot: 0,

            frame_buffer: FrameBuffer::new(NES_WIDTH, NES_HEIGHT),
            debug_ctx,
        }
    }

    pub fn set_cartridge(&mut self, cartridge: Option<Rc<RefCell<Cartridge>>>) {
        self.cartridge = cartridge;
    }

    pub fn read_reg(&mut self, addr: u16) -> u8 {
        match addr {
            0x2000 => self.ppu_ctrl,
            0x2001 => self.ppu_mask,
            0x2002 => {
                let value = self.ppu_status;
                if self.debug_ctx.borrow().cpu_debug_read {
                    return value;
                }
                self.r_w = 0;
                self.ppu_status &= 0x7F;
                value
            }
            0x2003 => self.oam_addr,
            0x2004 => self.oam_data,
            0x2005 => unimplemented!("read PPU reg 0x2005 isn't implemented yet"),
            0x2006 => unimplemented!("read PPU reg 0x2006 isn't implemented yet"),
            0x2007 => self.ppu_data,
            0x4014 => self.oam_dma,
            _ => panic!("Address (${addr:04X}) is not a ppu register"),
        }
    }

    pub fn write_reg(&mut self, addr: u16, value: u8) {
        match addr {
            0x2000 => self.ppu_ctrl = value,
            0x2001 => self.ppu_mask = value,
            0x2002 => self.ppu_status = value,
            0x2003 => self.oam_addr = value,
            0x2004 => self.oam_data = value,
            0x2005 => {
                // alternate between low and high byte on each write
                let coarse = (value >> 3) as u16;
                if self.r_w == 0 {
                    self.r_t &= 0b0111_1111_1110_0000;
                    self.r_t |= coarse;
                    self.r_x = value & 3;
                } else {
                    self.r_t &= 0b0000_1100_0001_1111;
                    self.r_t |= coarse << 5;
                    self.r_t |= (value as u16 & 3) << 12;
                }
                self.r_w ^= 1;
            }
            0x2006 => {
                if self.r_w == 0 {
                    self.r_t = (value as u16 & 0x3F) << 8;
                } else {
                    self.r_t |= value as u16;
                    self.r_v = self.r_t;
                }
                self.r_w ^= 1;
            }
            0x2007 => {
                self.ppu_data = value;
                self.write(self.r_v, value);
                self.r_v += if self.ppu_ctrl & 0b100 > 0 { 32 } else { 1 };
            }
            0x4014 => self.oam_dma = value,
            _ => panic!("Address given is not a ppu regiter"),
        }
    }

    fn read(&self, addr: u16) -> u8 {
        if addr < 0x2000 {
            // let temp = vec![0x18, 0x38, 0x18, 0x18, 0x18, 0x18, 0x7E, 00, 00, 00, 00, 00, 00, 00, 00, 00];
            // return temp[addr as usize % 16];
            return self.cartridge.as_ref().unwrap().borrow().get_chr_rom(addr);
        }
        if addr < 0x3000 {
            return self.vram[addr as usize - 0x2000];
        }
        unimplemented!("ppu read");
    }

    fn write(&mut self, addr: u16, value: u8) {
        if addr < 0x2000 {
            unimplemented!("ppu write < 0x2000");
        }

        if addr < 0x3F00 {
            self.vram[addr as usize - 0x2000] = value;
        }
    }

    // one ppu cycle
    // should be called 3 times for every cpu cycle
    pub fn tick(&mut self) {
        if self.dot == 1 && self.scanline == 241 {
            self.ppu_status |= 0x80;
        }

        self.dot += 1;
        if self.dot > 340 {
            self.scanline += 1;
            self.dot = 0;
        }

        if self.scanline > 261 {
            self.scanline = 0;
        }
    }

    fn get_color(&self, x: u32) -> u32 {
        match x {
            0 => 0x000000,
            1 => 0xFF0000,
            2 => 0x00FF00,
            3 => 0x0000FF,
            _ => panic!(""),
        }
    }

    // helper function to draw a tile into main buffer
    fn draw_tile(&mut self, x: usize, y: usize, tile_num: usize) {
        let base_addr = (tile_num | ((self.ppu_ctrl as usize & 0x10) << 4)) * 16;

        for row in 0..8 {
            let plane0 = self.read((base_addr + row) as u16);
            let plane1 = self.read((base_addr + row + 8) as u16);

            for col in 0..8 {
                let bit0 = (plane0 >> (7 - col)) & 1;
                let bit1 = (plane1 >> (7 - col)) & 1;
                let color_index = (bit1 << 1) | bit0;

                let pixel_x = x * 8 + col;
                let pixel_y = y * 8 + row;
                let pixel_addr = pixel_y * NES_WIDTH + pixel_x;

                self.frame_buffer
                    .write(pixel_addr, self.get_color(color_index as u32));
            }
        }
    }

    // debugging function to output a frame buffer with pattern_table
    // does not need to run every frame
    // TODO: make more efficient, dont need to update every frame
    pub fn pattern_table(&self) -> FrameBuffer {
        if !self
            .debug_ctx
            .borrow()
            .flag_enabled(&super::debug::DebugFlag::Ppu)
        {
            return FrameBuffer::new(0, 0);
        }

        let mut buf = FrameBuffer::new(PATTERN_TABLE_WIDTH, PATTERN_TABLE_HEIGHT);

        for addr in 0..0x2000 {
            let fine_y = addr & 7;
            let plane_sig = (addr & 8) >> 3;
            let tile_num = (addr & 0x1FF0) >> 4;
            let large_y = tile_num / 16;

            let bit_plane = self.read(addr as u16);

            for bit_num in 0..8 {
                let mask = 0x80 >> bit_num;
                let bit = u32::from((bit_plane & mask) > 0);
                let pixel_addr = (PATTERN_TABLE_WIDTH * fine_y)
                    + (8 * tile_num)
                    + bit_num
                    + (large_y * PATTERN_TABLE_WIDTH * 7);

                if plane_sig == 0 {
                    buf.write(pixel_addr, bit);
                } else {
                    let lsb = buf.read(pixel_addr);
                    let color_offset = (bit << 1) | lsb;
                    buf.write(pixel_addr, self.get_color(color_offset));
                }
            }
        }

        buf
    }

    // draw nametable to main screen buffer
    pub fn draw_nametable(&mut self) {
        let mut x = 0;
        let mut y = 0;

        for addr in 0..0x3c0 {
            let tile = self.vram[addr];
            self.draw_tile(x, y, tile as usize);

            x += 1;
            if x >= 32 {
                x = 0;
                y += 1;
            }

            if y >= 30 {
                y = 0;
            }
        }
    }

    // frame getter
    pub fn frame(&self) -> &FrameBuffer {
        &self.frame_buffer
    }
}
