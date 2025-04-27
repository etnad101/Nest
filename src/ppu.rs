use std::{cell::RefCell, rc::Rc};

use simple_graphics::display::{Color, BLACK, WHITE};

use crate::{bus::Bus, WINDOW_WIDTH};

const WINDOW_BUFF_SIZE: usize = crate::WINDOW_HEIGHT * crate::WINDOW_WIDTH;
const TEST_PALETTE: [Color; 4] = [0x000000, 0x565656, 0x9b9b9b, 0xffffff];
const ONE_TILE: [u8; 16] = [0x18, 0x38, 0x18, 0x18, 0x18, 0x18, 0x7E, 00, 00, 00, 00, 00, 00, 00, 00, 00];

pub struct Ppu {
    debug: bool,
    bus: Rc<RefCell<Bus>>,
    frame_buffer: [Color; WINDOW_BUFF_SIZE],
}

impl Ppu {
    pub fn new(bus: Rc<RefCell<Bus>>) -> Self {
        Self {
            debug: false,
            bus,
            frame_buffer: [0; WINDOW_BUFF_SIZE],
        }    
    }

    pub fn set_debug_mode(&mut self, debug: bool) {
        self.debug = debug;
    }

    pub fn get_frame(&mut self) -> Vec<Color> {
        let mut buffer_x = 0;

        for i in 0..8 {
            let p1 = self.bus.borrow().ppu_read(i);
            let p2 = self.bus.borrow().ppu_read(i + 8);

            let fine_y_offset = (i & 7) as usize; // last 3 bits

            for bit in 0..8 {
                let mask = 0x80 >> bit;
                let p1: usize = if (p1 & mask) > 0 {1} else {0};
                let p2: usize = if (p2 & mask) > 0 {1} else {0};
                let palette_index = (p2 << 1) | p1;

                let buff_ptr = (fine_y_offset * WINDOW_WIDTH) + buffer_x; 
                self.frame_buffer[buff_ptr] = TEST_PALETTE[palette_index];
                buffer_x += 1;
            }
        }
        self.frame_buffer.to_vec()
    }

}