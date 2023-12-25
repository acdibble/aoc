use std::{collections::HashMap, time::SystemTime};

const DATA: &'static str = include_str!("../data.txt");

fn parse_field(field: &'static str) -> i32 {
    field.split("=").last().unwrap().parse().unwrap()
}

#[derive(Debug, Clone, Copy)]
struct Part<T: Clone + Copy> {
    x: T,
    m: T,
    a: T,
    s: T,
}

impl<T: Clone + Copy> Part<T> {
    fn get(&self, field: &'static str) -> T {
        match field {
            "x" => self.x,
            "a" => self.a,
            "m" => self.m,
            "s" => self.s,
            _ => unreachable!(),
        }
    }

    fn get_mut(&mut self, field: &'static str) -> &mut T {
        match field {
            "x" => &mut self.x,
            "m" => &mut self.m,
            "a" => &mut self.a,
            "s" => &mut self.s,
            _ => unreachable!(),
        }
    }
}

impl Part<i32> {
    fn rating(&self) -> i32 {
        self.x + self.m + self.a + self.s
    }
}

impl Part<(usize, usize)> {
    fn new() -> Self {
        Self {
            x: (1, 4000),
            m: (1, 4000),
            a: (1, 4000),
            s: (1, 4000),
        }
    }

    fn split_lt(&self, field: &'static str, amount: usize) -> (Self, Self) {
        let mut left = *self;
        let mut right = *self;
        let upper = &mut left.get_mut(field).1;
        let lower = &mut right.get_mut(field).0;
        *upper = (*upper).min(amount - 1);
        *lower = (*lower).max(amount);
        (left, right)
    }

    fn split_gt(&self, field: &'static str, amount: usize) -> (Self, Self) {
        let mut left = *self;
        let mut right = *self;
        let upper = &mut left.get_mut(field).1;
        let lower = &mut right.get_mut(field).0;
        *upper = (*upper).min(amount);
        *lower = (*lower).max(amount + 1);
        (right, left)
    }

    fn split(&self, cond: Condition) -> (Self, Self) {
        match cond {
            (field, "<", amount) => self.split_lt(field, amount as usize),
            (field, ">", amount) => self.split_gt(field, amount as usize),
            _ => unreachable!(),
        }
    }

    fn possibilities(&self) -> usize {
        macro_rules! len {
            ($range:expr) => {
                if $range.1 < $range.0 {
                    0
                } else {
                    1 + $range.1 - $range.0
                }
            };
        }

        len!(self.x) * len!(self.m) * len!(self.a) * len!(self.s)
    }
}

type Condition = (&'static str, &'static str, i32);

#[derive(Debug)]
struct Rule {
    dest: &'static str,
    cond: Option<Condition>,
}

impl Rule {
    fn test(&self, part: &Part<i32>) -> bool {
        if let Some((field, op, amount)) = self.cond {
            let value = part.get(field);

            match op {
                ">" => value > amount,
                "<" => value < amount,
                _ => unreachable!(),
            }
        } else {
            true
        }
    }
}

impl From<&'static str> for Rule {
    fn from(value: &'static str) -> Self {
        let colon = value.find(':');
        if colon.is_none() {
            return Self {
                dest: value,
                cond: None,
            };
        }

        let colon = colon.unwrap();

        let field = &value[..1];
        let op = &value[1..2];
        let amount: i32 = value[2..colon].parse().unwrap();
        let dest = &value[colon + 1..];

        Self {
            dest,
            cond: Some((field, op, amount)),
        }
    }
}

#[derive(Debug)]
struct Workflow {
    name: &'static str,
    rules: Vec<Rule>,
}

impl Workflow {
    fn find_dest(&self, part: &Part<i32>) -> &'static str {
        for cond in &self.rules {
            if cond.test(part) {
                return cond.dest;
            }
        }

        unreachable!()
    }
}

impl From<&'static str> for Workflow {
    fn from(s: &'static str) -> Self {
        let name_end = s.find('{').unwrap();
        let name = &s[..name_end];
        let rules = s[name_end + 1..s.len() - 1]
            .split(",")
            .map(Rule::from)
            .collect();

        Workflow { name, rules }
    }
}

fn part_one() -> i32 {
    let mut split = DATA.trim().split("\n\n");
    let workflows: HashMap<_, _> = split
        .next()
        .unwrap()
        .lines()
        .map(Workflow::from)
        .map(|w| (w.name, w))
        .collect();

    let parts = split.next().unwrap().lines().map(|l| {
        let mut split = l[1..l.len() - 1].split(",");

        Part {
            x: parse_field(split.next().unwrap()),
            m: parse_field(split.next().unwrap()),
            a: parse_field(split.next().unwrap()),
            s: parse_field(split.next().unwrap()),
        }
    });

    let mut total = 0;

    for part in parts {
        let mut workflow = "in";

        while workflow != "A" && workflow != "R" {
            workflow = workflows[workflow].find_dest(&part);
        }

        if workflow == "A" {
            total += part.rating();
        }
    }

    total
}

fn walk(
    workflows: &HashMap<&'static str, Workflow>,
    workflow: &'static str,
    mut range: Part<(usize, usize)>,
) -> usize {
    let flow = &workflows[workflow];
    let mut total = 0;

    for rule in &flow.rules {
        if let Some(cond) = rule.cond {
            let (l, r) = range.split(cond);
            if rule.dest == "A" {
                total += l.possibilities();
            } else if rule.dest != "R" {
                total += walk(workflows, rule.dest, l);
            }
            range = r;
        } else {
            if rule.dest == "A" {
                total += range.possibilities();
            } else if rule.dest != "R" {
                total += walk(workflows, rule.dest, range)
            }
        }
    }

    total
}

fn part_two() -> usize {
    let mut split = DATA.trim().split("\n\n");
    let workflows: HashMap<_, _> = split
        .next()
        .unwrap()
        .lines()
        .map(Workflow::from)
        .map(|w| (w.name, w))
        .collect();

    walk(&workflows, "in", Part::<(usize, usize)>::new())
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
