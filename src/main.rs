mod bus;
mod cartridge;
mod cpu;
mod emulator;

use cartridge::Cartridge;
use emulator::{DebugMode, Emulator};

const WINDOW_TITLE: &str = "Nest";
const WINDOW_WIDTH: usize = 256;
const WINDOW_HEIGHT: usize = 240;

fn main() {
    let mut emulator = Emulator::new();

    emulator.set_debug_mode(vec![DebugMode::CPU, DebugMode::STEP]);

    let cartridge = Cartridge::new("./roms/DonkeyKong.nes".to_string());
    emulator.load_cartridge(cartridge);
    emulator.reset();
}