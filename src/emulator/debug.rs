use super::cpu::BreakpointType;

#[derive(PartialEq, Eq)]
pub enum StepMode {
    Instruction,
    Frame,
}

#[derive(PartialEq, Eq)]
pub enum DebugFlag {
    Cpu,
    Ppu,
    Json,
    Step(StepMode),
}

pub struct DebugContext {
    enabled_flags: Vec<DebugFlag>,
    cpu_instruction_log: Vec<String>,
    pc_breakpoint: Option<u16>,

    pub cpu_debug_read: bool,
}

impl DebugContext {
    pub fn new() -> Self {
        Self {
            enabled_flags: Vec::new(),
            cpu_instruction_log: Vec::new(),
            pc_breakpoint: None,

            cpu_debug_read: false,
        }
    }

    pub fn flag_enabled(&self, flag: &DebugFlag) -> bool {
        self.enabled_flags.contains(flag)
    }

    pub fn set_debug_flags(&mut self, flags: Vec<DebugFlag>) {
        self.enabled_flags = flags;
    }

    pub fn set_debug_flag(&mut self, flag: DebugFlag) {
        if !self.flag_enabled(&flag) {
            self.enabled_flags.push(flag);
        }
    }

    pub fn clear_debug_flag(&mut self, flag: &DebugFlag) {
        self.enabled_flags.retain(|x| x != flag);
    }

    pub fn toggle_debug_flag(&mut self, flag: DebugFlag) {
        if self.enabled_flags.contains(&flag) {
            self.clear_debug_flag(&flag);
        } else {
            self.set_debug_flag(flag);
        }
    }

    pub fn log_instr(&mut self, log: String) {
        self.cpu_instruction_log.push(log);

        if self.cpu_instruction_log.len() > 1000 {
            self.cpu_instruction_log.drain(0..500);
        }
    }

    pub fn last_instruction(&self) -> String {
        let last = match self.cpu_instruction_log.last() {
            Some(s) => s,
            None => &String::new(),
        };

        last.clone()
    }

    pub fn set_breakpoint(&mut self, breakpoint: Option<u16>) {
        self.pc_breakpoint = breakpoint;
    }

    pub fn hit_breakpoint(&self, bp: BreakpointType) -> bool {
        match bp {
            BreakpointType::Pc(pc) => {
                if self.pc_breakpoint.is_none() {
                    false
                } else {
                    pc == self.pc_breakpoint.unwrap()
                }
            }
            _ => false,
        }
    }
}
