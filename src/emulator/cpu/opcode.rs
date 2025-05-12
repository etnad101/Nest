use std::{collections::HashMap, hash::Hash};

use super::AddressingMode;

#[derive(Clone, Copy)]
pub(super) enum OpcodeName {
    LDA,
    LDX,
    LDY,
    STA,
    STX,
    STY,
    TAX,
    TAY,
    TXA,
    TYA,
    ADC,
    SBC,
    INC,
    DEC,
    INX,
    DEX,
    INY,
    DEY,
    ASL,
    LSR,
    ROL,
    ROR,
    AND,
    ORA,
    EOR,
    BIT,
    CMP,
    CPX,
    CPY,
    BCC,
    BCS,
    BEQ,
    BNE,
    BPL,
    BMI,
    BVC,
    BVS,
    JMP,
    JSR,
    RTS,
    BRK,
    RTI,
    PHA,
    PLA,
    PHP,
    PLP,
    TXS,
    TSX,
    CLC,
    SEC,
    CLI,
    SEI,
    CLD,
    SED,
    CLV,
    NOP,
}

impl std::fmt::Display for OpcodeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            OpcodeName::LDA => "LDA",
            OpcodeName::LDX => "LDX",
            OpcodeName::LDY => "LDY",
            OpcodeName::STA => "STA",
            OpcodeName::STX => "STX",
            OpcodeName::STY => "STY",
            OpcodeName::TAX => "TAX",
            OpcodeName::TAY => "TAY",
            OpcodeName::TXA => "TXA",
            OpcodeName::TYA => "TYA",
            OpcodeName::ADC => "ADC",
            OpcodeName::SBC => "SBC",
            OpcodeName::INC => "INC",
            OpcodeName::DEC => "DEC",
            OpcodeName::INX => "INX",
            OpcodeName::DEX => "DEX",
            OpcodeName::INY => "INY",
            OpcodeName::DEY => "DEY",
            OpcodeName::ASL => "ASL",
            OpcodeName::LSR => "LSR",
            OpcodeName::ROL => "ROL",
            OpcodeName::ROR => "ROR",
            OpcodeName::AND => "AND",
            OpcodeName::ORA => "ORA",
            OpcodeName::EOR => "EOR",
            OpcodeName::BIT => "BIT",
            OpcodeName::CMP => "CMP",
            OpcodeName::CPX => "CPX",
            OpcodeName::CPY => "CPY",
            OpcodeName::BCC => "BCC",
            OpcodeName::BCS => "BCS",
            OpcodeName::BEQ => "BEQ",
            OpcodeName::BNE => "BNE",
            OpcodeName::BPL => "BPL",
            OpcodeName::BMI => "BMI",
            OpcodeName::BVC => "BVC",
            OpcodeName::BVS => "BVS",
            OpcodeName::JMP => "JMP",
            OpcodeName::JSR => "JSR",
            OpcodeName::RTS => "RTS",
            OpcodeName::BRK => "BRK",
            OpcodeName::RTI => "RTI",
            OpcodeName::PHA => "PHA",
            OpcodeName::PLA => "PLA",
            OpcodeName::PHP => "PHP",
            OpcodeName::PLP => "PLP",
            OpcodeName::TXS => "TXS",
            OpcodeName::TSX => "TSX",
            OpcodeName::CLC => "CLC",
            OpcodeName::SEC => "SEC",
            OpcodeName::CLI => "CLI",
            OpcodeName::SEI => "SEI",
            OpcodeName::CLD => "CLD",
            OpcodeName::SED => "SED",
            OpcodeName::CLV => "CLV",
            OpcodeName::NOP => "NOP",
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

    pub fn get_opcode_map() -> HashMap<u8, Opcode> {
        let opcodes: Vec<Opcode> = vec![
            Opcode::new(0xA9, OpcodeName::LDA, 2, 2, AddressingMode::Immediate),
            Opcode::new(0xA5, OpcodeName::LDA, 2, 3, AddressingMode::ZeroPage),
            Opcode::new(0xB5, OpcodeName::LDA, 2, 4, AddressingMode::ZeroPageX),
            Opcode::new(0xAD, OpcodeName::LDA, 3, 4, AddressingMode::Absolute),
            Opcode::new(0xBD, OpcodeName::LDA, 3, 4, AddressingMode::AbsoluteX), // +1 cycle if page crossed
            Opcode::new(0xB9, OpcodeName::LDA, 3, 4, AddressingMode::AbsoluteY), // +1 cycle if page crossed
            Opcode::new(0xA1, OpcodeName::LDA, 2, 6, AddressingMode::IndirectX),
            Opcode::new(0xB1, OpcodeName::LDA, 2, 5, AddressingMode::IndirectY), // +1 cycle if page crossed
            Opcode::new(0xA2, OpcodeName::LDX, 2, 2, AddressingMode::Immediate),
            Opcode::new(0xA6, OpcodeName::LDX, 2, 3, AddressingMode::ZeroPage),
            Opcode::new(0xB6, OpcodeName::LDX, 2, 4, AddressingMode::ZeroPageY),
            Opcode::new(0xAE, OpcodeName::LDX, 3, 4, AddressingMode::Absolute),
            Opcode::new(0xBE, OpcodeName::LDX, 3, 4, AddressingMode::AbsoluteY), // + 1 cycle if page crossed
            Opcode::new(0xA0, OpcodeName::LDY, 2, 2, AddressingMode::Immediate),
            Opcode::new(0xA4, OpcodeName::LDY, 2, 3, AddressingMode::ZeroPage),
            Opcode::new(0xB4, OpcodeName::LDY, 2, 4, AddressingMode::ZeroPageX),
            Opcode::new(0xAC, OpcodeName::LDY, 3, 4, AddressingMode::Absolute),
            Opcode::new(0xBC, OpcodeName::LDY, 3, 4, AddressingMode::AbsoluteX), // + 1 cycle if page crossed
            Opcode::new(0x85, OpcodeName::STA, 2, 3, AddressingMode::ZeroPage),
            Opcode::new(0x95, OpcodeName::STA, 2, 4, AddressingMode::ZeroPageX),
            Opcode::new(0x8D, OpcodeName::STA, 3, 4, AddressingMode::Absolute),
            Opcode::new(0x9D, OpcodeName::STA, 3, 5, AddressingMode::AbsoluteX),
            Opcode::new(0x99, OpcodeName::STA, 3, 5, AddressingMode::AbsoluteY),
            Opcode::new(0x81, OpcodeName::STA, 2, 6, AddressingMode::IndirectX),
            Opcode::new(0x91, OpcodeName::STA, 2, 6, AddressingMode::IndirectY),
            Opcode::new(0x86, OpcodeName::STX, 2, 3, AddressingMode::ZeroPage),
            Opcode::new(0x96, OpcodeName::STX, 2, 4, AddressingMode::ZeroPageY),
            Opcode::new(0x8E, OpcodeName::STX, 3, 4, AddressingMode::Absolute),
            Opcode::new(0x84, OpcodeName::STY, 2, 3, AddressingMode::ZeroPage),
            Opcode::new(0x94, OpcodeName::STY, 2, 4, AddressingMode::ZeroPageX),
            Opcode::new(0x8C, OpcodeName::STY, 3, 4, AddressingMode::Absolute),
            // Transfer
            Opcode::new(0xAA, OpcodeName::TAX, 1, 2, AddressingMode::Implicit),
            Opcode::new(0xA8, OpcodeName::TAY, 1, 2, AddressingMode::Implicit),
            Opcode::new(0x8A, OpcodeName::TXA, 1, 2, AddressingMode::Implicit),
            Opcode::new(0x98, OpcodeName::TYA, 1, 2, AddressingMode::Implicit),
            // Arithmetic
            Opcode::new(0x69, OpcodeName::ADC, 2, 2, AddressingMode::Immediate),
            Opcode::new(0x65, OpcodeName::ADC, 2, 3, AddressingMode::ZeroPage),
            Opcode::new(0x75, OpcodeName::ADC, 2, 4, AddressingMode::ZeroPageX),
            Opcode::new(0x6D, OpcodeName::ADC, 3, 4, AddressingMode::Absolute),
            Opcode::new(0x7D, OpcodeName::ADC, 3, 4, AddressingMode::AbsoluteX), // + 1 cycle if page crossed
            Opcode::new(0x79, OpcodeName::ADC, 3, 4, AddressingMode::AbsoluteY), // + 1 cycle if page crossed
            Opcode::new(0x61, OpcodeName::ADC, 2, 6, AddressingMode::IndirectX),
            Opcode::new(0x71, OpcodeName::ADC, 2, 5, AddressingMode::IndirectY), // + 1 cycle if page crossed
            Opcode::new(0xE9, OpcodeName::SBC, 2, 2, AddressingMode::Immediate),
            Opcode::new(0xE5, OpcodeName::SBC, 2, 3, AddressingMode::ZeroPage),
            Opcode::new(0xF5, OpcodeName::SBC, 2, 4, AddressingMode::ZeroPageX),
            Opcode::new(0xED, OpcodeName::SBC, 3, 4, AddressingMode::Absolute),
            Opcode::new(0xFD, OpcodeName::SBC, 3, 4, AddressingMode::AbsoluteX), // + 1 cycle if page crossed
            Opcode::new(0xF9, OpcodeName::SBC, 3, 4, AddressingMode::AbsoluteY), // + 1 cycle if page crossed
            Opcode::new(0xE1, OpcodeName::SBC, 2, 6, AddressingMode::IndirectX),
            Opcode::new(0xF1, OpcodeName::SBC, 2, 5, AddressingMode::IndirectY), // + 1 cycle if page crossed
            Opcode::new(0xE6, OpcodeName::INC, 2, 5, AddressingMode::ZeroPage),
            Opcode::new(0xF6, OpcodeName::INC, 2, 6, AddressingMode::ZeroPageX),
            Opcode::new(0xEE, OpcodeName::INC, 3, 6, AddressingMode::Absolute),
            Opcode::new(0xFE, OpcodeName::INC, 3, 7, AddressingMode::AbsoluteX),
            Opcode::new(0xC6, OpcodeName::DEC, 2, 5, AddressingMode::ZeroPage),
            Opcode::new(0xD6, OpcodeName::DEC, 2, 6, AddressingMode::ZeroPageX),
            Opcode::new(0xCE, OpcodeName::DEC, 3, 6, AddressingMode::Absolute),
            Opcode::new(0xDE, OpcodeName::DEC, 3, 7, AddressingMode::AbsoluteX),
            Opcode::new(0xE8, OpcodeName::INX, 1, 2, AddressingMode::Implicit),
            Opcode::new(0xCA, OpcodeName::DEX, 1, 2, AddressingMode::Implicit),
            Opcode::new(0xC8, OpcodeName::INY, 1, 2, AddressingMode::Implicit),
            Opcode::new(0x88, OpcodeName::DEY, 1, 2, AddressingMode::Implicit),
            // Shift
            Opcode::new(0x0A, OpcodeName::ASL, 1, 2, AddressingMode::Accumulator),
            Opcode::new(0x06, OpcodeName::ASL, 2, 5, AddressingMode::ZeroPage),
            Opcode::new(0x16, OpcodeName::ASL, 2, 6, AddressingMode::ZeroPageX),
            Opcode::new(0x0E, OpcodeName::ASL, 3, 6, AddressingMode::Absolute),
            Opcode::new(0x1E, OpcodeName::ASL, 3, 7, AddressingMode::AbsoluteX),
            Opcode::new(0x4A, OpcodeName::LSR, 1, 2, AddressingMode::Accumulator),
            Opcode::new(0x46, OpcodeName::LSR, 2, 5, AddressingMode::ZeroPage),
            Opcode::new(0x56, OpcodeName::LSR, 2, 6, AddressingMode::ZeroPageX),
            Opcode::new(0x4E, OpcodeName::LSR, 3, 6, AddressingMode::Absolute),
            Opcode::new(0x5E, OpcodeName::LSR, 3, 7, AddressingMode::AbsoluteX),
            Opcode::new(0x2A, OpcodeName::ROL, 1, 2, AddressingMode::Accumulator),
            Opcode::new(0x26, OpcodeName::ROL, 2, 5, AddressingMode::ZeroPage),
            Opcode::new(0x36, OpcodeName::ROL, 2, 6, AddressingMode::ZeroPageX),
            Opcode::new(0x2E, OpcodeName::ROL, 3, 6, AddressingMode::Absolute),
            Opcode::new(0x3E, OpcodeName::ROL, 3, 7, AddressingMode::AbsoluteX),
            Opcode::new(0x6A, OpcodeName::ROR, 1, 2, AddressingMode::Accumulator),
            Opcode::new(0x66, OpcodeName::ROR, 2, 5, AddressingMode::ZeroPage),
            Opcode::new(0x76, OpcodeName::ROR, 2, 6, AddressingMode::ZeroPageX),
            Opcode::new(0x6E, OpcodeName::ROR, 3, 6, AddressingMode::Absolute),
            Opcode::new(0x7E, OpcodeName::ROR, 3, 7, AddressingMode::AbsoluteX),
            // Bitwise
            Opcode::new(0x29, OpcodeName::AND, 2, 2, AddressingMode::Immediate),
            Opcode::new(0x25, OpcodeName::AND, 2, 3, AddressingMode::ZeroPage),
            Opcode::new(0x35, OpcodeName::AND, 2, 4, AddressingMode::ZeroPageX),
            Opcode::new(0x2D, OpcodeName::AND, 3, 4, AddressingMode::Absolute),
            Opcode::new(0x3D, OpcodeName::AND, 3, 4, AddressingMode::AbsoluteX), // + 1 cycle if page crossed
            Opcode::new(0x39, OpcodeName::AND, 3, 4, AddressingMode::AbsoluteY), // + 1 cycle if page crossed
            Opcode::new(0x21, OpcodeName::AND, 2, 6, AddressingMode::IndirectX),
            Opcode::new(0x31, OpcodeName::AND, 2, 5, AddressingMode::IndirectY), // + 1 cycle if page crossed
            Opcode::new(0x09, OpcodeName::ORA, 2, 2, AddressingMode::Immediate),
            Opcode::new(0x05, OpcodeName::ORA, 2, 3, AddressingMode::ZeroPage),
            Opcode::new(0x15, OpcodeName::ORA, 2, 4, AddressingMode::ZeroPageX),
            Opcode::new(0x0D, OpcodeName::ORA, 3, 4, AddressingMode::Absolute),
            Opcode::new(0x1D, OpcodeName::ORA, 3, 4, AddressingMode::AbsoluteX), // + 1 cycle if page crossed
            Opcode::new(0x19, OpcodeName::ORA, 3, 4, AddressingMode::AbsoluteY), // + 1 cycle if page crossed
            Opcode::new(0x01, OpcodeName::ORA, 2, 6, AddressingMode::IndirectX),
            Opcode::new(0x11, OpcodeName::ORA, 2, 5, AddressingMode::IndirectY), // + 1 cycle if page crossed
            Opcode::new(0x49, OpcodeName::EOR, 2, 2, AddressingMode::Immediate),
            Opcode::new(0x45, OpcodeName::EOR, 2, 3, AddressingMode::ZeroPage),
            Opcode::new(0x55, OpcodeName::EOR, 2, 4, AddressingMode::ZeroPageX),
            Opcode::new(0x4D, OpcodeName::EOR, 3, 4, AddressingMode::Absolute),
            Opcode::new(0x5D, OpcodeName::EOR, 3, 4, AddressingMode::AbsoluteX), // + 1 cycle if page crossed
            Opcode::new(0x59, OpcodeName::EOR, 3, 4, AddressingMode::AbsoluteY), // + 1 cycle if page crossed
            Opcode::new(0x41, OpcodeName::EOR, 2, 6, AddressingMode::IndirectX),
            Opcode::new(0x51, OpcodeName::EOR, 2, 5, AddressingMode::IndirectY), // + 1 cycle if page crossed
            Opcode::new(0x24, OpcodeName::BIT, 2, 3, AddressingMode::ZeroPage),
            Opcode::new(0x2C, OpcodeName::BIT, 3, 4, AddressingMode::Absolute),
            // Compare
            Opcode::new(0xC9, OpcodeName::CMP, 2, 2, AddressingMode::Immediate),
            Opcode::new(0xC5, OpcodeName::CMP, 2, 3, AddressingMode::ZeroPage),
            Opcode::new(0xD5, OpcodeName::CMP, 2, 4, AddressingMode::ZeroPageX),
            Opcode::new(0xCD, OpcodeName::CMP, 3, 4, AddressingMode::Absolute),
            Opcode::new(0xDD, OpcodeName::CMP, 3, 4, AddressingMode::AbsoluteX), // + 1 cycle if page crossed
            Opcode::new(0xD9, OpcodeName::CMP, 3, 4, AddressingMode::AbsoluteY),
            Opcode::new(0xC1, OpcodeName::CMP, 2, 6, AddressingMode::IndirectX), // + 1 cycle if page crossed
            Opcode::new(0xD1, OpcodeName::CMP, 2, 5, AddressingMode::IndirectY), // + 1 cycle if page crossed
            Opcode::new(0xE0, OpcodeName::CPX, 2, 2, AddressingMode::Immediate),
            Opcode::new(0xE4, OpcodeName::CPX, 2, 3, AddressingMode::ZeroPage),
            Opcode::new(0xEC, OpcodeName::CPX, 3, 4, AddressingMode::Absolute),
            Opcode::new(0xC0, OpcodeName::CPY, 2, 2, AddressingMode::Immediate),
            Opcode::new(0xC4, OpcodeName::CPY, 2, 3, AddressingMode::ZeroPage),
            Opcode::new(0xCC, OpcodeName::CPY, 3, 4, AddressingMode::Absolute),
            // Branch
            Opcode::new(0x90, OpcodeName::BCC, 2, 2, AddressingMode::Relative), // + 1 cycle if branch taken, + 1 if page crossed
            Opcode::new(0xB0, OpcodeName::BCS, 2, 2, AddressingMode::Relative), // + 1 cycle if branch taken, + 1 if page crossed
            Opcode::new(0xF0, OpcodeName::BEQ, 2, 2, AddressingMode::Relative), // + 1 cycle if branch taken, + 1 if page crossed
            Opcode::new(0xD0, OpcodeName::BNE, 2, 2, AddressingMode::Relative), // + 1 cycle if branch taken, + 1 if page crossed
            Opcode::new(0x10, OpcodeName::BPL, 2, 2, AddressingMode::Relative), // + 1 cycle if branch taken, + 1 if page crossed
            Opcode::new(0x30, OpcodeName::BMI, 2, 2, AddressingMode::Relative), // + 1 cycle if branch taken, + 1 if page crossed
            Opcode::new(0x50, OpcodeName::BVC, 2, 2, AddressingMode::Relative), // + 1 cycle if branch taken, + 1 if page crossed
            Opcode::new(0x70, OpcodeName::BVS, 2, 2, AddressingMode::Relative), // + 1 cycle if branch taken, + 1 if page crossed
            // Jump
            Opcode::new(0x4C, OpcodeName::JMP, 3, 3, AddressingMode::Absolute),
            Opcode::new(0x6C, OpcodeName::JMP, 3, 5, AddressingMode::Indirect),
            Opcode::new(0x20, OpcodeName::JSR, 3, 6, AddressingMode::Absolute),
            Opcode::new(0x60, OpcodeName::RTS, 1, 6, AddressingMode::Implicit),
            Opcode::new(0x00, OpcodeName::BRK, 1, 7, AddressingMode::Implicit), // Only 1 byte wide, but skips the second byte so can be considered 2 bytes
            Opcode::new(0x40, OpcodeName::RTI, 1, 6, AddressingMode::Implicit),
            // tackS
            Opcode::new(0x48, OpcodeName::PHA, 1, 3, AddressingMode::Implicit),
            Opcode::new(0x68, OpcodeName::PLA, 1, 4, AddressingMode::Implicit),
            Opcode::new(0x08, OpcodeName::PHP, 1, 3, AddressingMode::Implicit),
            Opcode::new(0x28, OpcodeName::PLP, 1, 4, AddressingMode::Implicit),
            Opcode::new(0x9A, OpcodeName::TXS, 1, 2, AddressingMode::Implicit),
            Opcode::new(0xBA, OpcodeName::TSX, 1, 2, AddressingMode::Implicit),
            // Flags
            Opcode::new(0x18, OpcodeName::CLC, 1, 2, AddressingMode::Implicit),
            Opcode::new(0x38, OpcodeName::SEC, 1, 2, AddressingMode::Implicit),
            Opcode::new(0x58, OpcodeName::CLI, 1, 2, AddressingMode::Implicit),
            Opcode::new(0x78, OpcodeName::SEI, 1, 2, AddressingMode::Implicit),
            Opcode::new(0xD8, OpcodeName::CLD, 1, 2, AddressingMode::Implicit),
            Opcode::new(0xF8, OpcodeName::SED, 1, 2, AddressingMode::Implicit),
            Opcode::new(0xB8, OpcodeName::CLV, 1, 2, AddressingMode::Implicit),
            // Other
            Opcode::new(0xEA, OpcodeName::NOP, 1, 2, AddressingMode::Implicit),
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
