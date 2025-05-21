pub mod opcode;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::{bus::Bus, CpuState};
use opcode::{Opcode, OpcodeName};

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
    logged_instruction: String,
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
            logged_instruction: String::new(),
        }
    }

    pub fn get_state(&self) -> CpuState {
        let p = self.get_p(false);
        CpuState {
            cycles: self.cycles,
            r_a: self.r_a,
            r_x: self.r_x,
            r_y: self.r_y,
            r_sp: self.r_sp,
            r_pc: self.r_pc,
            f_c: self.f_c,
            f_z: self.f_z,
            f_i: self.f_i,
            f_d: self.f_d,
            f_v: self.f_v,
            f_n: self.f_n,
            p: p,
        }
    }

    pub fn load_state(&mut self, state: CpuState) {
        self.r_pc = state.r_pc;
        self.r_sp = state.r_sp;
        self.r_a = state.r_a;
        self.r_x = state.r_x;
        self.r_y = state.r_y;
        self.set_p(state.p, true);
    }

    pub fn get_logged_instr(&self) -> String {
        self.logged_instruction.clone()
    }

    pub fn set_debug_mode(&mut self, debug: bool) {
        self.debug = debug;
    }

    #[allow(unused)]
    fn dump_mem(&mut self, size: usize, width: usize) {
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

    fn read(&mut self, addr: u16) -> u8 {
        self.bus.borrow_mut().cpu_read(addr)
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

    pub fn log_instr(&mut self, opcode: &Opcode) {
        if !self.debug {
            return;
        }

        let temp_pc = self.r_pc;
        let mut use_suffix = false;

        self.bus.borrow_mut().set_cpu_debug_read(true);

        self.logged_instruction = format!("{:04X}  ", self.r_pc);

        let mut args: [u8; 3] = [0; 3];

        for i in 0..opcode.bytes() {
            let byte = self.read(self.r_pc + i);
            self.logged_instruction.push_str(&format!("{byte:02X} "));
            args[i as usize] = byte;
        }

        for _ in 0..3 - opcode.bytes() {
            self.logged_instruction.push_str("   ");
        }

        self.logged_instruction.push_str(&format!(" {} ", opcode.name()));

        // TODO: Something here is messing up the cpu state
        let end_val: usize = match opcode.name() {
            OpcodeName::Bcc
            | OpcodeName::Bcs
            | OpcodeName::Beq
            | OpcodeName::Bne
            | OpcodeName::Bpl
            | OpcodeName::Bmi
            | OpcodeName::Bvc
            | OpcodeName::Bvs => {
                let addr = self.read(self.r_pc + 1);
                self.calculate_branch_addr(addr).wrapping_add(1).into()
            }
            OpcodeName::Stx
            | OpcodeName::Bit
            | OpcodeName::Sta
            | OpcodeName::Ldx
            | OpcodeName::Lda
            | OpcodeName::Ora
            | OpcodeName::And
            | OpcodeName::Eor
            | OpcodeName::Adc
            | OpcodeName::Cmp
            | OpcodeName::Sbc
            | OpcodeName::Ldy
            | OpcodeName::Sty
            | OpcodeName::Cpx
            | OpcodeName::Cpy
            | OpcodeName::Lsr
            | OpcodeName::Asl
            | OpcodeName::Rol
            | OpcodeName::Ror
            | OpcodeName::Inc
            | OpcodeName::Dec => {
                if let AddressingMode::Accumulator = opcode.mode() {
                    0
                } else {
                    use_suffix = true;
                    self.r_pc += 1;
                    let addr = self.get_address(opcode.mode());
                    self.read(addr).into()
                }
            }
            OpcodeName::Jmp => {
                if let AddressingMode::Absolute = opcode.mode() {
                    0
                } else {
                    self.r_pc += 1;
                    use_suffix = true;
                    self.get_address(opcode.mode()).into()
                }
            }
            _ => 0,
        };

        self.r_pc = temp_pc;

        match opcode.mode() {
            AddressingMode::Accumulator => {
                self.logged_instruction.push('A');
            }
            AddressingMode::Absolute => {
                self.logged_instruction.push_str(&format!("${:02X}{:02X}", args[2], args[1]));
                if use_suffix {
                    self.logged_instruction.push_str(&format!(" = {end_val:02X}"));
                } else {
                }
            }
            AddressingMode::AbsoluteX => {
                self.r_pc += 1;
                let addr = self.get_address(opcode.mode());
                self.logged_instruction.push_str(&format!(
                    "${:02X}{:02X},X @ {:04X} = {:02X}",
                    args[2],
                    args[1],
                    addr,
                    end_val
                ));
            }
            AddressingMode::AbsoluteY => {
                self.r_pc += 1;
                let addr = self.get_address(opcode.mode());
                self.logged_instruction.push_str(&format!(
                    "${:02X}{:02X},Y @ {:04X} = {:02X}",
                    args[2],
                    args[1],
                    addr,
                    end_val
                ));
            }
            AddressingMode::Immediate => {
                self.logged_instruction.push_str(&format!("#${:02X}", args[1]));
            }
            AddressingMode::Indirect => {
                self.logged_instruction.push_str(&format!("(${:02X}{:02X}) = {:04X}", args[2], args[1], end_val));
            }
            AddressingMode::IndirectX => {
                let addr1: u8 = args[1].wrapping_add(self.r_x);
                self.r_pc = self.r_pc.wrapping_add(1);
                let indirect_x_ptr = self.indirect_x_ptr();
                self.logged_instruction.push_str(&format!(
                    "(${:02X},X) @ {:02X} = {:04X} = {:02X}",
                    args[1],
                    addr1,
                    indirect_x_ptr,
                    end_val
                ));
            }
            AddressingMode::IndirectY => {
                self.r_pc = self.r_pc.wrapping_add(1);
                let (addr2, addr1) = self.indirect_y_ptr();
                self.logged_instruction.push_str(&format!(
                    "(${:02X}),Y = {:04X} @ {:04X} = {:02X}",
                    args[1], addr1, addr2, end_val
                ));
            }
            AddressingMode::ZeroPage => {
                let val = self.read(self.r_pc + 1);
                self.logged_instruction.push_str(&format!("${val:02X} = {end_val:02X}"));
            }
            AddressingMode::ZeroPageX => {
                let val = self.read(self.r_pc + 1).wrapping_add(self.r_x);
                self.logged_instruction.push_str(&format!(
                    "${:02X},X @ {:02X} = {:02X}",
                    args[1],
                    val,
                    end_val
                ));
            }
            AddressingMode::ZeroPageY => {
                let val = self.read(self.r_pc + 1).wrapping_add(self.r_y);
                self.logged_instruction.push_str(&format!(
                    "${:02X},Y @ {:02X} = {:02X}",
                    args[1],
                    val,
                    end_val
                ));
            }
            AddressingMode::Relative => {
                self.logged_instruction.push_str(&format!("${end_val:04X}"));
            }
            AddressingMode::Implicit => {
            }
        }

        self.r_pc = temp_pc;

        self.bus.borrow_mut().set_cpu_debug_read(false);
    }

    fn get_p(&self, f_b: bool) -> u8 {
        let n_flag: u8 = u8::from(self.f_n);
        let v_flag: u8 = u8::from(self.f_v);
        let d_flag: u8 = u8::from(self.f_d);
        let i_flag: u8 = u8::from(self.f_i);
        let z_flag: u8 = u8::from(self.f_z);
        let c_flag: u8 = u8::from(self.f_c);
        let b_flag: u8 = u8::from(f_b);

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

    fn branch(&mut self, op_name: OpcodeName) {
        let mut branch = false;
        match op_name {
            OpcodeName::Bcc => {
                if !self.f_c {
                    branch = true;
                }
            }
            OpcodeName::Bcs => {
                if self.f_c {
                    branch = true;
                }
            }
            OpcodeName::Beq => {
                if self.f_z {
                    branch = true;
                }
            }
            OpcodeName::Bne => {
                if !self.f_z {
                    branch = true;
                }
            }
            OpcodeName::Bpl => {
                if !self.f_n {
                    branch = true;
                }
            }
            OpcodeName::Bmi => {
                if self.f_n {
                    branch = true;
                }
            }
            OpcodeName::Bvc => {
                if !self.f_v {
                    branch = true;
                }
            }
            OpcodeName::Bvs => {
                if self.f_v {
                    branch = true;
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
            self.cycles += 1;
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
            self.cycles += 1;
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
            self.cycles += 1;
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
            self.cycles += 1;
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
        let carry_bit = u8::from(self.f_c);

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
            self.cycles += 1;
        }
    }

    fn i_ora(&mut self, mode: AddressingMode) {
        let addr = self.get_address(mode);
        self.r_a |= self.read(addr);
        self.update_zn_flags(self.r_a);
        if self.page_crossed {
            self.cycles += 1;
        }
    }

    fn i_eor(&mut self, mode: AddressingMode) {
        let addr = self.get_address(mode);
        self.r_a ^= self.read(addr);
        self.update_zn_flags(self.r_a);
        if self.page_crossed {
            self.cycles += 1;
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
        let addr = self.r_pc + 1;
        let lo = addr as u8;
        let hi = (addr >> 8) as u8;
        let p = self.get_p(true);

        self.push_stack(hi);
        self.push_stack(lo);
        self.push_stack(p);

        self.f_i = true;
        self.r_pc = 0xFFFE;
        let lo = self.read(self.r_pc) as u16;
        let hi = self.read(self.r_pc + 1) as u16;
        self.r_pc = (hi << 8) | lo;
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
            OpcodeName::Lda => self.i_lda(opcode.mode()),
            OpcodeName::Sta => self.i_sta(opcode.mode()),
            OpcodeName::Ldx => self.i_ldx(opcode.mode()),
            OpcodeName::Stx => self.i_stx(opcode.mode()),
            OpcodeName::Ldy => self.i_ldy(opcode.mode()),
            OpcodeName::Sty => self.i_sty(opcode.mode()),
            OpcodeName::Tax => {
                self.r_x = self.r_a;
                self.update_zn_flags(self.r_x);
            }
            OpcodeName::Tay => {
                self.r_y = self.r_a;
                self.update_zn_flags(self.r_y);
            }
            OpcodeName::Txa => {
                self.r_a = self.r_x;
                self.update_zn_flags(self.r_a);
            }
            OpcodeName::Tya => {
                self.r_a = self.r_y;
                self.update_zn_flags(self.r_a);
            }
            OpcodeName::Adc => self.i_adc(opcode.mode()),
            OpcodeName::Sbc => self.i_sbc(opcode.mode()),
            OpcodeName::Inc => self.i_inc(opcode.mode()),
            OpcodeName::Dec => self.i_dec(opcode.mode()),
            OpcodeName::Inx => {
                self.r_x = self.r_x.wrapping_add(1);
                self.update_zn_flags(self.r_x);
            }
            OpcodeName::Dex => {
                self.r_x = self.r_x.wrapping_sub(1);
                self.update_zn_flags(self.r_x);
            }
            OpcodeName::Iny => {
                self.r_y = self.r_y.wrapping_add(1);
                self.update_zn_flags(self.r_y);
            }
            OpcodeName::Dey => {
                self.r_y = self.r_y.wrapping_sub(1);
                self.update_zn_flags(self.r_y);
            }
            OpcodeName::Asl => self.i_asl(opcode.mode()),
            OpcodeName::Lsr => self.i_lsr(opcode.mode()),
            OpcodeName::Rol => self.i_rol(opcode.mode()),
            OpcodeName::Ror => self.i_ror(opcode.mode()),
            OpcodeName::And => self.i_and(opcode.mode()),
            OpcodeName::Ora => self.i_ora(opcode.mode()),
            OpcodeName::Eor => self.i_eor(opcode.mode()),
            OpcodeName::Bit => self.i_bit(opcode.mode()),
            OpcodeName::Cmp => self.compare(self.r_a, opcode.mode()),
            OpcodeName::Cpx => self.compare(self.r_x, opcode.mode()),
            OpcodeName::Cpy => self.compare(self.r_y, opcode.mode()),
            OpcodeName::Bcc
            | OpcodeName::Bcs
            | OpcodeName::Beq
            | OpcodeName::Bne
            | OpcodeName::Bpl
            | OpcodeName::Bmi
            | OpcodeName::Bvc
            | OpcodeName::Bvs => self.branch(opcode.name()),
            OpcodeName::Jmp => self.r_pc = self.get_address(opcode.mode()),
            OpcodeName::Jsr => self.i_jsr(opcode.mode()),
            OpcodeName::Rts => self.i_rts(),
            OpcodeName::Brk => self.i_brk(),
            OpcodeName::Rti => self.i_rti(),
            OpcodeName::Pha => self.push_stack(self.r_a),
            OpcodeName::Pla => {
                self.r_a = self.pop_stack();
                self.update_zn_flags(self.r_a);
            }
            OpcodeName::Php => {
                let p = self.get_p(true);
                self.push_stack(p);
            }
            OpcodeName::Plp => {
                let p = self.pop_stack();
                self.set_p(p, true);
            }
            OpcodeName::Txs => self.r_sp = self.r_x,
            OpcodeName::Tsx => {
                self.r_x = self.r_sp;
                self.update_zn_flags(self.r_x);
            }
            OpcodeName::Clc => self.f_c = false,
            OpcodeName::Sec => self.f_c = true,
            OpcodeName::Cli => {
                self.f_i = false;
            }
            OpcodeName::Sei => {
                self.f_i = true;
            }
            OpcodeName::Cld => self.f_d = false,
            OpcodeName::Sed => self.f_d = true,
            OpcodeName::Clv => self.f_v = false,
            OpcodeName::Nop => (),
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
