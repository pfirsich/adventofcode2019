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

#[derive(PartialEq)]
enum ParamMode {
    Position,
    Immediate
}

impl ParamMode {
    fn read(instruction: i64, param_num: usize) -> ParamMode {
        let digit_base = 10i64.pow(param_num as u32 + 1);
        return match (instruction / digit_base) % 10 {
            0 => ParamMode::Position,
            1 => ParamMode::Immediate,
            _ => panic!("Unrecognized parameter mode digit")
        }
    }
}

enum OpCode {
    Add,
    Mul,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Terminate
}

#[derive(PartialEq)]
enum ParamType {
    Read,
    Write
}

struct Operation {
    op_code: OpCode,
    parameters: Vec<ParamType>,
}

impl Operation {
    fn read(instruction: i64) -> Operation {
        // I would make these guys static, but I cannot have a vec in a static, so I
        // allocate and copy a bunch instead :)
        return match instruction % 100 {
            1 => Operation { op_code: OpCode::Add,
                             parameters: vec![ParamType::Read, ParamType::Read, ParamType::Write] },
            2 => Operation { op_code: OpCode::Mul,
                             parameters: vec![ParamType::Read, ParamType::Read, ParamType::Write] },
            3 => Operation { op_code: OpCode::Input, parameters: vec![ParamType::Write] },
            4 => Operation { op_code: OpCode::Output, parameters: vec![ParamType::Read] },
            5 => Operation { op_code: OpCode::JumpIfTrue, 
                             parameters: vec![ParamType::Read, ParamType::Read] },
            6 => Operation { op_code: OpCode::JumpIfFalse, 
                             parameters: vec![ParamType::Read, ParamType::Read] },
            7 => Operation { op_code: OpCode::LessThan,
                             parameters: vec![ParamType::Read, ParamType::Read, ParamType::Write] },
            8 => Operation { op_code: OpCode::Equals,
                             parameters: vec![ParamType::Read, ParamType::Read, ParamType::Write] },
            99 => Operation { op_code: OpCode::Terminate, parameters: vec![] },
            _ => panic!("Unknown opcode: {}", instruction)
        }
    }
    
    fn get_param_address(&self, memory: &Vec<i64>, ip: usize, param_num: usize) -> usize {
        let param_pointer = ip + param_num;
        if param_pointer >= memory.len() {
            panic!("Cannot read parameter {} for instruction {} at {}. Out of bounds.", param_num, memory[ip], ip);
        }
        let mode = ParamMode::read(memory[ip], param_num);
        match mode {
            ParamMode::Position => {
                let address = memory[param_pointer];
                if address < 0 || address as usize > memory.len() {
                    panic!("Cannot read address pointed to by parameter: {}. Out of bounds.", address);
                }
                return address as usize;
            }
            ParamMode::Immediate => {
                if self.parameters[param_num - 1] == ParamType::Write {
                    panic!("Write parameter {} must not be in immediate mode for instruction: {}", param_num, memory[ip]);
                }
                return param_pointer;
            }
        }
    }

    fn execute<I: InputSource, O: OutputSink>(&self, memory: &mut Vec<i64>, ip: usize, input_source: &mut I, output_sink: &mut O) -> Option<usize> {
        let param = |param_num: usize| self.get_param_address(&memory, ip, param_num);
        let validate_addr = |value: i64| {
            if value < 0 {
                panic!("Cannot jump to negative address");
            }
            return value as usize;
        };
        match self.op_code {
            OpCode::Add => {
                let addr = param(3);
                memory[addr] = memory[param(1)] + memory[param(2)];
            },
            OpCode::Mul => {
                let addr = param(3);
                memory[addr] = memory[param(1)] * memory[param(2)];
            },
            OpCode::Input => {
                let addr = param(1);
                memory[addr] = input_source.read();
            },
            OpCode::Output => {
                output_sink.write(memory[param(1)]);
            },
            OpCode::JumpIfTrue => {
                let addr = param(1); 
                if memory[addr] != 0 {
                    return Some(validate_addr(memory[param(2)]));
                }
            },
            OpCode::JumpIfFalse => {
                let addr = param(1);
                if memory[addr] == 0 {
                    return Some(validate_addr(memory[param(2)]));
                }
            },
            OpCode::LessThan => {
                let addr = param(3);
                memory[addr] = if memory[param(1)] < memory[param(2)] { 1 } else { 0 }
            }
            OpCode::Equals => {
                let addr = param(3);
                memory[addr] = if memory[param(1)] == memory[param(2)] { 1 } else { 0 }
            }
            OpCode::Terminate => return None,
        }
        return Some(ip + 1 + self.parameters.len());
    }
}

fn read_program(filename: &str) -> Vec<i64> {
    fn parse_int(s: &str) -> i64 {
        return s.trim().parse::<i64>().unwrap();
    }

    let program_str = fs::read_to_string(&filename).unwrap();
    return program_str.split(",").map(parse_int).collect::<Vec<i64>>();
}

fn run_vm<I: InputSource, O: OutputSink>(program: &Vec<i64>, input_source: &mut I, output_sink: &mut O) {
    let mut memory = program.clone();
    let mut ip: usize = 0; // instruction pointer
    while ip < memory.len() {
        let instruction = memory[ip];
        let operation = Operation::read(instruction);
        let new_ip = operation.execute(&mut memory, ip, input_source, output_sink);
        match new_ip {
            Some(v) => ip = v,
            None => break
        }
    }
}

fn main() {
    let program = read_program("../input");
    let mut input: VecDeque<i64> = VecDeque::from(vec![5]);
    let mut output = ConsoleOutputSink {};
    run_vm(&program, &mut input, &mut output);
}
