use std::{collections::HashMap, time::SystemTime};

const DATA: &'static str = include_str!("../data.txt");

#[derive(Debug)]
struct Rule {
    before: Vec<i32>,
    after: Vec<i32>,
}

impl Rule {
    fn new() -> Self {
        Self {
            before: Default::default(),
            after: Default::default(),
        }
    }
}

type RuleBook = HashMap<i32, Rule>;

#[derive(Debug)]
struct Manual(Vec<i32>);

impl Manual {
    fn new(pages: Vec<i32>) -> Self {
        Self(pages)
    }

    fn is_valid(&self, rules: &RuleBook) -> bool {
        for (i, n) in self.0.iter().enumerate() {
            let rule = rules.get(n).unwrap();
            if !self.0.iter().take(i).all(|m| rule.before.contains(m))
                || !self.0.iter().skip(i + 1).all(|m| rule.after.contains(m))
            {
                return false;
            }
        }

        true
    }

    fn middle_page(&self) -> i32 {
        self.0[self.0.len() / 2]
    }

    fn fix(&mut self, rules: &RuleBook) {
        let mut i = 0usize;

        while i < self.0.len() {
            let rule = rules.get(&self.0[i]).unwrap();

            if let Some(bad) = self
                .0
                .iter()
                .skip(i + 1)
                .find_map(|m| rule.before.contains(m).then_some(*m))
            {
                let index = self.0.iter().position(|m| m == &bad).unwrap();
                self.0.remove(index);
                self.0.insert(i, bad);
                i = 0;
            } else {
                i += 1;
            }
        }
    }
}

fn parse_data() -> (RuleBook, Vec<Manual>) {
    let mut rules = HashMap::new();

    let mut lines = DATA.lines();

    while let Some(line) = lines.next() {
        if line == "" {
            break;
        }

        let mut parts = line.split('|');

        let first = parts.next().unwrap().parse().unwrap();
        let second = parts.next().unwrap().parse().unwrap();

        let page = rules.entry(first).or_insert(Rule::new());
        page.after.push(second);
        let page = rules.entry(second).or_insert(Rule::new());
        page.before.push(first);
    }

    (
        rules,
        lines
            .into_iter()
            .map(|l| Manual::new(l.split(',').map(|n| n.parse().unwrap()).collect()))
            .collect(),
    )
}

fn part_one() -> i32 {
    let (rules, manuals) = parse_data();

    manuals
        .into_iter()
        .filter(|m| m.is_valid(&rules))
        .map(|m| m.middle_page())
        .sum()
}

fn part_two() -> i32 {
    let (rules, mut manuals) = parse_data();

    manuals.retain(|m| !m.is_valid(&rules));

    for m in &mut manuals {
        m.fix(&rules);
        assert!(m.is_valid(&rules));
    }

    manuals.iter().map(|m| m.middle_page()).sum()
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
