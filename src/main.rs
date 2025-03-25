mod bus;
mod cartridge;
mod cpu;
mod emulator;

use cartridge::Cartridge;
use emulator::Emulator;

fn main() {
    let mut emulator = Emulator::new(true);

    let cartridge = Cartridge::new("./roms/nestest.nes".to_string());
    emulator.load_cartridge(cartridge);

    emulator.run();
}

#[cfg(test)]
mod test {
    #[test]
    fn cool_test() {
        assert!(true);
    }
}
