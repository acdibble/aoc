use std::{
    collections::{HashMap, HashSet, VecDeque},
    convert, env, fs,
    path::Path,
    time::SystemTime,
};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Opcode {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

impl convert::TryFrom<usize> for Opcode {
    type Error = ();

    fn try_from(number: usize) -> Result<Self, <Self as convert::TryFrom<usize>>::Error> {
        match number {
            0 => Ok(Self::Addr),
            1 => Ok(Self::Addi),
            2 => Ok(Self::Mulr),
            3 => Ok(Self::Muli),
            4 => Ok(Self::Banr),
            5 => Ok(Self::Bani),
            6 => Ok(Self::Borr),
            7 => Ok(Self::Bori),
            8 => Ok(Self::Setr),
            9 => Ok(Self::Seti),
            10 => Ok(Self::Gtir),
            11 => Ok(Self::Gtri),
            12 => Ok(Self::Gtrr),
            13 => Ok(Self::Eqir),
            14 => Ok(Self::Eqri),
            15 => Ok(Self::Eqrr),
            _ => Err(()),
        }
    }
}

impl Opcode {
    fn iter() -> impl Iterator<Item = Self> {
        (0..).map_while(|u| u.try_into().ok())
    }
}

struct Instruction(usize, usize, usize, usize);

impl Instruction {
    fn new(&[op, a, b, c]: &[usize; 4]) -> Self {
        Self(op, a, b, c)
    }
}

#[derive(Debug, Copy, Clone)]
struct CPU {
    registers: [usize; 4],
    opcode_mapping: [Opcode; 16],
}

impl CPU {
    fn new() -> Self {
        let mut mapping = [Opcode::Addi; 16];

        for (index, op) in Opcode::iter().enumerate() {
            mapping[index] = op;
        }

        Self {
            registers: [0; 4],
            opcode_mapping: mapping,
        }
    }

    fn evaluate(&mut self, instruction: &Instruction) {
        macro_rules! register_a {
            () => {
                self.registers[instruction.1]
            };
        }

        macro_rules! register_b {
            () => {
                self.registers[instruction.2]
            };
        }
        macro_rules! value_a {
            () => {
                instruction.1
            };
        }

        macro_rules! value_b {
            () => {
                instruction.2
            };
        }

        let result = match self.opcode_mapping[instruction.0] {
            Opcode::Addr => register_a!() + register_b!(),
            Opcode::Addi => register_a!() + value_b!(),
            Opcode::Mulr => register_a!() * register_b!(),
            Opcode::Muli => register_a!() * value_b!(),
            Opcode::Banr => register_a!() & register_b!(),
            Opcode::Bani => register_a!() & value_b!(),
            Opcode::Borr => register_a!() | register_b!(),
            Opcode::Bori => register_a!() | value_b!(),
            Opcode::Setr => register_a!(),
            Opcode::Seti => value_a!(),
            Opcode::Gtir => (value_a!() > register_b!()) as usize,
            Opcode::Gtri => (register_a!() > value_b!()) as usize,
            Opcode::Gtrr => (register_a!() > register_b!()) as usize,
            Opcode::Eqir => (value_a!() == register_b!()) as usize,
            Opcode::Eqri => (register_a!() == value_b!()) as usize,
            Opcode::Eqrr => (register_a!() == register_b!()) as usize,
        };

        self.registers[instruction.3] = result;
    }
}

macro_rules! before_after_array {
    ($line:ident) => {{
        let mut array = [0usize; 4];

        for (index, num) in $line
            .chars()
            .filter(|c| c.is_digit(10))
            .flat_map(|c| c.to_digit(10))
            .enumerate()
        {
            array[index] = num as usize;
        }

        array
    }};
}

macro_rules! usize_array {
    ($line:ident) => {{
        let mut array = [0; 4];

        for (index, num) in $line
            .split_ascii_whitespace()
            .flat_map(|n| n.parse::<usize>())
            .enumerate()
        {
            array[index] = num;
        }

        array
    }};
}

fn part_one(input: &str) -> (i32, usize) {
    let mut lines = input.lines();

    let mut ambiguous_samples = 0;

    let mut set_map = HashMap::new();

    while let Some(line) = lines.next() {
        if line == "" {
            break;
        }

        let mut cpu = CPU::new();
        cpu.registers = before_after_array!(line);

        let next_line = lines.next().unwrap();
        let inputs = usize_array!(next_line);

        let next_line = lines.next().unwrap();
        let expected = before_after_array!(next_line);

        let mut matches = 0;
        for (index, opcode) in Opcode::iter().enumerate() {
            let entry = set_map.entry(opcode).or_insert_with(|| HashSet::new());
            let mut new_cpu = cpu;
            new_cpu.evaluate(&Instruction(index, inputs[1], inputs[2], inputs[3]));
            if new_cpu.registers == expected {
                matches += 1;
                entry.insert(inputs[0]);
            }
        }

        if matches >= 3 {
            ambiguous_samples += 1;
        }

        lines.next();
    }

    let mut queue: VecDeque<_> = set_map.into_iter().collect();

    let mut opcode_map = [Opcode::Addi; 16];

    while let Some((opcode, set)) = queue.pop_front() {
        if set.len() > 1 {
            queue.push_back((opcode, set));
            continue;
        }

        let value = set.into_iter().nth(0).unwrap();
        opcode_map[value] = opcode;

        for (_, set) in queue.iter_mut() {
            set.remove(&value);
        }
    }

    lines.next();

    let mut cpu = CPU::new();
    cpu.opcode_mapping = opcode_map;
    while let Some(line) = lines.next() {
        let digits = usize_array!(line);

        cpu.evaluate(&Instruction::new(&digits));
    }

    (ambiguous_samples, cpu.registers[0])
}

fn time_it<F, T>(fun: F) -> T
where
    F: Fn() -> T,
{
    let start = SystemTime::now();
    let result = fun();
    println!("Time elapsed: {} µs", start.elapsed().unwrap().as_micros());
    result
}

fn main() -> std::io::Result<()> {
    let file_path = env::current_dir()?.join(Path::new("data.txt"));
    let input = fs::read_to_string(file_path)?;

    time_it(|| println!("parts (1, 2): {:?}", part_one(&input)));

    Ok(())
}
