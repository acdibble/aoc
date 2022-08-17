use std::collections::VecDeque;

#[derive(Clone, Copy, Debug)]
enum Mode {
    Position,
    Immediate,
}

impl From<i32> for Mode {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::Position,
            1 => Self::Immediate,
            _ => unreachable!(),
        }
    }
}

enum OpCode {
    Add,
    Mul,
    In,
    Out,
    Jnz,
    Jz,
    Lt,
    Eq,
    Halt,
}

impl From<i32> for OpCode {
    fn from(value: i32) -> Self {
        match value {
            1 => Self::Add,
            2 => Self::Mul,
            3 => Self::In,
            4 => Self::Out,
            5 => Self::Jnz,
            6 => Self::Jz,
            7 => Self::Lt,
            8 => Self::Eq,
            99 => Self::Halt,
            _ => unreachable!("unexpected op: {value}"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct VM {
    intcodes: Box<[i32]>,
    pc: usize,
    input: VecDeque<i32>,
    output: VecDeque<i32>,

    mode_one: Mode,
    mode_two: Mode,
    mode_three: Mode,

    halted: bool,
}

impl From<Vec<i32>> for VM {
    fn from(intcodes: Vec<i32>) -> Self {
        Self {
            intcodes: intcodes.into_boxed_slice(),
            pc: 0,
            input: Default::default(),
            output: Default::default(),
            mode_one: Mode::Position,
            mode_two: Mode::Position,
            mode_three: Mode::Position,
            halted: false,
        }
    }
}

impl From<&str> for VM {
    fn from(input: &str) -> Self {
        let opcodes = Self::parse_intcodes(input);
        Self::from(opcodes)
    }
}

impl From<&Vec<i32>> for VM {
    fn from(intcodes: &Vec<i32>) -> Self {
        Self {
            intcodes: intcodes.clone().into_boxed_slice(),
            pc: 0,
            input: Default::default(),
            output: Default::default(),
            mode_one: Mode::Position,
            mode_two: Mode::Position,
            mode_three: Mode::Position,
            halted: false,
        }
    }
}

impl VM {
    pub fn parse_intcodes(input: &str) -> Vec<i32> {
        input
            .trim()
            .split(',')
            .flat_map(str::parse)
            .collect::<Vec<i32>>()
    }

    pub fn reset(&mut self, intcodes: &Vec<i32>) {
        self.intcodes.copy_from_slice(intcodes);
        self.pc = 0;
        self.input.clear();
        self.output.clear();
        self.halted = false;
    }

    fn read_int(&mut self) -> i32 {
        let code = self.intcodes[self.pc];
        self.pc += 1;
        code
    }

    fn read_param(&mut self, mode: Mode) -> i32 {
        let value = self.read_int();

        match mode {
            Mode::Position => self.intcodes[value as usize],
            Mode::Immediate => value,
        }
    }

    fn read_op(&mut self) -> OpCode {
        let int = self.read_int();

        self.mode_one = Mode::from(int / 100 % 10);
        self.mode_two = Mode::from(int / 1000 % 10);
        self.mode_three = Mode::from(int / 10000 % 10);

        OpCode::from(int % 100)
    }

    fn read_input(&mut self) -> Option<i32> {
        self.input.pop_back()
    }

    fn write_output(&mut self, value: i32) {
        self.output.push_front(value)
    }

    pub fn write_input(&mut self, value: i32) {
        self.input.push_front(value)
    }

    pub fn read_output(&mut self) -> Option<i32> {
        self.output.pop_back()
    }

    pub fn run(&mut self) {
        if self.halted {
            return;
        }

        loop {
            match self.read_op() {
                OpCode::Add => {
                    let a = self.read_param(self.mode_one);
                    let b = self.read_param(self.mode_two);
                    let index = self.read_int() as usize;

                    self.intcodes[index] = a + b;
                }
                OpCode::Mul => {
                    let a = self.read_param(self.mode_one);
                    let b = self.read_param(self.mode_two);
                    let index = self.read_int() as usize;

                    self.intcodes[index] = a * b;
                }
                OpCode::In => match self.read_input() {
                    Some(value) => {
                        let index = self.read_int() as usize;
                        self.intcodes[index] = value;
                    }
                    _ => {
                        self.pc -= 1;
                        return;
                    }
                },
                OpCode::Out => {
                    let param = self.read_param(self.mode_one);

                    self.write_output(param);
                }
                OpCode::Jnz => {
                    let a = self.read_param(self.mode_one);
                    let b = self.read_param(self.mode_two);

                    if a != 0 {
                        self.pc = b as usize;
                    }
                }
                OpCode::Jz => {
                    let a = self.read_param(self.mode_one);
                    let b = self.read_param(self.mode_two);

                    if a == 0 {
                        self.pc = b as usize;
                    }
                }
                OpCode::Lt => {
                    let a = self.read_param(self.mode_one);
                    let b = self.read_param(self.mode_two);
                    let index = self.read_int() as usize;

                    self.intcodes[index] = if a < b { 1 } else { 0 }
                }
                OpCode::Eq => {
                    let a = self.read_param(self.mode_one);
                    let b = self.read_param(self.mode_two);
                    let index = self.read_int() as usize;

                    self.intcodes[index] = if a == b { 1 } else { 0 }
                }
                OpCode::Halt => {
                    self.halted = true;
                    return;
                }
            }
        }
    }
}
