mod emulator;
mod gui;
mod testing;

use emulator::cartridge::Cartridge;
use emulator::{DebugFlag, Emulator};
use gui::NestApp;

const WINDOW_TITLE: &str = "Nest";
const RUN_JSON_TEST: bool = false;

fn main() {
    let mut emulator = Emulator::new();

    if RUN_JSON_TEST {
        testing::run_json_tests(&mut emulator);
        return;
    }

    emulator.set_debug_flags(vec![DebugFlag::Ppu]);

    let cartridge = Cartridge::new("./roms/DonkeyKong.nes".to_string()).unwrap();
    emulator.load_cartridge(cartridge);
    emulator.reset();

    let options = eframe::NativeOptions {
        ..Default::default()
    };

    eframe::run_native(
        WINDOW_TITLE,
        options,
        Box::new(|_cc| Ok(Box::new(NestApp::new(emulator)))),
    )
    .unwrap();
}