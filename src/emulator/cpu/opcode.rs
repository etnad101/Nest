use std::collections::HashMap;

use super::AddressingMode;

// all official opcode names
#[derive(Clone, Copy)]
pub enum OpcodeName {
    Lda,
    Ldx,
    Ldy,
    Sta,
    Stx,
    Sty,
    Tax,
    Tay,
    Txa,
    Tya,
    Adc,
    Sbc,
    Inc,
    Dec,
    Inx,
    Dex,
    Iny,
    Dey,
    Asl,
    Lsr,
    Rol,
    Ror,
    And,
    Ora,
    Eor,
    Bit,
    Cmp,
    Cpx,
    Cpy,
    Bcc,
    Bcs,
    Beq,
    Bne,
    Bpl,
    Bmi,
    Bvc,
    Bvs,
    Jmp,
    Jsr,
    Rts,
    Brk,
    Rti,
    Pha,
    Pla,
    Php,
    Plp,
    Txs,
    Tsx,
    Clc,
    Sec,
    Cli,
    Sei,
    Cld,
    Sed,
    Clv,
    Nop,
}

impl std::fmt::Display for OpcodeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            OpcodeName::Lda => "LDA",
            OpcodeName::Ldx => "LDX",
            OpcodeName::Ldy => "LDY",
            OpcodeName::Sta => "STA",
            OpcodeName::Stx => "STX",
            OpcodeName::Sty => "STY",
            OpcodeName::Tax => "TAX",
            OpcodeName::Tay => "TAY",
            OpcodeName::Txa => "TXA",
            OpcodeName::Tya => "TYA",
            OpcodeName::Adc => "ADC",
            OpcodeName::Sbc => "SBC",
            OpcodeName::Inc => "INC",
            OpcodeName::Dec => "DEC",
            OpcodeName::Inx => "INX",
            OpcodeName::Dex => "DEX",
            OpcodeName::Iny => "INY",
            OpcodeName::Dey => "DEY",
            OpcodeName::Asl => "ASL",
            OpcodeName::Lsr => "LSR",
            OpcodeName::Rol => "ROL",
            OpcodeName::Ror => "ROR",
            OpcodeName::And => "AND",
            OpcodeName::Ora => "ORA",
            OpcodeName::Eor => "EOR",
            OpcodeName::Bit => "BIT",
            OpcodeName::Cmp => "CMP",
            OpcodeName::Cpx => "CPX",
            OpcodeName::Cpy => "CPY",
            OpcodeName::Bcc => "BCC",
            OpcodeName::Bcs => "BCS",
            OpcodeName::Beq => "BEQ",
            OpcodeName::Bne => "BNE",
            OpcodeName::Bpl => "BPL",
            OpcodeName::Bmi => "BMI",
            OpcodeName::Bvc => "BVC",
            OpcodeName::Bvs => "BVS",
            OpcodeName::Jmp => "JMP",
            OpcodeName::Jsr => "JSR",
            OpcodeName::Rts => "RTS",
            OpcodeName::Brk => "BRK",
            OpcodeName::Rti => "RTI",
            OpcodeName::Pha => "PHA",
            OpcodeName::Pla => "PLA",
            OpcodeName::Php => "PHP",
            OpcodeName::Plp => "PLP",
            OpcodeName::Txs => "TXS",
            OpcodeName::Tsx => "TSX",
            OpcodeName::Clc => "CLC",
            OpcodeName::Sec => "SEC",
            OpcodeName::Cli => "CLI",
            OpcodeName::Sei => "SEI",
            OpcodeName::Cld => "CLD",
            OpcodeName::Sed => "SED",
            OpcodeName::Clv => "CLV",
            OpcodeName::Nop => "NOP",
        };
        f.write_str(val)
    }
}

#[derive(Clone, Copy)]
pub struct Opcode {
    code: u8,
    name: OpcodeName,
    bytes: u16,
    cycles: usize,
    mode: AddressingMode,
}

impl Opcode {
    pub fn new(
        code: u8,
        name: OpcodeName,
        bytes: u16,
        cycles: usize,
        mode: AddressingMode,
    ) -> Self {
        Self {
            code,
            name,
            bytes,
            cycles,
            mode,
        }
    }

