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

enum ParamMode {
    POSITION,
    IMMEDIATE
}

impl ParamMode {
    fn read(instruction: i64, param_num: usize) -> ParamMode {
        let digit_base = 10i64.pow(param_num as usize + 1);
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
            ADD => 3,
            MUL => 3,
            INPUT => 1,
            OUTPUT => 1,
            TERMINATE => 0,
        }
    }

    fn get_param_type(&self, param_num: usize) -> ParamType {
        return match self {
            ADD => {
                return match param_num {
                    1 => ParamType::READ,
                    2 => ParamType::READ,
                    3 => ParamType::WRITE, 
                    _ => panic!("ADD does not have a parameter {}", param_num)
                }
            },
            MUL => {
                return match param_num {
                    1 => ParamType::READ,
                    2 => ParamType::READ,
                    3 => ParamType::WRITE,
                    _ => panic!("MUL does not have a parameter {}", param_num)
                }
            },
            INPUT => {
                return match param_num {
                    1 => ParamType::WRITE,
                    _ => panic!("INPUT does not have a parameter {}", param_num)
                }
            },
            OUTPUT => {
                return match param_num {
                    1 => ParamType::READ,
                    _ => panic!("OUTPUT does not have a parameter {}", param_num)
                }
            },
            TERMINATE => {
                panic!("TERMINATE does not have parameters!");
            }
        }
    }
}

fn read_param(memory: &Vec<i64>, instruction_pointer: usize, param_num: usize) -> i64 {
    let param_pointer = instruction_pointer + param_num;
    if param_pointer >= memory.len() {
        panic!("Cannot read parameter {} for instruction {} at {}. Out of bounds.", param_num, memory[instruction_pointer], instruction_pointer);
    }
    let mode = ParamMode::read(memory[instruction_pointer], param_num);
    return match mode {
        POSITION => {
            let address = memory[param_pointer];
            if address >= memory.len() {
                panic!("Cannot read address pointed to by parameter: {}. Out of bounds.", address);
            }
            return memory[address as usize];
        }
        IMMEDIATE => {
            return memory[param_pointer];
        }
    }
}

fn write_param(memory: &mut Vec<i64>, instruction_pointer: usize, param_num: usize, value: i64) {
    let param_pointer = instruction_pointer + param_num;
    if param_pointer >= memory.len() {
        panic!("Cannot read parameter {} for instruction {} at {}. Out of bounds.", param_num, memory[instruction_pointer], instruction_pointer);
    }
    let mode = ParamMode::read(memory[instruction_pointer], param_num);
    match mode {
        POSITION => {
            let address = memory[param_pointer];
            if address >= memory.len() {
                panic!("Cannot write to address pointed to by parameter: {}. Out of bounds.", address);
            }
            memory[address] = value;
        },
        IMMEDIATE => {
            memory[param_pointer] = value;
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
            ADD => {
                write_param(&mut memory, ip, 3, read_param(&memory, ip, 1) + read_param(&memory, ip, 2));
            },
            MUL => {
                write_param(&mut memory, ip, 3, read_param(&memory, ip, 1) * read_param(&memory, ip, 2));
            },
            INPUT => {
                write_param(&mut memory, ip, 1, input_source.read());
            },
            OUTPUT => {
                output_sink.write(read_param(&memory, ip, 1));
            }
            TERMINATE => break,
        }
    }
}

fn main() {
    let program = read_program("../input");
    let mut input = VecDeque::from_iter(&[1]);
    //let mut output: Vec<i64> = Vec::new();
    let mut output = ConsoleOutputSink {};
    run_vm(&program, &mut input, &mut output);
    //println!("{:?}", output);
}