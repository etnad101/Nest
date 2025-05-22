mod emulator;
mod gui;
mod testing;

use emulator::cartridge::Cartridge;
use emulator::{DebugFlag, Emulator};
use gui::NestApp;

const WINDOW_TITLE: &str = "Nest";
const RUN_JSON_TEST: bool = false;

fn main() {
    // create new emulator instance
    let mut emulator = Emulator::new();

    // run cpu json tests if specified
    if RUN_JSON_TEST {
        testing::run_json_tests(&mut emulator);
        return;
    }

    // enable debugging modes
    emulator.set_debug_flags(vec![DebugFlag::Ppu, DebugFlag::Cpu]);

    // load cartridge from file
    let cartridge = Cartridge::new("./roms/DonkeyKong.nes".to_string()).unwrap();
    emulator.load_cartridge(cartridge);

    // make sure to reset emulator to begin executing with proper state
    emulator.reset();

    // setup gui
    let options = eframe::NativeOptions {
        ..Default::default()
    };

    // run emulator in egui loop
    eframe::run_native(
        WINDOW_TITLE,
        options,
        Box::new(|_cc| Ok(Box::new(NestApp::new(emulator)))),
    )
    .unwrap();
}
