mod emulator;

use emulator::cartridge::Cartridge;
use emulator::{DebugMode, Emulator};
use simple_graphics::display::Display;

const WINDOW_TITLE: &str = "Nest";

fn main() {
    let mut main_display = Display::new(
        WINDOW_TITLE,
        emulator::NES_WIDTH,
        emulator::NES_HEIGHT,
        true,
    )
    .expect("Failed to create display");

    let mut pattern_table_display = Display::new(
        "Pattern Tables",
        emulator::PATTERN_TABLE_WIDTH,
        emulator::PATTERN_TABLE_HEIGHT,
        true,
    )
    .expect("Failed to create display");

    let mut emulator = Emulator::new();

    emulator.set_debug_mode(vec![DebugMode::Ppu]);

    let cartridge = Cartridge::new("./roms/DonkeyKong.nes".to_string());
    emulator.load_cartridge(cartridge);

    emulator.run_with_callback(&mut |(frame, pattern_table)| {
        main_display.set_buffer(frame.to_vec());
        main_display.render().unwrap();
        pattern_table_display.set_buffer(pattern_table);
        pattern_table_display.render().unwrap();
    });

    // get input
    // tick components
    // @ end of frame copy frame buffer
    // update screen
}
