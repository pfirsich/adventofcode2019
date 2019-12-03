use std::fs;

fn execute_add(instruction_pointer: usize, memory: &mut Vec<u64>) {
    assert!(memory[instruction_pointer] == 1);
    let param1 = memory[instruction_pointer + 1] as usize;
    let param2 = memory[instruction_pointer + 2] as usize;
    let dest = memory[instruction_pointer + 3] as usize;
    memory[dest] = memory[param1] + memory[param2];
}

fn execute_mul(instruction_pointer: usize, memory: &mut Vec<u64>) {
    assert!(memory[instruction_pointer] == 2);
    let param1 = memory[instruction_pointer + 1] as usize;
    let param2 = memory[instruction_pointer + 2] as usize;
    let dest = memory[instruction_pointer + 3] as usize;
    memory[dest] = memory[param1] * memory[param2];
}

fn str_to_u64(s: &str) -> u64 {
    return s.trim().parse::<u64>().unwrap();
}

fn run_program(init_memory: &Vec<u64>, noun: u64, verb: u64) -> u64 {
    let mut memory = init_memory.clone();
    let mut instruction_pointer: usize = 0;
    memory[1] = noun;
    memory[2] = verb;
    loop {
        match memory[instruction_pointer] {
            1 => {
                execute_add(instruction_pointer, &mut memory);
                instruction_pointer += 4;
            },
            2 => {
                execute_mul(instruction_pointer, &mut memory);
                instruction_pointer += 4;
            },
            99 => break,
            _ => panic!("Unknown opcode: {}", memory[instruction_pointer])
        }
    }
    return memory[0];
}

fn main() {
    let program_string = fs::read_to_string("../input").unwrap();
    let memory = program_string.split(",").map(str_to_u64).collect::<Vec<u64>>();
    run_program(&memory, 12, 2);
    println!("Computation result: {}", run_program(&memory, 12, 2));
    for noun in 0..100 {
        for verb in 0..100 {
            if run_program(&memory, noun, verb) == 19690720 {
                println!("Noun = {}, verb = {}", noun, verb);
                return;
            }
        }
    }
}
