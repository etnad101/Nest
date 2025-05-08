mod opcode;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::bus::Bus;
use opcode::Opcode;

pub const CLOCK_SPEED: usize = 21_441_960;

macro_rules! print_spaces {
    ($x:literal) => {
        print!("{}", " ".repeat($x));
    };
}

#[derive(Clone, Copy)]
pub enum AddressingMode {
    Accumulator,
    Implicit,
    Relative,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX,
    IndirectY,
}

pub struct Cpu {
    opcodes: HashMap<u8, Opcode>,

    debug: bool,

    bus: Rc<RefCell<Bus>>,
    cycles: usize,
    page_crossed: bool,

    // registers
    r_a: u8,
    r_x: u8,
    r_y: u8,
    r_sp: u8,
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

impl Cpu {
    pub fn new(bus: Rc<RefCell<Bus>>) -> Self {
        Self {
            opcodes: Opcode::get_opcode_map(),

            debug: false,
            bus,
            cycles: 0,
            page_crossed: false,
            r_a: 0,
            r_x: 0,
            r_y: 0,
            r_sp: 0xFD,
            r_pc: 0xFFFC,

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

    pub fn set_debug_mode(&mut self, debug: bool) {
        self.debug = debug;
    }

    #[allow(unused)]
    fn dump_mem(&self, size: usize, width: usize) {
        println!();
        for i in 0..=(size / width) {
            print!("|{:04x}|", i * width);
            for j in 0..width {
                let addr = (i * width) + j;
                print!("{:02x} ", self.read(addr as u16));
            }
            println!();
        }
    }

    fn read(&self, addr: u16) -> u8 {
        self.bus.borrow().cpu_read(addr)
    }

    fn write(&mut self, addr: u16, value: u8) {
        self.bus.borrow_mut().write(addr, value);
    }

    fn indirect_x_ptr(&mut self) -> u16 {
        let base: u8 = self.read(self.r_pc);
        self.r_pc += 1;
        let lo_ptr: u8 = base.wrapping_add(self.r_x);
        let lo: u16 = self.read(lo_ptr.into()) as u16;
        let hi: u16 = self.read(lo_ptr.wrapping_add(1).into()) as u16;
        (hi << 8) | lo
    }

    fn indirect_y_ptr(&mut self) -> (u16, u16) {
        let lo_ptr: u8 = self.read(self.r_pc);
        self.r_pc += 1;
        let lo = self.read(lo_ptr.into()) as u16;
        let hi = self.read(lo_ptr.wrapping_add(1).into()) as u16;

        let base_addr: u16 = (hi << 8) | lo;
        let addr: u16 = base_addr.wrapping_add(self.r_y.into());

        if ((addr) >> 8) != (base_addr >> 8) {
            self.page_crossed = true;
        }
        (addr, base_addr)
    }

    fn get_address(&mut self, mode: AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => {
                let addr: u16 = self.r_pc;
                self.r_pc += 1;
                addr
            }
            AddressingMode::ZeroPage => {
                let addr: u16 = self.read(self.r_pc).into();
                self.r_pc += 1;
                addr
            }
            AddressingMode::ZeroPageX => {
                let addr: u8 = self.read(self.r_pc).wrapping_add(self.r_x);
                self.r_pc += 1;
                addr as u16
            }
            AddressingMode::ZeroPageY => {
                let addr: u8 = self.read(self.r_pc).wrapping_add(self.r_y);
                self.r_pc += 1;
                addr as u16
            }
            AddressingMode::Absolute => {
                let lo: u16 = self.read(self.r_pc) as u16;
                self.r_pc = self.r_pc.wrapping_add(1);
                let hi: u16 = self.read(self.r_pc) as u16;
                self.r_pc = self.r_pc.wrapping_add(1);
                (hi << 8) | lo
            }
            AddressingMode::AbsoluteX => {
                let lo: u16 = self.read(self.r_pc) as u16;
                self.r_pc += 1;
                let hi: u16 = self.read(self.r_pc) as u16;
                self.r_pc += 1;

                let base_addr: u16 = (hi << 8) | lo;
                let offset: u16 = self.r_x as u16;

                if ((base_addr + offset) >> 8) != (base_addr >> 8) {
                    self.page_crossed = true;
                }

                base_addr + offset
            }
            AddressingMode::AbsoluteY => {
                let lo: u16 = self.read(self.r_pc) as u16;
                self.r_pc += 1;
                let hi: u16 = self.read(self.r_pc) as u16;
                self.r_pc += 1;

                let base_addr: u16 = (hi << 8) | lo;
                let offset: u16 = self.r_y as u16;

                if ((base_addr.wrapping_add(offset)) >> 8) != (base_addr >> 8) {
                    self.page_crossed = true;
                }

                base_addr.wrapping_add(offset)
            }
            AddressingMode::Indirect => {
                let lo_ptr: u8 = self.read(self.r_pc);
                self.r_pc += 1;
                let hi_ptr: u8 = self.read(self.r_pc);
                self.r_pc += 1;

                let next_lo: u8 = lo_ptr.wrapping_add(1);

                let lo: u16 = ((hi_ptr as u16) << 8) | (lo_ptr as u16);
                let hi: u16 = ((hi_ptr as u16) << 8) | (next_lo as u16);

                let lo_val: u16 = self.read(lo) as u16;
                let hi_val: u16 = self.read(hi) as u16;

                (hi_val << 8) | lo_val
            }
            AddressingMode::IndirectX => self.indirect_x_ptr(),
            AddressingMode::IndirectY => self.indirect_y_ptr().0,
            _ => panic!("Addressing mode should not return an address"),
        }
    }

    fn log_instr(&mut self, opcode: &Opcode) {
        if !self.debug {
            return;
        }
        print!("{:04X}  ", self.r_pc);

        let mut args: [u8; 3] = [0; 3];

        for i in 0..opcode.bytes() {
            let byte = self.read(self.r_pc + i);
            print!("{:02X} ", byte);
            args[i as usize] = byte;
        }

        for _ in 0..3 - opcode.bytes() {
            print!("   ");
        }

        print!(" {} ", opcode.name());

        let temp_pc = self.r_pc;
        let mut use_postfix = false;

        let end_val: usize = match opcode.name() {
            "BCC" | "BCS" | "BEQ" | "BNE" | "BPL" | "BMI" | "BVC" | "BVS" => self
                .calculate_branch_addr(self.read(self.r_pc + 1))
                .wrapping_add(1)
                .into(),
            "STX" | "BIT" | "STA" | "LDX" | "LDA" | "ORA" | "AND" | "EOR" | "ADC" | "CMP"
            | "SBC" | "LDY" | "STY" | "CPX" | "CPY" | "LSR" | "ASL" | "ROL" | "ROR" | "INC"
            | "DEC" => {
                if let AddressingMode::Accumulator = opcode.mode() {
                    0
                } else {
                    use_postfix = true;
                    self.r_pc += 1;
                    let addr = self.get_address(opcode.mode());
                    self.read(addr).into()
                }
            }
            "JMP" => {
                if let AddressingMode::Absolute = opcode.mode() {
                    0
                } else {
                    self.r_pc += 1;
                    use_postfix = true;
                    self.get_address(opcode.mode()).into()
                }
            }
            _ => 0,
        };

        self.r_pc = temp_pc;

        match opcode.mode() {
            AddressingMode::Accumulator => {
                print!("A");
                print_spaces!(27);
            }
            AddressingMode::Absolute => {
                print!("${:02X}{:02X}", args[2], args[1]);
                if use_postfix {
                    print!(" = {:02X}", end_val);
                    print_spaces!(18);
                } else {
                    print_spaces!(23);
                }
            }
            AddressingMode::AbsoluteX => {
                self.r_pc += 1;
                print!(
                    "${:02X}{:02X},X @ {:04X} = {:02X}",
                    args[2],
                    args[1],
                    self.get_address(opcode.mode()),
                    end_val
                );
                print_spaces!(9);
            }
            AddressingMode::AbsoluteY => {
                self.r_pc += 1;
                print!(
                    "${:02X}{:02X},Y @ {:04X} = {:02X}",
                    args[2],
                    args[1],
                    self.get_address(opcode.mode()),
                    end_val
                );
                print_spaces!(9);
            }
            AddressingMode::Immediate => {
                print!("#${:02X}", args[1]);
                print_spaces!(24);
            }
            AddressingMode::Indirect => {
                print!("(${:02X}{:02X}) = {:04X}", args[2], args[1], end_val);
                print_spaces!(14);
            }
            AddressingMode::IndirectX => {
                let addr1: u8 = args[1].wrapping_add(self.r_x);
                self.r_pc = self.r_pc.wrapping_add(1);
                print!(
                    "(${:02X},X) @ {:02X} = {:04X} = {:02X}",
                    args[1],
                    addr1,
                    self.indirect_x_ptr(),
                    end_val
                );
                print_spaces!(4);
            }
            AddressingMode::IndirectY => {
                self.r_pc = self.r_pc.wrapping_add(1);
                let (addr2, addr1) = self.indirect_y_ptr();
                print!(
                    "(${:02X}),Y = {:04X} @ {:04X} = {:02X}",
                    args[1], addr1, addr2, end_val
                );
                print_spaces!(2);
            }
            AddressingMode::ZeroPage => {
                print!("${:02X} = {:02X}", self.read(self.r_pc + 1), end_val);
                //self.dump_ram();
                print_spaces!(20);
            }
            AddressingMode::ZeroPageX => {
                print!(
                    "${:02X},X @ {:02X} = {:02X}",
                    args[1],
                    self.read(self.r_pc + 1).wrapping_add(self.r_x),
                    end_val
                );
                print_spaces!(13);
            }
            AddressingMode::ZeroPageY => {
                print!(
                    "${:02X},Y @ {:02X} = {:02X}",
                    args[1],
                    self.read(self.r_pc + 1).wrapping_add(self.r_y),
                    end_val
                );
                print_spaces!(13);
            }
            AddressingMode::Relative => {
                print!("${:04X}", end_val);
                print_spaces!(23);
            }
            AddressingMode::Implicit => {
                print_spaces!(28);
            }
        }

        self.r_pc = temp_pc;

        let p = self.get_p(false);
        println!(
            "A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X} CYC:{}",
            self.r_a, self.r_x, self.r_y, p, self.r_sp, self.cycles
        );
    }

    fn get_p(&mut self, f_b: bool) -> u8 {
        let n_flag: u8 = if self.f_n { 1 } else { 0 };
        let v_flag: u8 = if self.f_v { 1 } else { 0 };
        let d_flag: u8 = if self.f_d { 1 } else { 0 };
        let i_flag: u8 = if self.f_i { 1 } else { 0 };
        let z_flag: u8 = if self.f_z { 1 } else { 0 };
        let c_flag: u8 = if self.f_c { 1 } else { 0 };
        let b_flag: u8 = if f_b { 1 } else { 0 };

        (n_flag << 7)
            | (v_flag << 6)
            | (1 << 5)
            | (b_flag << 4)
            | (d_flag << 3)
            | (i_flag << 2)
            | (z_flag << 1)
            | c_flag
    }

    fn set_p(&mut self, flags: u8, update_i_now: bool) {
        self.f_c = (flags & 0x01) > 0;
        self.f_z = (flags & 0x02) > 0;
        self.f_d = (flags & 0x08) > 0;
        self.f_v = (flags & 0x40) > 0;
        self.f_n = (flags & 0x80) > 0;

        if update_i_now {
            self.f_i = (flags & 0x04) > 0;
        } else {
            self.pending_iflag_value = (flags & 0x04) > 0;
            self.pending_iflag_update = true;
        }
    }

    fn update_zn_flags(&mut self, value: u8) {
        self.f_z = value == 0;
        self.f_n = (value & 0x80) > 0;
    }

    fn compare(&mut self, reg: u8, mode: AddressingMode) {
        let addr = self.get_address(mode);
        let value = self.read(addr);

        self.f_c = reg >= value;
        self.update_zn_flags(reg.wrapping_sub(value));
        if self.page_crossed {
            self.cycles += 1;
        }
    }

    fn calculate_branch_addr(&mut self, offset: u8) -> u16 {
        let mut addr = self.r_pc as i16;
        if offset & 0x80 > 0 {
            let neg = (!offset + 1) as i16;
            addr -= neg;
        } else {
            addr += offset as i16
        }

        (addr as u16).wrapping_add(1)
    }

    fn branch(&mut self, op_name: &'static str) {
        let mut branch = false;
        match op_name {
            "BCC" => {
                if !self.f_c {
                    branch = true
                }
            }
            "BCS" => {
                if self.f_c {
                    branch = true
                }
            }
            "BEQ" => {
                if self.f_z {
                    branch = true
                }
            }
            "BNE" => {
                if !self.f_z {
                    branch = true
                }
            }
            "BPL" => {
                if !self.f_n {
                    branch = true
                }
            }
            "BMI" => {
                if self.f_n {
                    branch = true
                }
            }
            "BVC" => {
                if !self.f_v {
                    branch = true
                }
            }
            "BVS" => {
                if self.f_v {
                    branch = true
                }
            }
            _ => panic!("ERROR: Only branch opcodes should be calling this function"),
        }

        if branch {
            self.cycles += 1;
            let offset = self.read(self.r_pc);
            // let pre_pc = self.r_pc;
            self.r_pc = self.calculate_branch_addr(offset);
            // I think i should add a cycle if it crosses a page, but the tests say otherwise
            // if (((self.pc) >> 8) != ( prePc >> 8)) {
            //     self.cycles++;
            // }
        } else {
            self.r_pc = self.r_pc.wrapping_add(1);
        }
    }

    fn push_stack(&mut self, value: u8) {
        let addr = (self.r_sp as u16) + 0x0100;
        self.write(addr, value);
        self.r_sp = self.r_sp.wrapping_sub(1);
    }

    fn pop_stack(&mut self) -> u8 {
        self.r_sp = self.r_sp.wrapping_add(1);
        let addr = (self.r_sp as u16) + 0x0100;
        self.read(addr)
    }

    // instructions
    fn i_lda(&mut self, mode: AddressingMode) {
        let addr = self.get_address(mode);
        self.r_a = self.read(addr);
        self.update_zn_flags(self.r_a);
        if self.page_crossed {
            self.cycles += 1
        }
    }

    fn i_sta(&mut self, mode: AddressingMode) {
        let addr = self.get_address(mode);
        self.write(addr, self.r_a);
    }

    fn i_ldx(&mut self, mode: AddressingMode) {
        let addr = self.get_address(mode);
        self.r_x = self.read(addr);
        self.update_zn_flags(self.r_x);
        if self.page_crossed {
            self.cycles += 1
        }
    }

    fn i_stx(&mut self, mode: AddressingMode) {
        let addr = self.get_address(mode);
        self.write(addr, self.r_x);
    }

    fn i_ldy(&mut self, mode: AddressingMode) {
        let addr = self.get_address(mode);
        self.r_y = self.read(addr);
        self.update_zn_flags(self.r_y);
        if self.page_crossed {
            self.cycles += 1
        }
    }

    fn i_sty(&mut self, mode: AddressingMode) {
        let addr = self.get_address(mode);
        self.write(addr, self.r_y);
    }

    fn adc_helper(&mut self, mode: AddressingMode, sub: bool) {
        let addr = self.get_address(mode);
        let a = self.r_a as u16;
        let mut b = self.read(addr) as u16;
        if sub {
            b ^= 0xFF;
        }
        let mut sum = a + b;

        if self.f_c {
            sum += 1;
        }

        self.r_a = sum as u8;

        self.f_c = sum > 0xFF;
        self.f_v = ((sum ^ a) & (sum ^ b) & 0x80) > 0;

        self.update_zn_flags(self.r_a);

        if self.page_crossed {
            self.cycles += 1
        }
    }

    fn i_adc(&mut self, mode: AddressingMode) {
        self.adc_helper(mode, false);
    }

    fn i_sbc(&mut self, mode: AddressingMode) {
        self.adc_helper(mode, true);
    }

    fn i_inc(&mut self, mode: AddressingMode) {
        let addr = self.get_address(mode);
        let value = self.read(addr).wrapping_add(1);
        self.write(addr, value);
        self.update_zn_flags(value);
    }

    fn i_dec(&mut self, mode: AddressingMode) {
        let addr = self.get_address(mode);
        let value = self.read(addr).wrapping_sub(1);
        self.write(addr, value);
        self.update_zn_flags(value);
    }

    fn i_asl(&mut self, mode: AddressingMode) {
        let value: u8;
        if let AddressingMode::Accumulator = mode {
            value = self.r_a;
            self.r_a = value << 1;
        } else {
            let addr = self.get_address(mode);
            value = self.read(addr);
            self.write(addr, value << 1);
        }

        self.f_c = (value & 0x80) > 0;

        self.update_zn_flags(value << 1);
    }

    fn i_lsr(&mut self, mode: AddressingMode) {
        let value: u8;
        if let AddressingMode::Accumulator = mode {
            value = self.r_a;
            self.r_a = value >> 1;
        } else {
            let addr = self.get_address(mode);
            value = self.read(addr);
            self.write(addr, value >> 1);
        }

        self.f_c = (value & 1) > 0;

        self.update_zn_flags(value >> 1);
    }

    fn i_rol(&mut self, mode: AddressingMode) {
        let value: u8;
        let new_value;
        let carry_bit = if self.f_c { 1 } else { 0 };

        if let AddressingMode::Accumulator = mode {
            value = self.r_a;
            new_value = (value << 1) | carry_bit;
            self.r_a = new_value;
        } else {
            let addr = self.get_address(mode);
            value = self.read(addr);
            new_value = (value << 1) | carry_bit;
            self.write(addr, new_value);
        }

        self.f_c = (value & 0x80) > 0;

        self.update_zn_flags(new_value);
    }

    fn i_ror(&mut self, mode: AddressingMode) {
        let value: u8;
        let new_value;
        let carry_bit = if self.f_c { 0x80 } else { 0 };

        if let AddressingMode::Accumulator = mode {
            value = self.r_a;
            new_value = (value >> 1) | carry_bit;
            self.r_a = new_value;
        } else {
            let addr = self.get_address(mode);
            value = self.read(addr);
            new_value = (value >> 1) | carry_bit;
            self.write(addr, new_value);
        }

        self.f_c = (value & 1) > 0;

        self.update_zn_flags(new_value);
    }

    fn i_and(&mut self, mode: AddressingMode) {
        let addr = self.get_address(mode);
        self.r_a &= self.read(addr);
        self.update_zn_flags(self.r_a);
        if self.page_crossed {
            self.cycles += 1
        }
    }

    fn i_ora(&mut self, mode: AddressingMode) {
        let addr = self.get_address(mode);
        self.r_a |= self.read(addr);
        self.update_zn_flags(self.r_a);
        if self.page_crossed {
            self.cycles += 1
        }
    }

    fn i_eor(&mut self, mode: AddressingMode) {
        let addr = self.get_address(mode);
        self.r_a ^= self.read(addr);
        self.update_zn_flags(self.r_a);
        if self.page_crossed {
            self.cycles += 1
        }
    }

    fn i_bit(&mut self, mode: AddressingMode) {
        let addr = self.get_address(mode);
        let mem = self.read(addr);
        let res = self.r_a & mem;

        self.f_v = (mem & 0x40) > 0;
        self.f_n = (mem & 0x80) > 0;
        self.f_z = res == 0;
    }

    fn i_jsr(&mut self, mode: AddressingMode) {
        let addr = self.get_address(mode);
        let lo = (self.r_pc - 1) as u8;
        let hi = ((self.r_pc - 1) >> 8) as u8;
        self.push_stack(hi);
        self.push_stack(lo);
        self.r_pc = addr;
    }

    fn i_rts(&mut self) {
        let lo = self.pop_stack() as u16;
        let hi = self.pop_stack() as u16;
        let addr = (hi << 8) | lo;
        self.r_pc = addr + 1;
    }

    fn i_brk(&mut self) {
        let addr = self.r_pc + 2;
        let lo = addr as u8;
        let hi = (addr >> 8) as u8;
        let p = self.get_p(true);

        self.push_stack(hi);
        self.push_stack(lo);
        self.push_stack(p);

        self.f_i = true;
        self.r_pc = 0xFFFE;
    }

    fn i_rti(&mut self) {
        let p = self.pop_stack();
        self.set_p(p, true);

        let lo = self.pop_stack() as u16;
        let hi = self.pop_stack() as u16;
        let addr = (hi << 8) | lo;

        self.r_pc = addr;
    }

    pub fn tick(&mut self) -> usize {
        self.page_crossed = false;
        // fetch opcode
        let code = self.read(self.r_pc);

        let opcode = self
            .opcodes
            .get(&code)
            .cloned()
            .unwrap_or_else(|| panic!("Unknown opcode {:#04X} @ {:#06X}", code, self.r_pc));

        self.log_instr(&opcode);

        self.r_pc += 1;

        match opcode.name() {
            "LDA" => self.i_lda(opcode.mode()),
            "STA" => self.i_sta(opcode.mode()),
            "LDX" => self.i_ldx(opcode.mode()),
            "STX" => self.i_stx(opcode.mode()),
            "LDY" => self.i_ldy(opcode.mode()),
            "STY" => self.i_sty(opcode.mode()),
            "TAX" => {
                self.r_x = self.r_a;
                self.update_zn_flags(self.r_x);
            }
            "TAY" => {
                self.r_y = self.r_a;
                self.update_zn_flags(self.r_y);
            }
            "TXA" => {
                self.r_a = self.r_x;
                self.update_zn_flags(self.r_a);
            }
            "TYA" => {
                self.r_a = self.r_y;
                self.update_zn_flags(self.r_a);
            }
            "ADC" => self.i_adc(opcode.mode()),
            "SBC" => self.i_sbc(opcode.mode()),
            "INC" => self.i_inc(opcode.mode()),
            "DEC" => self.i_dec(opcode.mode()),
            "INX" => {
                self.r_x = self.r_x.wrapping_add(1);
                self.update_zn_flags(self.r_x);
            }
            "DEX" => {
                self.r_x = self.r_x.wrapping_sub(1);
                self.update_zn_flags(self.r_x);
            }
            "INY" => {
                self.r_y = self.r_y.wrapping_add(1);
                self.update_zn_flags(self.r_y);
            }
            "DEY" => {
                self.r_y = self.r_y.wrapping_sub(1);
                self.update_zn_flags(self.r_y);
            }
            "ASL" => self.i_asl(opcode.mode()),
            "LSR" => self.i_lsr(opcode.mode()),
            "ROL" => self.i_rol(opcode.mode()),
            "ROR" => self.i_ror(opcode.mode()),
            "AND" => self.i_and(opcode.mode()),
            "ORA" => self.i_ora(opcode.mode()),
            "EOR" => self.i_eor(opcode.mode()),
            "BIT" => self.i_bit(opcode.mode()),
            "CMP" => self.compare(self.r_a, opcode.mode()),
            "CPX" => self.compare(self.r_x, opcode.mode()),
            "CPY" => self.compare(self.r_y, opcode.mode()),
            "BCC" | "BCS" | "BEQ" | "BNE" | "BPL" | "BMI" | "BVC" | "BVS" => {
                self.branch(opcode.name())
            }
            "JMP" => self.r_pc = self.get_address(opcode.mode()),
            "JSR" => self.i_jsr(opcode.mode()),
            "RTS" => self.i_rts(),
            "BRK" => self.i_brk(),
            "RTI" => self.i_rti(),
            "PHA" => self.push_stack(self.r_a),
            "PLA" => {
                self.r_a = self.pop_stack();
                self.update_zn_flags(self.r_a);
            }
            "PHP" => {
                let p = self.get_p(true);
                self.push_stack(p);
            }
            "PLP" => {
                let p = self.pop_stack();
                self.set_p(p, false);
            }
            "TXS" => self.r_sp = self.r_x,
            "TSX" => {
                self.r_x = self.r_sp;
                self.update_zn_flags(self.r_x);
            }
            "CLC" => self.f_c = false,
            "SEC" => self.f_c = true,
            "CLI" => {
                self.pending_iflag_value = false;
                self.pending_iflag_update = true;
            }
            "SEI" => {
                self.pending_iflag_value = true;
                self.pending_iflag_update = true;
            }
            "CLD" => self.f_d = false,
            "SED" => self.f_d = true,
            "CLV" => self.f_v = false,
            "NOP" => (),
            _ => panic!("opcode does not exist"),
        }

        self.cycles += opcode.cycles();
        opcode.cycles()
    }

    fn poll_irq(&mut self) {
        // Do something here

        // update I flag after polling irq
        if self.pending_iflag_update {
            self.f_i = self.pending_iflag_value;
            self.pending_iflag_update = false;
        }
    }

    pub fn reset(&mut self) {
        self.r_pc = 0xFFFC;
        self.r_sp = 0xFD;
        self.f_i = true;

        let lo = self.read(self.r_pc) as u16;
        let hi = self.read(self.r_pc + 1) as u16;
        self.r_pc = (hi << 8) | lo;

        self.cycles = 5;
        // TODO: remove this when not using nestest
        // self.r_pc = 0xC000; 
        // self.cycles += 2;
    }

    pub fn cycles(&mut self) -> usize {
        self.cycles
    }
}
