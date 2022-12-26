use std::{collections::HashMap, time::SystemTime};

const DATA: &'static str = include_str!("../data.txt");

#[derive(Debug)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
}

#[derive(Debug)]
enum Expr<'a> {
    Op(&'a str, Op, &'a str),
    Literal(i64),
}

fn parse_line(line: &str) -> (&str, Expr) {
    let mut parts = line.split_ascii_whitespace();
    let name = parts.next().unwrap();
    let monkey_or_literal = parts.next().unwrap();

    if let Ok(value) = monkey_or_literal.parse::<i64>() {
        return (&name[0..name.len() - 1], Expr::Literal(value));
    }

    let op = match parts.next().unwrap() {
        "+" => Op::Add,
        "-" => Op::Sub,
        "*" => Op::Mul,
        "/" => Op::Div,
        _ => unreachable!(),
    };

    (
        &name[0..name.len() - 1],
        Expr::Op(monkey_or_literal, op, parts.next().unwrap()),
    )
}

fn evaluate(name: &str, monkeys: &HashMap<&str, Expr>) -> i64 {
    match monkeys.get(name) {
        Some(Expr::Literal(value)) => *value,
        Some(Expr::Op(a, op, b)) => {
            let a = evaluate(a, monkeys);
            let b = evaluate(b, monkeys);
            match op {
                Op::Add => a + b,
                Op::Sub => a - b,
                Op::Mul => a * b,
                Op::Div => a / b,
                Op::Eq => {
                    if a == b {
                        1
                    } else {
                        0
                    }
                }
            }
        }
        None => unreachable!(),
    }
}

fn part_one() -> i64 {
    let monkeys: HashMap<_, _> = DATA.lines().map(parse_line).collect();

    evaluate("root", &monkeys)
}

fn part_two() -> i64 {
    let mut monkeys: HashMap<_, _> = DATA.lines().map(parse_line).collect();

    let root = monkeys.remove("root").unwrap();

    match root {
        Expr::Op(a, _, b) => {
            let cache: HashMap<_, _> = monkeys
                .keys()
                .map(|name| (*name, evaluate(name, &monkeys)))
                .collect();

            monkeys.insert("humn", Expr::Literal(0));

            for (name, value) in cache {
                if evaluate(name, &monkeys) == value {
                    monkeys.insert(name, Expr::Literal(value));
                }
            }

            monkeys.insert("root", Expr::Op(a, Op::Eq, b));
        }
        _ => unreachable!(),
    }

    for n in 3243420000000.. {
        monkeys.insert("humn", Expr::Literal(n));

        if evaluate("root", &monkeys) == 1 {
            return n;
        }
    }

    unreachable!()
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
