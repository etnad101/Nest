use crate::bus::Bus;

pub enum AddressingMode {

}

pub struct Cpu<'a> {
    debug: bool,

    bus: &'a Bus,
    cycles: usize,
    page_crossed: bool,

    // registers
    r_a: u8,
    r_x: u8,
    r_y: u8,
    r_s: u8,
    r_pc: u16,

    // flags
    f_c: bool,
    f_z: bool,
    f_i: bool,
    f_d: bool,
    f_v: bool,
    f_n: bool,

    pending_iflag_value: bool,
    pending_iflag_update: bool,
}

impl<'a> Cpu<'a> {
    pub fn new(bus: &'a Bus) -> Self {
        Self {
            debug: false,
            bus,
            cycles: 0,
            page_crossed: false,
            r_a: 0,
            r_x: 0,
            r_y: 0,
            r_s: 0xFD,
            r_pc: 0xC000,

            // flags
            f_c: false,
            f_z: false,
            f_i: true,
            f_d: false,
            f_v: false,
            f_n: false,

            pending_iflag_value: false,
            pending_iflag_update: false,

        }
    }

    fn getAddress(&self, mode: AddressingMode) -> u16 {
        todo!("getAddress");
    }
    fn getP(&self, f_B: bool) -> u8 {
        todo!("getP");
    }
    fn setP(&self, flags: u8, updateInow: bool) {
        todo!("setP");
    }
    fn updateZNFlags(&self, value: u8) {
        todo!("updateZNFlags");
    }
    fn compare(&self, reg: u8, mode: AddressingMode) {
        todo!("compare");
    }
    fn calculateBranchAddr(&self, offset: u8) -> u16 {
        todo!("calculateBranchAddr");
    }
    fn branch(&self, name: Name) {
        todo!("branch");
    }
    fn logInstr(&self, opcode: Opcode) {
        todo!("logInstr");
    }
    fn pushStack(&self, value: u8) {
        todo!("pushStack");
    }
    fn popStack(&self) -> u8 {
        todo!("popStack");
    }

    // instructions
    fn i_lda(&self, mode: AddressingMode) {
        todo!("i_lda");
    }
    fn i_sta(&self, mode: AddressingMode) {
        todo!("i_sta");
    }
    fn i_ldx(&self, mode: AddressingMode) {
        todo!("i_ldx");
    }
    fn i_stx(&self, mode: AddressingMode) {
        todo!("i_stx");
    }
    fn i_ldy(&self, mode: AddressingMode) {
        todo!("i_ldy");
    }
    fn i_sty(&self, mode: AddressingMode) {
        todo!("i_sty");
    }
    fn i_adc(&self, mode: AddressingMode) {
        todo!("i_adc");
    }
    fn i_sbc(&self, mode: AddressingMode) {
        todo!("i_sbc");
    }
    fn i_inc(&self, mode: AddressingMode) {
        todo!("i_inc");
    }
    fn i_dec(&self, mode: AddressingMode) {
        todo!("i_dec");
    }
    fn i_asl(&self, mode: AddressingMode) {
        todo!("i_asl");
    }
    fn i_lsr(&self, mode: AddressingMode) {
        todo!("i_lsr");
    }
    fn i_rol(&self, mode: AddressingMode) {
        todo!("i_rol");
    }
    fn i_ror(&self, mode: AddressingMode) {
        todo!("i_ror");
    }
    fn i_and(&self, mode: AddressingMode) {
        todo!("i_and");
    }
    fn i_ora(&self, mode: AddressingMode) {
        todo!("i_ora");
    }
    fn i_eor(&self, mode: AddressingMode) {
        todo!("i_eor");
    }
    fn i_bit(&self, mode: AddressingMode) {
        todo!("i_bit");
    }
    fn i_jsr(&self, mode: AddressingMode) {
        todo!("i_jsr");
    }
    fn i_rts(&self) {
        todo!("i_rts");
    }
    fn i_brk(&self) {
        todo!("i_brk");
    }
    fn i_rti(&self) {
        todo!("i_rti");
    }

    fn tick(&self) -> usize {
        todo!("tick");
    }
    fn pollIRQ(&self) {
        todo!("pollIRQ");
    }
    fn reset(&self) {
        todo!("reset");
    }
    fn getCycles(&self) -> usize {
        todo!("getCycles");
    }

}