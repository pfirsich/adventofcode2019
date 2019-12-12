use std::fs;
use std::collections::VecDeque;
use std::collections::HashMap;

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

impl Default for ConsoleOutputSink {
    fn default() -> Self {
        return ConsoleOutputSink {};
    }
}

struct InfiniteTape {
    data: Vec<i64>,
}

impl InfiniteTape {
    fn set(&mut self, index: usize, value: i64) {
        if index >= self.data.len() {
            self.data.resize(index + 1, 0);
        }
        self.data[index] = value;
    }

    fn get(&self, index: usize) -> i64 {
        if index >= self.data.len() {
            return 0;
        } else {
            return self.data[index];
        }
    }
}

#[derive(PartialEq)]
enum ParamMode {
    Position,
    Immediate,
    Relative,
}

impl ParamMode {
    fn read(instruction: i64, param_num: usize) -> ParamMode {
        let digit_base = 10i64.pow(param_num as u32 + 1);
        return match (instruction / digit_base) % 10 {
            0 => ParamMode::Position,
            1 => ParamMode::Immediate,
            2 => ParamMode::Relative,
            _ => panic!("Unrecognized parameter mode digit")
        }
    }
}

#[derive(PartialEq, Debug)]
enum OpCode {
    Add,
    Mul,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    AdjustRelativeBase,
    Terminate,
}

#[derive(PartialEq)]
enum ParamType {
    Read,
    Write
}

impl OpCode {
    fn read(instruction: i64) -> OpCode {
        // I would make these guys static, but I cannot have a vec in a static, so I
        // allocate and copy a bunch instead :)
        return match instruction % 100 {
            1 => OpCode::Add,
            2 => OpCode::Mul,
            3 => OpCode::Input,
            4 => OpCode::Output,
            5 => OpCode::JumpIfTrue,
            6 => OpCode::JumpIfFalse,
            7 => OpCode::LessThan,
            8 => OpCode::Equals,
            9 => OpCode::AdjustRelativeBase,
            99 => OpCode::Terminate,
            _ => panic!("Unknown opcode: {}", instruction)
        }
    }

    fn get_param_count(&self) -> usize {
        return match self {
            OpCode::Add => 3,
            OpCode::Mul => 3,
            OpCode::Input => 1,
            OpCode::Output => 1,
            OpCode::JumpIfTrue => 2,
            OpCode::JumpIfFalse => 2,
            OpCode::LessThan => 3,
            OpCode::Equals => 3,
            OpCode::AdjustRelativeBase => 1,
            OpCode::Terminate => 0,
        }
    }

