pub struct Apu {
    //TODO: add registers
}

impl Apu {
    pub fn new() -> Self {
        Self {}
    }

    pub fn read_reg(&self, addr: u16) -> u8 {
        unimplemented!("APU read reg")
    }

    pub fn write_reg(&self, addr: u16, value: u8) {
        unimplemented!("APU write reg")
    }
}
