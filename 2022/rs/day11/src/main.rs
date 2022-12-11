use std::time::SystemTime;

const DATA: &'static str = include_str!("../data.txt");

#[derive(Debug)]
enum Operand {
    Old,
    Literal(u64),
}

impl Operand {
    fn or(self, other: Self) -> Self {
        match self {
            Self::Old => other,
            _ => self,
        }
    }
}

impl From<&str> for Operand {
    fn from(value: &str) -> Self {
        match value {
            "old" => Self::Old,
            _ => Self::Literal(value.parse().unwrap()),
        }
    }
}

#[derive(Debug)]
enum Op {
    Add(Operand),
    Mul(Operand),
}

impl Op {
    fn from(operator: &str, operand1: Operand, operand2: Operand) -> Self {
        match operator {
            "+" => Self::Add(operand1.or(operand2)),
            "*" => Self::Mul(operand1.or(operand2)),
            _ => unreachable!(),
        }
    }
}

fn parse_list_items(line: &str) -> Vec<u64> {
    let nums = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

    line.split_ascii_whitespace()
        .flat_map(|item| {
            if !item.starts_with(nums) {
                return None;
            }

            let end = item.len() - if item.ends_with(',') { 1 } else { 0 };

            item[0..end].parse().ok()
        })
        .collect()
}

fn parse_operation(line: &str) -> Op {
    let mut parts = line.split_ascii_whitespace();
    parts.next();
    parts.next();
    parts.next();
    let operand1 = Operand::from(parts.next().unwrap());
    let operator = parts.next().unwrap();
    let operand2 = Operand::from(parts.next().unwrap());
    Op::from(operator, operand1, operand2)
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    operation: Op,
    test: u64,
    yes: usize,
    no: usize,
    monkey_business: u64,
}

impl Monkey {
    fn inspect(&mut self, item: &u64) -> u64 {
        self.monkey_business += 1;
        match self.operation {
            Op::Add(Operand::Literal(value)) => item + value,
            Op::Add(Operand::Old) => item + item,
            Op::Mul(Operand::Literal(value)) => item * value,
            Op::Mul(Operand::Old) => item * item,
        }
    }

    fn test(&self, item: &u64) -> usize {
        if item % self.test == 0 {
            self.yes
        } else {
            self.no
        }
    }
}

fn parse_monkeys() -> Vec<Monkey> {
    let mut lines = DATA.lines();

    let mut monkeys = Vec::new();

    while let Some(_) = lines.next() {
        let items = lines.next().unwrap().trim();
        let operation = lines.next().unwrap().trim();

        let test = lines.next().unwrap().trim();
        let yes = lines.next().unwrap().trim();
        let no = lines.next().unwrap().trim();
        lines.next();

        monkeys.push(Monkey {
            items: parse_list_items(items),
            operation: parse_operation(operation),
            test: test
                .split_ascii_whitespace()
                .last()
                .unwrap()
                .parse()
                .unwrap(),
            yes: yes
                .split_ascii_whitespace()
                .last()
                .unwrap()
                .parse()
                .unwrap(),
            no: no.split_ascii_whitespace().last().unwrap().parse().unwrap(),
            monkey_business: 0,
        })
    }

    monkeys
}

fn find_monkey_business(mut monkeys: Vec<Monkey>, modulo: Option<u64>, rounds: u64) -> u64 {
    let mut buffer = Vec::new();

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            std::mem::swap(&mut monkeys[i].items, &mut buffer);
            for item in &buffer {
                let mut new_value = monkeys[i].inspect(item);
                if let Some(modulo) = modulo {
                    new_value %= modulo
                } else {
                    new_value /= 3
                };
                let index = monkeys[i].test(&new_value);
                monkeys[index].items.push(new_value);
            }
            buffer.clear();
        }
    }

    let mut highest = 0;
    let mut second = 0;

    for monkey in monkeys {
        if monkey.monkey_business > highest {
            second = highest;
            highest = monkey.monkey_business;
        } else if monkey.monkey_business > second {
            second = monkey.monkey_business;
        }
    }

    highest * second
}

fn part_one() -> u64 {
    let monkeys = parse_monkeys();

    find_monkey_business(monkeys, None, 20)
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

fn part_two() -> u64 {
    let monkeys = parse_monkeys();

    let modulo = monkeys.iter().map(|m| m.test).fold(1, |a, b| lcm(a, b));

    find_monkey_business(monkeys, Some(modulo), 10000)
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

fn main() {
    time_it(|| println!("part 1: {}", part_one()));
    time_it(|| println!("part 2: {}", part_two()));
}
