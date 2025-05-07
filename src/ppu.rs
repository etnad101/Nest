use std::{cell::RefCell, rc::Rc};

use simple_graphics::display::{Color, Display, BLACK, WHITE};

use crate::{bus::Bus, emulator::{WINDOW_HEIGHT, WINDOW_WIDTH}};

const WINDOW_BUFF_SIZE: usize = crate::emulator::WINDOW_HEIGHT * crate::emulator::WINDOW_WIDTH;
const TEST_PALETTE: [Color; 4] = [0x000000, 0x565656, 0x9b9b9b, 0xffffff];
const ONE_TILE: [u8; 16] = [0x18, 0x38, 0x18, 0x18, 0x18, 0x18, 0x7E, 00, 00, 00, 00, 00, 00, 00, 00, 00];
const PT_BUFFER_SIZE: usize = 128 * 256;

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
        match *self {
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

pub struct Ppu {
    debug: bool,
    bus: Rc<RefCell<Bus>>,

    pt_display: Option<Display>, 
    main_frame_buffer: Box<[Color; WINDOW_BUFF_SIZE]>,
    pt_buffer: Box<[Color; PT_BUFFER_SIZE]>
}

impl Ppu {
    pub fn new(bus: Rc<RefCell<Bus>>) -> Self {
        Self {
            debug: false,
            bus,
            main_frame_buffer: Box::new([0; WINDOW_BUFF_SIZE]),
            pt_display: None,
            pt_buffer: Box::new([0; PT_BUFFER_SIZE]),
        }    
    }

    pub fn set_debug_mode(&mut self, debug: bool) {
        self.debug = debug;

        if self.debug {
            let mut d = Display::new("Pattern Tables", 128, 256, true)
                .expect("Failed to create pattern table window");
            d.limit_frame_rate(None);
            self.pt_display = Some(d);
        } else {
            self.pt_display = None;
        }
    }

    fn draw_pattern_tables(&mut self) {
        for addr in 0..0x2000 {
            let plane_data = self.bus.borrow().ppu_read(addr as u16);

            /*
            DCBA98 76543210
            ---------------
            0HNNNN NNNNPyyy
            |||||| |||||+++- T: Fine Y offset, the row number within a tile
            |||||| ||||+---- P: Bit plane (0: less significant bit; 1: more significant bit)
            ||++++-++++----- N: Tile number from name table
            |+-------------- H: Half of pattern table (0: "left"; 1: "right")
            +--------------- 0: Pattern table is at $0000-$1FFF 
            */

            // Used to calculate pixel position
            let fine_y_offset = addr & 0x7;
            let bit_plane = (addr & 0x8) >> 3;
            let tile_number = (addr & 0xFF0) >> 4;
            let row_number = tile_number / 16;

            // println!("{} -> fine_y: {}, plane: {}, tile_number: {}, row: {}", addr, fine_y_offset, bit_plane, tile_number, row_number);
            for bit in 0..8 {
                // Get position on screen for current pixel
                let pixel_pos = (row_number * 16 * 128) + (tile_number % 16) * 8 + fine_y_offset * 128 + bit;
                let pixel_data = if plane_data & (0x80 >> bit) > 0 {1} else {0};

                if bit_plane == 0 {
                    /*
                    If the current data is from the less significant plane,
                    temporarily store the bit in the screen buffer to be
                    fetched later
                    */
                    self.pt_buffer[pixel_pos] = pixel_data.into();
                } else {
                    /*
                    If current data is from more significant plane,
                    grab previously stored bit, and store the proper
                    pixel color in the location
                    */
                    let lsb = self.pt_buffer[pixel_pos];
                    let palette_index = ((pixel_data << 1) | lsb) as usize;
                    let color = TEST_PALETTE[palette_index];
                    self.pt_buffer[pixel_pos] = color;
                }
            }

            let pt_display = self.pt_display.as_mut().unwrap();
            pt_display.set_buffer(self.pt_buffer.to_vec());
            pt_display.render().unwrap();

            if !pt_display.is_open() {
                // TODO - This might be a problem because it dosent update the emulator debug vec
                self.debug = false;
            }
        }

    }

    pub fn handle_debug(&mut self) {
        if !self.debug {
            return;
        }
        self.draw_pattern_tables();
    }


    pub fn get_frame(&mut self) -> Vec<Color> {
        self.main_frame_buffer.to_vec()
    }

}