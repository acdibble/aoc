use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Step {
    name: char,
    constraints: HashSet<char>,
}

fn main() {
    let mut parsed: Vec<Step> = Vec::with_capacity(26);
    let mut all_steps: HashSet<char> = Default::default();
    let mut steps_with_constraints: HashSet<char> = Default::default();

    BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .for_each(|line| {
            let unwrapped = line.unwrap();
            let mut chars = unwrapped.chars();
            let constraint = chars.nth(5).unwrap();
            let name = chars.nth(30).unwrap();
            all_steps.insert(name);
            steps_with_constraints.insert(name);
            all_steps.insert(constraint);
            if let Some(index) = parsed.iter().position(|step| step.name == name) {
                parsed[index].constraints.insert(constraint);
            } else {
                let mut constraints = HashSet::new();
                constraints.insert(constraint);
                parsed.push(Step { name, constraints });
            }
        });

    let mut available_steps: Vec<_> = all_steps
        .difference(&steps_with_constraints)
        .cloned()
        .collect();

    let mut result = String::new();

    while available_steps.len() != 0 {
        available_steps.sort();
        let current = available_steps.remove(0);
        result.push(current);

        for step in parsed.iter_mut() {
            if step.constraints.remove(&current) && step.constraints.len() == 0 {
                available_steps.push(step.name);
            }
        }
    }

    println!("{:?}", &result);
}
