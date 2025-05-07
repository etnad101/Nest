mod bus;
mod cartridge;
mod cpu;
mod ppu;
mod emulator;

use cartridge::Cartridge;
use emulator::{DebugMode, Emulator};

fn main() {

    let mut emulator = Emulator::new();


    emulator.set_debug_mode(vec![DebugMode::CPU]);

    let cartridge = Cartridge::new("./roms/DonkeyKong.nes".to_string());
    emulator.load_cartridge(cartridge);

    emulator.run();
}