mod bus;
mod cartridge;
mod cpu;
mod emulator;

use cartridge::Cartridge;
use emulator::Emulator;
use simple_graphics::display::Display;

const WINDOW_TITLE: &str = "Nest";
const WINDOW_WIDTH: usize = 256;
const WINDOW_HEIGHT: usize = 240;

fn main() {
    let mut display = Display::new(WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT, true)
        .expect("Failed to create window");

    let mut emulator = Emulator::new(true);

    let cartridge = Cartridge::new("./roms/nestest.nes".to_string());
    emulator.load_cartridge(cartridge);

    emulator.run();
}