use std::fs;
use std::collections::VecDeque;
use std::io;
use std::io::Read;
use std::thread;

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

struct Screen {
    tiles: Vec<Vec<i64>>,
    score: i64,
}

impl Screen {
    fn new() -> Screen {
        return Screen {
            tiles: Vec::new(),
            score: 0,
        }
    }

    fn update(&mut self, vm_output: &VecDeque<i64>, num_tiles: usize) {
        for i in 0..num_tiles {
            let x = vm_output[i*3+0];
            let y = vm_output[i*3+1];
            if x == -1 && y == 0 {
                self.score = vm_output[i*3+2];
            } else {
                assert!(x >= 0 && y >= 0);
                let ux = x as usize;
                let uy = y as usize;
                if uy >= self.tiles.len() {
                    self.tiles.resize(uy + 1, Vec::new());
                }
                if ux >= self.tiles[uy].len() {
                    self.tiles[uy].resize(ux + 1, 0);
                }
                self.tiles[uy][ux] = vm_output[i*3+2];
            }
        }
    }

    fn draw(&self) {
        for y in 0..self.tiles.len() {
            for x in 0..self.tiles[y].len() {
                print!("{}", match self.tiles[y][x] {
                    0 => " ",
                    1 => "#",
                    2 => "B",
                    3 => "-",
                    4 => "o",
                    _ => panic!("Unknown tile id: {}", self.tiles[y][x])
                });
            }
            println!("");
        }
        println!("Score: {}", self.score);
    }

    fn count(&self, tile: i64) -> usize {
        let mut count = 0;
        for y in 0..self.tiles.len() {
            for x in 0..self.tiles[y].len() {
                if self.tiles[y][x] == tile {
                    count += 1;
                }
            }
        }
        return count;
    }

    fn find(&self, tile: i64) -> Option<(usize, usize)> {
        for y in 0..self.tiles.len() {
            for x in 0..self.tiles[y].len() {
                if self.tiles[y][x] == tile {
                    return Some((x, y));
                }
            }
        }
        return None;
    }
}

enum JoystickInput {
    Neutral,
    Left,
    Right,
}

struct ArcadeCabinet {
    vm: Vm<VecDeque<i64>, VecDeque<i64>>,
    screen: Screen,
}

impl ArcadeCabinet {
    fn new(game_program: Vec<i64>, coins: i64) -> ArcadeCabinet {
        let mut cabinet = ArcadeCabinet {
            vm: Vm::new(game_program),
            screen: Screen::new(),
        };
        cabinet.vm.memory.set(0, coins);
        cabinet.vm.run();
        cabinet.update_screen();
        return cabinet;
    }

    fn update_screen(&mut self) {
        assert!(self.vm.output_sink.len() % 3 == 0);
        self.screen.update(&self.vm.output_sink, self.vm.output_sink.len() / 3);
        self.vm.output_sink.clear();
    }

    fn step(&mut self, joystick_input: JoystickInput) {
        self.vm.input_source.push_back(match joystick_input {
            JoystickInput::Neutral => 0,
            JoystickInput::Left => -1,
            JoystickInput::Right => 1,
        });
        self.vm.run();
        self.update_screen();
    }
}

// Obviously this was more complicated in the past
struct BreakoutAi {
}

impl BreakoutAi {
    fn new() -> BreakoutAi {
        return BreakoutAi { }
    }

    fn think(&mut self, screen: &Screen) -> JoystickInput {
        let (paddle_x, paddle_y) = screen.find(3).expect("Paddle not found");
        let (ball_x, ball_y) = screen.find(4).expect("Ball not found");
        if paddle_x > ball_x {
            return JoystickInput::Left;
        } else if paddle_x < ball_x {
            return JoystickInput::Right;
        } else {
            return JoystickInput::Neutral;
        }
    }
}

fn main() {
    let program = read_program("../input");

    let mut arcade = ArcadeCabinet::new(program, 2);
    let mut ai = BreakoutAi::new();
    println!("Initial block count: {}", arcade.screen.count(2));
    let stdin = io::stdin();
    let mut inbytes = stdin.lock().bytes();
    while arcade.vm.state != VmState::Terminated {
        let input = ai.think(&arcade.screen);
        arcade.step(input);
        arcade.screen.draw();
        thread::sleep_ms(10);
    }
}
