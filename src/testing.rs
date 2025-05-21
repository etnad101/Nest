use serde::{Deserialize, Serialize};

use crate::emulator::{self, CpuState, Emulator};

#[derive(Serialize, Deserialize)]
struct State {
pc: u16, 
s: u8, 
a: u8, 
x: u8, 
y: u8, 
p: u8, 
ram: Vec<(u16, u8)>
}

#[derive(Serialize, Deserialize)]
struct JsonData {
    name: String,
    initial: State,
    #[serde(rename = "final")]
    fin: State,
}

fn run_single_test(emulator: &mut Emulator, test: JsonData) -> bool {
    // Setting initial state
    let initial = CpuState {
        cycles: 0,
        r_pc: test.initial.pc,
        r_a: test.initial.a,
        r_x: test.initial.x,
        r_y: test.initial.y,
        r_sp: test.initial.s,
        p: test.initial.p,
        f_c: false,
        f_d: false,
        f_i: false,
        f_n: false,
        f_v: false,
        f_z: false,
    };

    emulator.load_state(initial);

    for (addr, value) in test.initial.ram {
        emulator.write(addr, value);
    }

    // Run instruction
    emulator.tick_cpu();

    // Test resulting state
    let result = emulator.get_state();

    if result.r_pc != test.fin.pc {
        println!("pc -> result: {:#06x} != expected: {:#06x}", result.r_pc, test.fin.pc);
        return false;
    } 
    if result.r_a != test.fin.a {
        println!("a -> result: {:#04x} != expected: {:#04x}", result.r_a, test.fin.a);
        return false;
    } 
    if result.r_x != test.fin.x {
        println!("x -> result: {:#04x} != expected: {:#04x}", result.r_x, test.fin.x);
        return false;
    } 
    if result.r_y != test.fin.y {
        println!("y -> result: {} != expected: {}", result.r_y, test.fin.y);
        return false;
    } 
    if result.r_sp != test.fin.s {
        println!("sp -> result: {:#04x} != expected: {:#04x}", result.r_sp, test.fin.s);
        return false;
    } 
    if result.p != test.fin.p {
        println!("p -> result: {:#04x} != expected: {:#04x}", result.p, test.fin.p);
        return false;
    } 

    for (addr, expected) in test.fin.ram {
        let res = emulator.read(addr);

        if res != expected {
            println!("mem not eq @ {addr} -> result: {res} != expected: {expected}");
            return false;
        }
    }

    true
}

fn test_valid_opcode(name: &str) -> bool {
    let split: Vec<&str> = name.split(' ').collect();

    let opcode = u8::from_str_radix(split[0], 16).unwrap();

    let binding = crate::emulator::cpu::opcode::Opcode::get_opcode_map();
    let found = binding.get(&opcode);
    found.is_some()
}

pub fn run_json_tests(emulator: &mut Emulator) {
    use std::fs;
    emulator.set_debug_flag(emulator::DebugFlag::Json);

    let dir = fs::read_dir("./json_tests").unwrap();
    for entry in dir {
        let path = entry.unwrap().path();
        let file = fs::read_to_string(path).unwrap();
        let tests: Vec<JsonData> = serde_json::from_str(&file).unwrap();

        if !test_valid_opcode(&tests[0].name) {
            continue;
        }

        let total_tests = tests.len();
        let mut current_test = 1;
        let mut passed_tests = 1;

        println!();
        for test in tests {
            print!("\r{} running test {}/{}, {} passed ", test.name, current_test, total_tests, passed_tests);
            if run_single_test(emulator, test) {
                passed_tests += 1;
            } 
            current_test += 1;
        }
    }

}