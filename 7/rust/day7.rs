use std::fs;
use std::collections::VecDeque;

trait InputSource {
    fn read(&mut self) -> i64;
    fn len(&self) -> usize;
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

    fn len(&self) -> usize {
        return self.len();
    }
}

impl OutputSink for VecDeque<i64> {
    fn write(&mut self, value: i64) {
        self.push_back(value);
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

#[derive(PartialEq)]
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

#[derive(Copy, Clone, PartialEq)]
enum VmState {
    NotStarted,
    Running,
    WaitForInput,
    Terminated,
}

struct Vm<I: InputSource, O: OutputSink> {
    memory: Vec<i64>,
    instruction_pointer: usize,
    input_source: I,
    output_sink: O,
    state: VmState,
}

impl<I: InputSource + Default, O: OutputSink + Default> Vm<I, O> {
    fn new(program: Vec<i64>) -> Vm<I, O> {
        return Vm {
            memory: program,
            instruction_pointer: 0,
            input_source: I::default(),
            output_sink: O::default(),
            state: VmState::NotStarted,
        };
    }

    fn step(&mut self) -> VmState {
        self.state = VmState::Running;
        let operation = Operation::read(self.memory[self.instruction_pointer]);
        if operation.op_code == OpCode::Input && self.input_source.len() == 0 {
            self.state = VmState::WaitForInput;
            return self.state;
        }
        let new_ip = operation.execute(&mut self.memory, self.instruction_pointer, &mut self.input_source, &mut self.output_sink);
        match new_ip {
            Some(v) => self.instruction_pointer = v,
            None => self.state = VmState::Terminated,
        }
        return self.state;
    }

    fn run(&mut self) -> VmState {
        while self.instruction_pointer < self.memory.len() {
            match self.step() {
                VmState::NotStarted => panic!("Invalid state after step()"),
                VmState::Running => (), // keep going
                VmState::WaitForInput => break, // suspend
                VmState::Terminated => break // done
            }
        }
        return self.state;
    }
}

fn test_amp_circuit(program: &Vec<i64>, phase_setting: &Vec<i64>) -> i64 {
    let mut input = 0;
    for i in 0..5 {
        let mut vm: Vm<VecDeque<i64>, VecDeque<i64>> = Vm::new(program.clone());
        vm.input_source.push_back(phase_setting[i]);
        vm.input_source.push_back(input);
        vm.run();
        assert!(vm.output_sink.len() == 1);
        input = vm.output_sink[0];
    }
    return input;
}

fn test_amp_feedback_circuit(program: &Vec<i64>, phase_setting: &Vec<i64>) -> i64 {
    let mut amps: Vec<Vm<VecDeque<i64>, VecDeque<i64>>> = Vec::new();
    const AMP_COUNT: usize = 5;
    for i in 0..AMP_COUNT {
        amps.push(Vm::new(program.clone()));
        amps[i].input_source.push_back(phase_setting[i]);
    }

    let mut input = 0;
    loop {
        for i in 0..AMP_COUNT {
            assert!(amps[i].state != VmState::Terminated);
            amps[i].input_source.push_back(input);
            amps[i].run();
            assert!(amps[i].output_sink.len() == 1);
            input = amps[i].output_sink.pop_front().unwrap();
        }
        if amps[AMP_COUNT-1].state == VmState::Terminated {
            return input;
        }
    }
}

fn optimize_phase_setting(program: &Vec<i64>, init_phase_setting: &Vec<i64>, system: fn(&Vec<i64>, &Vec<i64>) -> i64) -> (i64, Vec<i64>) {
    let mut phase_setting = init_phase_setting.clone();
    let mut max_output = 0;
    let mut max_phase_setting = phase_setting.clone();
    loop {
        let output = system(program, &phase_setting);
        if output > max_output {
            max_output = output;
            max_phase_setting = phase_setting.clone();
        }
        match next_permutation(&phase_setting) {
            Some(next) => phase_setting = next,
            None => break
        }
    }
    return (max_output, max_phase_setting);
}

// Returns permutation that is greater than the input (as little as possible)
fn next_permutation<T: PartialOrd + Copy + std::fmt::Debug>(input: &Vec<T>) -> Option<Vec<T>> {
    // find longest weakly decreasing suffix
    let mut suf = input.len() - 1; // points to first element of the suffix
    while suf > 0 && input[suf-1] >= input[suf] {
        suf -= 1;
    }

    // If the whole Vec is decreasing, it is already maximal and there are no further permutations
    if suf <= 0 {
        return None;
    }

    let pivot = suf - 1;
    // Find smallest (rightmost) element in input that's greater than pivot (so swapping makes it bigger)
    let mut swapi = input.len() - 1;
    while input[swapi] <= input[pivot] {
        swapi -= 1;
    }

    let mut out = input.clone();
    out.swap(pivot, swapi);
    // Now the suffix is still decreasing, if we reverse it, our permutation is smaller
    let suf_len = input.len() - suf;
    for i in 0..suf_len/2 {
        out.swap(suf + i, input.len() - 1 - i);
    }
    return Some(out);
}

fn main() {
    let program = read_program("../input");
    
    let init_phase_setting: Vec<i64> = vec![0, 1, 2, 3, 4];
    let (max_output, max_phase_setting) = optimize_phase_setting(&program, &init_phase_setting, test_amp_circuit);
    println!("Max output: {}. Phase setting: {:?}", max_output, max_phase_setting);

    let init_fb_phase_setting: Vec<i64> = vec![5, 6, 7, 8, 9];
    test_amp_feedback_circuit(&program, &init_fb_phase_setting);
    let (max_fb_output, max_fb_phase_setting) = optimize_phase_setting(&program, &init_fb_phase_setting, test_amp_feedback_circuit);
    println!("Max feedback system output: {}, Phase setting: {:?}", max_fb_output, max_fb_phase_setting);
}