    // creates a hash map with all the official opcodes
    pub fn get_opcode_map() -> HashMap<u8, Opcode> {
        let opcodes: Vec<Opcode> = vec![
            Opcode::new(0xA9, OpcodeName::Lda, 2, 2, AddressingMode::Immediate),
            Opcode::new(0xA5, OpcodeName::Lda, 2, 3, AddressingMode::ZeroPage),
            Opcode::new(0xB5, OpcodeName::Lda, 2, 4, AddressingMode::ZeroPageX),
            Opcode::new(0xAD, OpcodeName::Lda, 3, 4, AddressingMode::Absolute),
            Opcode::new(0xBD, OpcodeName::Lda, 3, 4, AddressingMode::AbsoluteX), // +1 cycle if page crossed
            Opcode::new(0xB9, OpcodeName::Lda, 3, 4, AddressingMode::AbsoluteY), // +1 cycle if page crossed
            Opcode::new(0xA1, OpcodeName::Lda, 2, 6, AddressingMode::IndirectX),
            Opcode::new(0xB1, OpcodeName::Lda, 2, 5, AddressingMode::IndirectY), // +1 cycle if page crossed
            Opcode::new(0xA2, OpcodeName::Ldx, 2, 2, AddressingMode::Immediate),
            Opcode::new(0xA6, OpcodeName::Ldx, 2, 3, AddressingMode::ZeroPage),
            Opcode::new(0xB6, OpcodeName::Ldx, 2, 4, AddressingMode::ZeroPageY),
            Opcode::new(0xAE, OpcodeName::Ldx, 3, 4, AddressingMode::Absolute),
            Opcode::new(0xBE, OpcodeName::Ldx, 3, 4, AddressingMode::AbsoluteY), // + 1 cycle if page crossed
            Opcode::new(0xA0, OpcodeName::Ldy, 2, 2, AddressingMode::Immediate),
            Opcode::new(0xA4, OpcodeName::Ldy, 2, 3, AddressingMode::ZeroPage),
            Opcode::new(0xB4, OpcodeName::Ldy, 2, 4, AddressingMode::ZeroPageX),
            Opcode::new(0xAC, OpcodeName::Ldy, 3, 4, AddressingMode::Absolute),
            Opcode::new(0xBC, OpcodeName::Ldy, 3, 4, AddressingMode::AbsoluteX), // + 1 cycle if page crossed
            Opcode::new(0x85, OpcodeName::Sta, 2, 3, AddressingMode::ZeroPage),
            Opcode::new(0x95, OpcodeName::Sta, 2, 4, AddressingMode::ZeroPageX),
            Opcode::new(0x8D, OpcodeName::Sta, 3, 4, AddressingMode::Absolute),
            Opcode::new(0x9D, OpcodeName::Sta, 3, 5, AddressingMode::AbsoluteX),
            Opcode::new(0x99, OpcodeName::Sta, 3, 5, AddressingMode::AbsoluteY),
            Opcode::new(0x81, OpcodeName::Sta, 2, 6, AddressingMode::IndirectX),
            Opcode::new(0x91, OpcodeName::Sta, 2, 6, AddressingMode::IndirectY),
            Opcode::new(0x86, OpcodeName::Stx, 2, 3, AddressingMode::ZeroPage),
            Opcode::new(0x96, OpcodeName::Stx, 2, 4, AddressingMode::ZeroPageY),
            Opcode::new(0x8E, OpcodeName::Stx, 3, 4, AddressingMode::Absolute),
            Opcode::new(0x84, OpcodeName::Sty, 2, 3, AddressingMode::ZeroPage),
            Opcode::new(0x94, OpcodeName::Sty, 2, 4, AddressingMode::ZeroPageX),
            Opcode::new(0x8C, OpcodeName::Sty, 3, 4, AddressingMode::Absolute),
            // Transfer
            Opcode::new(0xAA, OpcodeName::Tax, 1, 2, AddressingMode::Implicit),
            Opcode::new(0xA8, OpcodeName::Tay, 1, 2, AddressingMode::Implicit),
            Opcode::new(0x8A, OpcodeName::Txa, 1, 2, AddressingMode::Implicit),
            Opcode::new(0x98, OpcodeName::Tya, 1, 2, AddressingMode::Implicit),
            // Arithmetic
            Opcode::new(0x69, OpcodeName::Adc, 2, 2, AddressingMode::Immediate),
            Opcode::new(0x65, OpcodeName::Adc, 2, 3, AddressingMode::ZeroPage),
            Opcode::new(0x75, OpcodeName::Adc, 2, 4, AddressingMode::ZeroPageX),
            Opcode::new(0x6D, OpcodeName::Adc, 3, 4, AddressingMode::Absolute),
            Opcode::new(0x7D, OpcodeName::Adc, 3, 4, AddressingMode::AbsoluteX), // + 1 cycle if page crossed
            Opcode::new(0x79, OpcodeName::Adc, 3, 4, AddressingMode::AbsoluteY), // + 1 cycle if page crossed
            Opcode::new(0x61, OpcodeName::Adc, 2, 6, AddressingMode::IndirectX),
            Opcode::new(0x71, OpcodeName::Adc, 2, 5, AddressingMode::IndirectY), // + 1 cycle if page crossed
            Opcode::new(0xE9, OpcodeName::Sbc, 2, 2, AddressingMode::Immediate),
            Opcode::new(0xE5, OpcodeName::Sbc, 2, 3, AddressingMode::ZeroPage),
            Opcode::new(0xF5, OpcodeName::Sbc, 2, 4, AddressingMode::ZeroPageX),
            Opcode::new(0xED, OpcodeName::Sbc, 3, 4, AddressingMode::Absolute),
            Opcode::new(0xFD, OpcodeName::Sbc, 3, 4, AddressingMode::AbsoluteX), // + 1 cycle if page crossed
            Opcode::new(0xF9, OpcodeName::Sbc, 3, 4, AddressingMode::AbsoluteY), // + 1 cycle if page crossed
            Opcode::new(0xE1, OpcodeName::Sbc, 2, 6, AddressingMode::IndirectX),
            Opcode::new(0xF1, OpcodeName::Sbc, 2, 5, AddressingMode::IndirectY), // + 1 cycle if page crossed
            Opcode::new(0xE6, OpcodeName::Inc, 2, 5, AddressingMode::ZeroPage),
            Opcode::new(0xF6, OpcodeName::Inc, 2, 6, AddressingMode::ZeroPageX),
            Opcode::new(0xEE, OpcodeName::Inc, 3, 6, AddressingMode::Absolute),
            Opcode::new(0xFE, OpcodeName::Inc, 3, 7, AddressingMode::AbsoluteX),
            Opcode::new(0xC6, OpcodeName::Dec, 2, 5, AddressingMode::ZeroPage),
            Opcode::new(0xD6, OpcodeName::Dec, 2, 6, AddressingMode::ZeroPageX),
            Opcode::new(0xCE, OpcodeName::Dec, 3, 6, AddressingMode::Absolute),
            Opcode::new(0xDE, OpcodeName::Dec, 3, 7, AddressingMode::AbsoluteX),
            Opcode::new(0xE8, OpcodeName::Inx, 1, 2, AddressingMode::Implicit),
            Opcode::new(0xCA, OpcodeName::Dex, 1, 2, AddressingMode::Implicit),
            Opcode::new(0xC8, OpcodeName::Iny, 1, 2, AddressingMode::Implicit),
            Opcode::new(0x88, OpcodeName::Dey, 1, 2, AddressingMode::Implicit),
            // Shift
            Opcode::new(0x0A, OpcodeName::Asl, 1, 2, AddressingMode::Accumulator),
            Opcode::new(0x06, OpcodeName::Asl, 2, 5, AddressingMode::ZeroPage),
            Opcode::new(0x16, OpcodeName::Asl, 2, 6, AddressingMode::ZeroPageX),
            Opcode::new(0x0E, OpcodeName::Asl, 3, 6, AddressingMode::Absolute),
            Opcode::new(0x1E, OpcodeName::Asl, 3, 7, AddressingMode::AbsoluteX),
            Opcode::new(0x4A, OpcodeName::Lsr, 1, 2, AddressingMode::Accumulator),
            Opcode::new(0x46, OpcodeName::Lsr, 2, 5, AddressingMode::ZeroPage),
            Opcode::new(0x56, OpcodeName::Lsr, 2, 6, AddressingMode::ZeroPageX),
            Opcode::new(0x4E, OpcodeName::Lsr, 3, 6, AddressingMode::Absolute),
            Opcode::new(0x5E, OpcodeName::Lsr, 3, 7, AddressingMode::AbsoluteX),
            Opcode::new(0x2A, OpcodeName::Rol, 1, 2, AddressingMode::Accumulator),
            Opcode::new(0x26, OpcodeName::Rol, 2, 5, AddressingMode::ZeroPage),
            Opcode::new(0x36, OpcodeName::Rol, 2, 6, AddressingMode::ZeroPageX),
            Opcode::new(0x2E, OpcodeName::Rol, 3, 6, AddressingMode::Absolute),
            Opcode::new(0x3E, OpcodeName::Rol, 3, 7, AddressingMode::AbsoluteX),
            Opcode::new(0x6A, OpcodeName::Ror, 1, 2, AddressingMode::Accumulator),
            Opcode::new(0x66, OpcodeName::Ror, 2, 5, AddressingMode::ZeroPage),
            Opcode::new(0x76, OpcodeName::Ror, 2, 6, AddressingMode::ZeroPageX),
            Opcode::new(0x6E, OpcodeName::Ror, 3, 6, AddressingMode::Absolute),
            Opcode::new(0x7E, OpcodeName::Ror, 3, 7, AddressingMode::AbsoluteX),
            // Bitwise
            Opcode::new(0x29, OpcodeName::And, 2, 2, AddressingMode::Immediate),
            Opcode::new(0x25, OpcodeName::And, 2, 3, AddressingMode::ZeroPage),
            Opcode::new(0x35, OpcodeName::And, 2, 4, AddressingMode::ZeroPageX),
            Opcode::new(0x2D, OpcodeName::And, 3, 4, AddressingMode::Absolute),
            Opcode::new(0x3D, OpcodeName::And, 3, 4, AddressingMode::AbsoluteX), // + 1 cycle if page crossed
            Opcode::new(0x39, OpcodeName::And, 3, 4, AddressingMode::AbsoluteY), // + 1 cycle if page crossed
            Opcode::new(0x21, OpcodeName::And, 2, 6, AddressingMode::IndirectX),
            Opcode::new(0x31, OpcodeName::And, 2, 5, AddressingMode::IndirectY), // + 1 cycle if page crossed
            Opcode::new(0x09, OpcodeName::Ora, 2, 2, AddressingMode::Immediate),
            Opcode::new(0x05, OpcodeName::Ora, 2, 3, AddressingMode::ZeroPage),
            Opcode::new(0x15, OpcodeName::Ora, 2, 4, AddressingMode::ZeroPageX),
            Opcode::new(0x0D, OpcodeName::Ora, 3, 4, AddressingMode::Absolute),
            Opcode::new(0x1D, OpcodeName::Ora, 3, 4, AddressingMode::AbsoluteX), // + 1 cycle if page crossed
            Opcode::new(0x19, OpcodeName::Ora, 3, 4, AddressingMode::AbsoluteY), // + 1 cycle if page crossed
            Opcode::new(0x01, OpcodeName::Ora, 2, 6, AddressingMode::IndirectX),
            Opcode::new(0x11, OpcodeName::Ora, 2, 5, AddressingMode::IndirectY), // + 1 cycle if page crossed
            Opcode::new(0x49, OpcodeName::Eor, 2, 2, AddressingMode::Immediate),
            Opcode::new(0x45, OpcodeName::Eor, 2, 3, AddressingMode::ZeroPage),
            Opcode::new(0x55, OpcodeName::Eor, 2, 4, AddressingMode::ZeroPageX),
            Opcode::new(0x4D, OpcodeName::Eor, 3, 4, AddressingMode::Absolute),
            Opcode::new(0x5D, OpcodeName::Eor, 3, 4, AddressingMode::AbsoluteX), // + 1 cycle if page crossed
            Opcode::new(0x59, OpcodeName::Eor, 3, 4, AddressingMode::AbsoluteY), // + 1 cycle if page crossed
            Opcode::new(0x41, OpcodeName::Eor, 2, 6, AddressingMode::IndirectX),
            Opcode::new(0x51, OpcodeName::Eor, 2, 5, AddressingMode::IndirectY), // + 1 cycle if page crossed
            Opcode::new(0x24, OpcodeName::Bit, 2, 3, AddressingMode::ZeroPage),
            Opcode::new(0x2C, OpcodeName::Bit, 3, 4, AddressingMode::Absolute),
            // Compare
            Opcode::new(0xC9, OpcodeName::Cmp, 2, 2, AddressingMode::Immediate),
            Opcode::new(0xC5, OpcodeName::Cmp, 2, 3, AddressingMode::ZeroPage),
            Opcode::new(0xD5, OpcodeName::Cmp, 2, 4, AddressingMode::ZeroPageX),
            Opcode::new(0xCD, OpcodeName::Cmp, 3, 4, AddressingMode::Absolute),
            Opcode::new(0xDD, OpcodeName::Cmp, 3, 4, AddressingMode::AbsoluteX), // + 1 cycle if page crossed
            Opcode::new(0xD9, OpcodeName::Cmp, 3, 4, AddressingMode::AbsoluteY),
            Opcode::new(0xC1, OpcodeName::Cmp, 2, 6, AddressingMode::IndirectX), // + 1 cycle if page crossed
            Opcode::new(0xD1, OpcodeName::Cmp, 2, 5, AddressingMode::IndirectY), // + 1 cycle if page crossed
            Opcode::new(0xE0, OpcodeName::Cpx, 2, 2, AddressingMode::Immediate),
            Opcode::new(0xE4, OpcodeName::Cpx, 2, 3, AddressingMode::ZeroPage),
            Opcode::new(0xEC, OpcodeName::Cpx, 3, 4, AddressingMode::Absolute),
            Opcode::new(0xC0, OpcodeName::Cpy, 2, 2, AddressingMode::Immediate),
            Opcode::new(0xC4, OpcodeName::Cpy, 2, 3, AddressingMode::ZeroPage),
            Opcode::new(0xCC, OpcodeName::Cpy, 3, 4, AddressingMode::Absolute),
            // Branch
            Opcode::new(0x90, OpcodeName::Bcc, 2, 2, AddressingMode::Relative), // + 1 cycle if branch taken, + 1 if page crossed
            Opcode::new(0xB0, OpcodeName::Bcs, 2, 2, AddressingMode::Relative), // + 1 cycle if branch taken, + 1 if page crossed
            Opcode::new(0xF0, OpcodeName::Beq, 2, 2, AddressingMode::Relative), // + 1 cycle if branch taken, + 1 if page crossed
            Opcode::new(0xD0, OpcodeName::Bne, 2, 2, AddressingMode::Relative), // + 1 cycle if branch taken, + 1 if page crossed
            Opcode::new(0x10, OpcodeName::Bpl, 2, 2, AddressingMode::Relative), // + 1 cycle if branch taken, + 1 if page crossed
            Opcode::new(0x30, OpcodeName::Bmi, 2, 2, AddressingMode::Relative), // + 1 cycle if branch taken, + 1 if page crossed
            Opcode::new(0x50, OpcodeName::Bvc, 2, 2, AddressingMode::Relative), // + 1 cycle if branch taken, + 1 if page crossed
            Opcode::new(0x70, OpcodeName::Bvs, 2, 2, AddressingMode::Relative), // + 1 cycle if branch taken, + 1 if page crossed
            // Jump
            Opcode::new(0x4C, OpcodeName::Jmp, 3, 3, AddressingMode::Absolute),
            Opcode::new(0x6C, OpcodeName::Jmp, 3, 5, AddressingMode::Indirect),
            Opcode::new(0x20, OpcodeName::Jsr, 3, 6, AddressingMode::Absolute),
            Opcode::new(0x60, OpcodeName::Rts, 1, 6, AddressingMode::Implicit),
            Opcode::new(0x00, OpcodeName::Brk, 1, 7, AddressingMode::Implicit), // Only 1 byte wide, but skips the second byte so can be considered 2 bytes
            Opcode::new(0x40, OpcodeName::Rti, 1, 6, AddressingMode::Implicit),
            // tackS
            Opcode::new(0x48, OpcodeName::Pha, 1, 3, AddressingMode::Implicit),
            Opcode::new(0x68, OpcodeName::Pla, 1, 4, AddressingMode::Implicit),
            Opcode::new(0x08, OpcodeName::Php, 1, 3, AddressingMode::Implicit),
            Opcode::new(0x28, OpcodeName::Plp, 1, 4, AddressingMode::Implicit),
            Opcode::new(0x9A, OpcodeName::Txs, 1, 2, AddressingMode::Implicit),
            Opcode::new(0xBA, OpcodeName::Tsx, 1, 2, AddressingMode::Implicit),
            // Flags
            Opcode::new(0x18, OpcodeName::Clc, 1, 2, AddressingMode::Implicit),
            Opcode::new(0x38, OpcodeName::Sec, 1, 2, AddressingMode::Implicit),
            Opcode::new(0x58, OpcodeName::Cli, 1, 2, AddressingMode::Implicit),
            Opcode::new(0x78, OpcodeName::Sei, 1, 2, AddressingMode::Implicit),
            Opcode::new(0xD8, OpcodeName::Cld, 1, 2, AddressingMode::Implicit),
            Opcode::new(0xF8, OpcodeName::Sed, 1, 2, AddressingMode::Implicit),
            Opcode::new(0xB8, OpcodeName::Clv, 1, 2, AddressingMode::Implicit),
            // Other
            Opcode::new(0xEA, OpcodeName::Nop, 1, 2, AddressingMode::Implicit),
        ];

        let mut map = HashMap::new();
        for op in opcodes {
            map.insert(op.code, op);
        }

        map
    }

    pub fn name(&self) -> OpcodeName {
        self.name
    }

    pub fn mode(&self) -> AddressingMode {
        self.mode
    }

    pub fn cycles(&self) -> usize {
        self.cycles
    }

    pub fn bytes(&self) -> u16 {
        self.bytes
    }
}
