use serde::{Deserialize, Serialize};

use crate::emulator::{self, CpuState, Emulator};

// cpu state to be read from json test file
#[derive(Serialize, Deserialize)]
struct State {
    pc: u16,
    s: u8,
    a: u8,
    x: u8,
    y: u8,
    p: u8,
    ram: Vec<(u16, u8)>,
}

// model of singe test
#[derive(Serialize, Deserialize)]
struct Test {
    name: String,
    initial: State,
    #[serde(rename = "final")]
    fin: State,
}

fn run_single_test(emulator: &mut Emulator, test: Test) -> bool {
    // set cpu state to inital value
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

    // load values to specified addreses
    for (addr, value) in test.initial.ram {
        emulator.write(addr, value);
    }

    // run loaded instruction
    emulator.tick_cpu();

    // get resulting state from cpu
    let result = emulator.get_state();

    // test each register and flag to expected state
    if result.r_pc != test.fin.pc {
        println!(
            "pc -> result: {:#06x} != expected: {:#06x}",
            result.r_pc, test.fin.pc
        );
        return false;
    }
    if result.r_a != test.fin.a {
        println!(
            "a -> result: {:#04x} != expected: {:#04x}",
            result.r_a, test.fin.a
        );
        return false;
    }
    if result.r_x != test.fin.x {
        println!(
            "x -> result: {:#04x} != expected: {:#04x}",
            result.r_x, test.fin.x
        );
        return false;
    }
    if result.r_y != test.fin.y {
        println!("y -> result: {} != expected: {}", result.r_y, test.fin.y);
        return false;
    }
    if result.r_sp != test.fin.s {
        println!(
            "sp -> result: {:#04x} != expected: {:#04x}",
            result.r_sp, test.fin.s
        );
        return false;
    }
    if result.p != test.fin.p {
        println!(
            "p -> result: {:#04x} != expected: {:#04x}",
            result.p, test.fin.p
        );
        return false;
    }

    // compare expected memory adresses
    for (addr, expected) in test.fin.ram {
        let res = emulator.read(addr);

        if res != expected {
            println!("mem not eq @ {addr} -> result: {res} != expected: {expected}");
            return false;
        }
    }

    true
}

// check if opcode is in opcode map, because json tests also test
// unoffical opcodes that this emulator doesn't implement
fn test_valid_opcode(name: &str) -> bool {
    let split: Vec<&str> = name.split(' ').collect();

    let opcode = u8::from_str_radix(split[0], 16).unwrap();

    let binding = crate::emulator::cpu::opcode::Opcode::get_opcode_map();
    let found = binding.get(&opcode);
    found.is_some()
}

// main test loop
pub fn run_json_tests(emulator: &mut Emulator) {
    use std::fs;

    // set json test mode, so the emulator uses a full 64kb of
    // readable and writable ram instead of memory normal
    // nes cpu memory map
    let debug_ctx = emulator.debug_ctx();
    debug_ctx
        .borrow_mut()
        .set_debug_flag(emulator::debug::DebugFlag::Json);

    // iterate through all files in test directory
    let dir = fs::read_dir("./json_tests").unwrap();
    for entry in dir {
        let path = entry.unwrap().path();
        let file = fs::read_to_string(path).unwrap();
        // serde parses 10k tests from file into vec
        let tests: Vec<Test> = serde_json::from_str(&file).unwrap();

        // skip file if its testing for unoffical opcode
        if !test_valid_opcode(&tests[0].name) {
            continue;
        }

        let total_tests = tests.len();
        let mut current_test = 1;
        let mut passed_tests = 1;

        // run tets and show how many pass
        println!();
        for test in tests {
            print!(
                "\r{} running test {}/{}, {} passed ",
                test.name, current_test, total_tests, passed_tests
            );
            if run_single_test(emulator, test) {
                passed_tests += 1;
            }
            current_test += 1;
        }
    }
}