    fn get_param_type(&self, param_num: usize) -> ParamType {
        return match self {
            OpCode::Add => match param_num {
                1 | 2 => ParamType::Read,
                3 => ParamType::Write,
                _ => panic!("Invalid param number {} for op code {:?}!", param_num, self)
            },
            OpCode::Mul => match param_num {
                1 | 2 => ParamType::Read,
                3 => ParamType::Write,
                _ => panic!("Invalid param number {} for op code {:?}!", param_num, self)
            },
            OpCode::Input => match param_num {
                1 => ParamType::Write,
                _ => panic!("Invalid param number {} for op code {:?}!", param_num, self)
            },
            OpCode::Output => match param_num {
                1 => ParamType::Read,
                _ => panic!("Invalid param number {} for op code {:?}!", param_num, self)
            },
            OpCode::JumpIfTrue => match param_num {
                1 | 2 => ParamType::Read,
                _ => panic!("Invalid param number {} for op code {:?}!", param_num, self)
            },
            OpCode::JumpIfFalse => match param_num {
                1 | 2 => ParamType::Read,
                _ => panic!("Invalid param number {} for op code {:?}!", param_num, self)
            },
            OpCode::LessThan => match param_num {
                1 | 2 => ParamType::Read,
                3 => ParamType::Write,
                _ => panic!("Invalid param number {} for op code {:?}!", param_num, self)
            },
            OpCode::Equals => match param_num {
                1 | 2 => ParamType::Read,
                3 => ParamType::Write,
                _ => panic!("Invalid param number {} for op code {:?}!", param_num, self)
            },
            OpCode::AdjustRelativeBase => match param_num {
                1 => ParamType::Read,
                _ => panic!("Invalid param number {} for op code {:?}!", param_num, self)
            },
            OpCode::Terminate => match param_num {
                _ => panic!("Invalid param number {} for op code {:?}!", param_num, self)
            }
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
enum VmState {
    NotStarted,
    Running,
    WaitForInput,
    Terminated,
}

struct Vm<I: InputSource, O: OutputSink> {
    memory: InfiniteTape,
    instruction_pointer: usize,
    input_source: I,
    output_sink: O,
    state: VmState,
    relative_base: usize,
}

impl<I: InputSource + Default, O: OutputSink + Default> Vm<I, O> {
    fn new(program: Vec<i64>) -> Vm<I, O> {
        return Vm {
            memory: InfiniteTape { data: program },
            instruction_pointer: 0,
            input_source: I::default(),
            output_sink: O::default(),
            state: VmState::NotStarted,
            relative_base: 0,
        };
    }

    fn get_param_address(&self, op_code: &OpCode, param_num: usize) -> usize {
        let ip = self.instruction_pointer;
        let param_pointer = ip + param_num;
        let mode = ParamMode::read(self.memory.get(ip), param_num);
        match mode {
            ParamMode::Position => {
                let address = self.memory.get(param_pointer);
                if address < 0 {
                    panic!("Invalid address: {}", address);
                }
                return address as usize;
            }
            ParamMode::Immediate => {
                if op_code.get_param_type(param_num) == ParamType::Write {
                    panic!("Write parameter {} must not be in immediate mode for instruction: {}", param_num, self.memory.get(ip));
                }
                return param_pointer;
            }
            ParamMode::Relative => {
                let address = self.memory.get(param_pointer) + self.relative_base as i64;
                if address < 0 {
                    panic!("Invalid address: {}", address);
                }
                return address as usize;
            }
        }
    }

    fn execute_operation(&mut self, op_code: &OpCode) -> Option<usize> {
        let get_param = |param_num: usize| self.memory.get(self.get_param_address(op_code, param_num));
        let validate_addr = |value: i64| {
            if value < 0 {
                panic!("Cannot jump to negative address");
            }
            return value as usize;
        };
        match op_code {
            OpCode::Add => {
                let addr = self.get_param_address(op_code, 3);
                self.memory.set(addr, get_param(1) + get_param(2));
            },
            OpCode::Mul => {
                let addr = self.get_param_address(op_code, 3);
                self.memory.set(addr, get_param(1) * get_param(2));
            },
            OpCode::Input => {
                let addr = self.get_param_address(op_code, 1);
                self.memory.set(addr, self.input_source.read());
            },
            OpCode::Output => {
                self.output_sink.write(get_param(1));
            },
            OpCode::JumpIfTrue => {
                let addr = self.get_param_address(op_code, 1); 
                if self.memory.get(addr) != 0 {
                    return Some(validate_addr(get_param(2)));
                }
            },
            OpCode::JumpIfFalse => {
                let addr = self.get_param_address(op_code, 1);
                if self.memory.get(addr) == 0 {
                    return Some(validate_addr(get_param(2)));
                }
            },
            OpCode::LessThan => {
                let addr = self.get_param_address(op_code, 3);
                self.memory.set(addr, if get_param(1) < get_param(2) { 1 } else { 0 })
            },
            OpCode::Equals => {
                let addr = self.get_param_address(op_code, 3);
                self.memory.set(addr, if get_param(1) == get_param(2) { 1 } else { 0 })
            },
            OpCode::AdjustRelativeBase => {
                let new_base = self.relative_base as i64 + get_param(1);
                if new_base < 0 {
                    panic!("Invalid new relative base: {}", new_base);
                }
                self.relative_base = new_base as usize;
            }
            OpCode::Terminate => return None,
        }
        return Some(self.instruction_pointer + 1 + op_code.get_param_count());
    }

    fn step(&mut self) -> VmState {
        self.state = VmState::Running;
        let op_code = OpCode::read(self.memory.get(self.instruction_pointer));
        if op_code == OpCode::Input && self.input_source.len() == 0 {
            self.state = VmState::WaitForInput;
            return self.state;
        }
        let new_ip = self.execute_operation(&op_code);
        match new_ip {
            Some(v) => self.instruction_pointer = v,
            None => self.state = VmState::Terminated,
        }
        return self.state;
    }

    fn run(&mut self) -> VmState {
        loop {
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

fn read_program(filename: &str) -> Vec<i64> {
    fn parse_int(s: &str) -> i64 {
        return s.trim().parse::<i64>().unwrap();
    }

    let program_str = fs::read_to_string(&filename).unwrap();
    return program_str.split(",").map(parse_int).collect::<Vec<i64>>();
}

#[derive(Clone, Copy)]
struct Position {
    x: i64,
    y: i64
}

struct Panel {
    position: Position,
    color: i64,
}

enum Direction {
    Up, Down, Left, Right
}

fn pos_hash(pos: &Position) -> i64 {
    return pos.x * 0x1000000 + pos.y;
}

fn hash_to_pos(hash: i64) -> (i64, i64) {
    let x = hash / 0x1000000;
    return (x, hash - x);
}

fn simulate_robot(program: &Vec<i64>, start_color: i64) -> Vec<Panel> {
    let mut panels: Vec<Panel> = Vec::new();
    let mut panel_map: HashMap<i64, usize> = HashMap::new(); // position hash -> panel index
    let mut cur = Position { x: 0, y: 0 };
    let mut dir = Direction::Up;
    let mut brain: Vm<VecDeque<i64>, VecDeque<i64>> = Vm::new(program.clone());
    panels.push(Panel { position: cur, color: start_color });
    panel_map.insert(pos_hash(&cur), 0);
    loop {
        let pos_hash = pos_hash(&cur);
        let mut panel = match panel_map.get(&pos_hash) {
            Some(idx) => &mut panels[*idx],
            None => {
                panels.push(Panel { position: cur, color: 0 });
                let idx = panels.len() - 1;
                panel_map.insert(pos_hash, idx);
                &mut panels[idx]
            }
        };
        brain.input_source.push_back(panel.color);
        match brain.run() {
            VmState::Terminated => break,
            _ => (), // keep going
        }

        assert!(brain.output_sink.len() == 2, "Output sink: {:?}", brain.output_sink);
        let new_color = brain.output_sink.pop_front().unwrap();
        let turn_dir = brain.output_sink.pop_front().unwrap();

        panel.color = new_color;
        let new_dir = match turn_dir {
            0 => match dir {
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Down,
                Direction::Right => Direction::Up,
            },
            1 => match dir {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
            }
            _ => panic!("Unexpected output: {}", turn_dir)
        };
        dir = new_dir;
        
        match dir {
            Direction::Up => cur.y += 1,
            Direction::Down => cur.y -= 1,
            Direction::Left => cur.x -= 1,
            Direction::Right => cur.x += 1,
        }
    }
    return panels;
}

fn main() {
    let program = read_program("../input");

    let panels = simulate_robot(&program, 1);
    println!("{} panels painted!", panels.len());

    let mut min = Position { x: 0, y: 0 };
    let mut max = Position { x: 0, y: 0 };
    for panel in &panels {
        min.x = std::cmp::min(panel.position.x, min.x);
        min.y = std::cmp::min(panel.position.y, min.y);
        max.x = std::cmp::max(panel.position.x, max.x);
        max.y = std::cmp::max(panel.position.y, max.y);
    }
    let range = Position {
        x: max.x - min.x + 1,
        y: max.y - min.y + 1
    };
    println!("min.x: {}, max.x: {} -> range.x: {}", min.x, max.x, range.x);
    println!("min.y: {}, max.y: {} -> range.y: {}", min.y, max.y, range.y);
    
    let mut ship: Vec<Vec<i64>> = Vec::new();
    ship.resize(range.y as usize, Vec::new());
    for y in 0..range.y {
        ship[y as usize].resize(range.x as usize, 0);
    }

    println!("Filling grid");
    for panel in &panels {
        let rx = (panel.position.x - min.x) as usize;
        let ry = (panel.position.y - min.y) as usize;
        ship[ry][rx] = panel.color;
    }

    println!("Output:");
    for y_ in 0..ship.len() {
        let y = ship.len() - 1 - y_;
        for x in 0..ship[y].len() {
            print!("{}", match ship[y][x] {
                0 => " ",
                1 => "X",
                _ => panic!("Unknown color: {}", ship[y][x])
            })
        }
        println!("");
    }
}
