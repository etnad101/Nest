mod bus;
mod cpu;
mod emulator;
mod cartridge;

use emulator::Emulator;
use cartridge::Cartridge;


fn main() {
    let mut emulator = Emulator::new(true);

    let cartridge = Cartridge::new("./roms/nestest.nes".to_string());
    emulator.load_cartridge(cartridge);

    emulator.run();
}
