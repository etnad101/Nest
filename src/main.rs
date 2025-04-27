mod bus;
mod cartridge;
mod cpu;
mod ppu;
mod emulator;

use cartridge::Cartridge;
use emulator::{DebugMode, Emulator};
use simple_graphics::display::Display;

const WINDOW_TITLE: &str = "Nest";
const WINDOW_WIDTH: usize = 256;
const WINDOW_HEIGHT: usize = 240;

fn main() {
    let mut display = Display::new(WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT, true)
        .expect("Failed to create window");
    display.limit_frame_rate(Some(std::time::Duration::from_micros(16_667)));

    let mut emulator = Emulator::new(&mut display);

    emulator.set_debug_mode(vec![DebugMode::PPU]);

    let cartridge = Cartridge::new("./roms/DonkeyKong.nes".to_string());
    emulator.load_cartridge(cartridge);

    emulator.run();
}