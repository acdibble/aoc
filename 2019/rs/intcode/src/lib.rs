use std::collections::VecDeque;

#[derive(Clone, Copy, Debug)]
enum Mode {
    Position,
    Immediate,
    Relative,
}

impl From<i64> for Mode {
    fn from(value: i64) -> Self {
        match value {
            0 => Self::Position,
            1 => Self::Immediate,
            2 => Self::Relative,
            _ => unreachable!(),
        }
    }
}

enum Op {
    Add,
    Multiply,
    Input,
    Output,
    JumpIfNotZero,
    JumpIfZero,
    LessThan,
    Equal,
    AdjustRelativeBase,
    Halt,
}

impl From<i64> for Op {
    fn from(value: i64) -> Self {
        match value {
            1 => Self::Add,
            2 => Self::Multiply,
            3 => Self::Input,
            4 => Self::Output,
            5 => Self::JumpIfNotZero,
            6 => Self::JumpIfZero,
            7 => Self::LessThan,
            8 => Self::Equal,
            9 => Self::AdjustRelativeBase,
            99 => Self::Halt,
            _ => unreachable!("unexpected op: {value}"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct VM {
    intcodes: Vec<i64>,
    pc: usize,
    input: VecDeque<i64>,
    output: VecDeque<i64>,

    mode_one: Mode,
    mode_two: Mode,
    mode_three: Mode,

    pub halted: bool,
    relative_base: i64,
}

impl From<Vec<i64>> for VM {
    fn from(intcodes: Vec<i64>) -> Self {
        Self {
            intcodes,
            pc: 0,
            input: Default::default(),
            output: Default::default(),
            mode_one: Mode::Position,
            mode_two: Mode::Position,
            mode_three: Mode::Position,
            halted: false,
            relative_base: 0,
        }
    }
}

impl From<&str> for VM {
    fn from(input: &str) -> Self {
        Self::from(Self::parse_intcodes(input))
    }
}

impl From<&Vec<i64>> for VM {
    fn from(intcodes: &Vec<i64>) -> Self {
        Self::from(intcodes.clone())
    }
}

impl VM {
    pub fn parse_intcodes(input: &str) -> Vec<i64> {
        input
            .trim()
            .split(',')
            .flat_map(str::parse)
            .collect::<Vec<i64>>()
    }

    pub fn reset(&mut self, intcodes: &Vec<i64>) {
        self.intcodes.copy_from_slice(intcodes);
        self.pc = 0;
        self.input.clear();
        self.output.clear();
        self.halted = false;
    }

    fn read_int(&mut self) -> i64 {
        let code = self.intcodes[self.pc];
        self.pc += 1;
        code
    }

    fn write_memory(&mut self, mode: Mode, value: i64) {
        let at = self.read_int();
        let index = match mode {
            Mode::Immediate => unreachable!("cannot write memory in immediate mode"),
            Mode::Relative => at + self.relative_base,
            Mode::Position => at,
        } as usize;

        if self.intcodes.len() <= index {
            self.intcodes.resize(index + 1, 0);
        }

        self.intcodes[index] = value;
    }

    fn read_memory(&mut self, index: usize) -> i64 {
        if self.intcodes.len() <= index {
            self.intcodes.resize(index + 1, 0);
        }

        self.intcodes[index]
    }

    fn read_param(&mut self, mode: Mode) -> i64 {
        let value = self.read_int();

        match mode {
            Mode::Position => self.read_memory(value as usize),
            Mode::Immediate => value,
            Mode::Relative => self.read_memory((self.relative_base + value) as usize),
        }
    }

    fn read_op(&mut self) -> Op {
        let int = self.read_int();

        self.mode_one = Mode::from(int / 100 % 10);
        self.mode_two = Mode::from(int / 1000 % 10);
        self.mode_three = Mode::from(int / 10000 % 10);

        Op::from(int % 100)
    }

    fn read_input(&mut self) -> Option<i64> {
        self.input.pop_back()
    }

    fn write_output(&mut self, value: i64) {
        self.output.push_front(value)
    }

    pub fn write_input(&mut self, value: i64) {
        self.input.push_front(value)
    }

    pub fn read_output(&mut self) -> Option<i64> {
        self.output.pop_back()
    }

    pub fn run(&mut self) {
        if self.halted {
            return;
        }

        loop {
            match self.read_op() {
                Op::Add => {
                    let a = self.read_param(self.mode_one);
                    let b = self.read_param(self.mode_two);
                    self.write_memory(self.mode_three, a + b);
                }
                Op::Multiply => {
                    let a = self.read_param(self.mode_one);
                    let b = self.read_param(self.mode_two);
                    self.write_memory(self.mode_three, a * b);
                }
                Op::Input => match self.read_input() {
                    Some(value) => self.write_memory(self.mode_one, value),
                    _ => {
                        self.pc -= 1;
                        return;
                    }
                },
                Op::Output => {
                    let param = self.read_param(self.mode_one);

                    self.write_output(param);
                }
                Op::JumpIfNotZero => {
                    let a = self.read_param(self.mode_one);
                    let b = self.read_param(self.mode_two);

                    if a != 0 {
                        self.pc = b as usize;
                    }
                }
                Op::JumpIfZero => {
                    let a = self.read_param(self.mode_one);
                    let b = self.read_param(self.mode_two);

                    if a == 0 {
                        self.pc = b as usize;
                    }
                }
                Op::LessThan => {
                    let a = self.read_param(self.mode_one);
                    let b = self.read_param(self.mode_two);
                    self.write_memory(self.mode_three, if a < b { 1 } else { 0 })
                }
                Op::Equal => {
                    let a = self.read_param(self.mode_one);
                    let b = self.read_param(self.mode_two);
                    self.write_memory(self.mode_three, if a == b { 1 } else { 0 })
                }
                Op::AdjustRelativeBase => {
                    self.relative_base += self.read_param(self.mode_one);
                }
                Op::Halt => {
                    self.halted = true;
                    return;
                }
            }
        }
    }
}
