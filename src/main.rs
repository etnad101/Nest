mod emulator;
mod gui;

use emulator::cartridge::Cartridge;
use emulator::{DebugMode, Emulator};
use gui::NestApp;

const WINDOW_TITLE: &str = "Nest";

fn main() {
    let mut emulator = Emulator::new();
    emulator.set_debug_mode(vec![DebugMode::Ppu]);

    let cartridge = Cartridge::new("./roms/DonkeyKong.nes".to_string()).unwrap();
    //emulator.load_cartridge(cartridge);
    //emulator.reset();

    let options = eframe::NativeOptions {
        ..Default::default()
    };

    eframe::run_native(
        WINDOW_TITLE,
        options,
        Box::new(|_cc| Ok(Box::new(NestApp::new(emulator)))),
    )
    .unwrap()
}
