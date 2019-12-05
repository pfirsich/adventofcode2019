use std::fs;
use std::collections::VecDeque;

trait InputSource {
    fn read(&mut self) -> i64;
}

trait OutputSink {
    fn write(&mut self, value: i64);
}

impl InputSource for VecDeque<i64> {
    fn read(&mut self) -> i64 {
        if self.len() == 0 {
            panic!("InputSource VecDeque is empty!");
        }
        return self.pop_front().unwrap();
    }
}

impl OutputSink for Vec<i64> {
    fn write(&mut self, value: i64) {
        self.push(value);
    }
}

struct ConsoleOutputSink {
}

impl OutputSink for ConsoleOutputSink {
    fn write(&mut self, value: i64) {
        println!("{}", value);
    }
}

fn read_program(filename: &str) -> Vec<i64> {
    fn parse_int(s: &str) -> i64 {
        return s.trim().parse::<i64>().unwrap();
    }

    let program_str = fs::read_to_string(&filename).unwrap();
    return program_str.split(",").map(parse_int).collect::<Vec<i64>>();
}

#[derive(PartialEq)]
enum ParamMode {
    POSITION,
    IMMEDIATE
}

impl ParamMode {
    fn read(instruction: i64, param_num: usize) -> ParamMode {
        let digit_base = 10i64.pow(param_num as u32 + 1);
        return match (instruction / digit_base) % 10 {
            0 => ParamMode::POSITION,
            1 => ParamMode::IMMEDIATE,
            _ => panic!("Unrecognized parameter mode digit")
        }
    }
}

enum OpCode {
    ADD = 1,
    MUL = 2,
    INPUT = 3,
    OUTPUT = 4,
    TERMINATE = 99,
}

#[derive(PartialEq)]
enum ParamType {
    READ,
    WRITE
}

impl OpCode {
    fn read(instruction: i64) -> OpCode {
        return match instruction % 100 {
            1 => OpCode::ADD,
            2 => OpCode::MUL,
            3 => OpCode::INPUT,
            4 => OpCode::OUTPUT,
            99 => OpCode::TERMINATE,
            _ => panic!("Unknown opcode: {}", instruction)
        }
    }

    fn get_param_count(&self) -> usize {
        match self {
            OpCode::ADD => 3,
            OpCode::MUL => 3,
            OpCode::INPUT => 1,
            OpCode::OUTPUT => 1,
            OpCode::TERMINATE => 0,
        }
    }

    fn get_param_type(&self, param_num: usize) -> ParamType {
        return match self {
            OpCode::ADD => {
                match param_num {
                    1 => ParamType::READ,
                    2 => ParamType::READ,
                    3 => ParamType::WRITE, 
                    _ => panic!("ADD does not have a parameter {}", param_num)
                }
            },
            OpCode::MUL => {
                match param_num {
                    1 => ParamType::READ,
                    2 => ParamType::READ,
                    3 => ParamType::WRITE,
                    _ => panic!("MUL does not have a parameter {}", param_num)
                }
            },
            OpCode::INPUT => {
                match param_num {
                    1 => ParamType::WRITE,
                    _ => panic!("INPUT does not have a parameter {}", param_num)
                }
            },
            OpCode::OUTPUT => {
                match param_num {
                    1 => ParamType::READ,
                    _ => panic!("OUTPUT does not have a parameter {}", param_num)
                }
            },
            OpCode::TERMINATE => {
                panic!("TERMINATE does not have parameters!");
            }
        }
    }
}

fn get_param_address(memory: &Vec<i64>, instruction_pointer: usize, param_num: usize) -> usize {
    let param_pointer = instruction_pointer + param_num;
    if param_pointer >= memory.len() {
        panic!("Cannot read parameter {} for instruction {} at {}. Out of bounds.", param_num, memory[instruction_pointer], instruction_pointer);
    }
    let mode = ParamMode::read(memory[instruction_pointer], param_num);
    match mode {
        ParamMode::POSITION => {
            let address = memory[param_pointer];
            if address < 0 || address as usize > memory.len() {
                panic!("Cannot read address pointed to by parameter: {}. Out of bounds.", address);
            }
            return address as usize;
        }
        ParamMode::IMMEDIATE => {
            return param_pointer;
        }
    }
}

fn run_vm<I: InputSource, O: OutputSink>(program: &Vec<i64>, input_source: &mut I, output_sink: &mut O) {
    let mut memory = program.clone();
    let mut ip: usize = 0; // instruction pointer
    while ip < memory.len() {
        let instruction = memory[ip];
        let opcode = OpCode::read(instruction);
        for i in 0..opcode.get_param_count() {
            let param_type = opcode.get_param_type(i + 1);
            let param_mode = ParamMode::read(instruction, i + 1);
            if param_type == ParamType::WRITE && param_mode == ParamMode::IMMEDIATE {
                panic!("Write parameter must not be in immediate mode! Instruction: {}", instruction);
            }
        }
        match opcode {
            OpCode::ADD => {
                memory[get_param_address(&mut memory, ip, 3)] =
                    memory[get_param_address(&memory, ip, 1)] 
                    + memory[get_param_address(&memory, ip, 2)];
            },
            OpCode::MUL => {
                memory[get_param_address(&mut memory, ip, 3)] =
                    memory[get_param_address(&memory, ip, 1)] 
                    * memory[get_param_address(&memory, ip, 2)] 
            },
            OpCode::INPUT => {
                memory[get_param_address(&mut memory, ip, 1)] = input_source.read();
            },
            OpCode::OUTPUT => {
                output_sink.write(memory[get_param_address(&memory, ip, 1)]);
            }
            OpCode::TERMINATE => break,
        }
        ip += opcode.get_param_count() + 1;
    }
}

fn main() {
    let program = read_program("../input");
    let mut input: VecDeque<i64> = VecDeque::from(vec![1]);
    //let mut output: Vec<i64> = Vec::new();
    let mut output = ConsoleOutputSink {};
    run_vm(&program, &mut input, &mut output);
    //println!("{:?}", output);
}
