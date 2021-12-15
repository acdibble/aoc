use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

#[derive(Debug)]
struct Step {
    name: char,
    constraints: HashSet<char>,
}

fn part_one(input: &str) -> String {
    let mut parsed: Vec<Step> = Vec::with_capacity(26);
    let mut all_steps: HashSet<char> = Default::default();
    let mut steps_with_constraints: HashSet<char> = Default::default();

    for line in input.lines() {
        let mut chars = line.chars();
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
    }

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

    result
}

#[derive(Default, Debug)]
struct Worker {
    time_remaining: u32,
    task: Option<char>,
}

impl Worker {
    fn assign(&mut self, c: &char) {
        self.task = Some(c.clone());
        self.time_remaining = 60 + *c as u32 - b'A' as u32;
    }

    fn tick(&mut self) {
        if self.task.is_some() && self.time_remaining > 0 {
            self.time_remaining -= 1;
        }
    }

    fn get_result(&mut self) -> Option<char> {
        let mut option = None;
        if self.task.is_some() && self.time_remaining == 0 {
            option = self.task;
            self.task = None;
        }
        return option;
    }

    fn needs_task(&self) -> bool {
        self.task.is_none()
    }
}

fn part_two(input: &str) -> i32 {
    let mut parsed: Vec<Step> = Vec::with_capacity(26);
    let mut all_steps: HashSet<char> = Default::default();
    let mut steps_with_constraints: HashSet<char> = Default::default();

    for line in input.lines() {
        let mut chars = line.chars();
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
    }

    let mut available_steps: Vec<_> = all_steps
        .difference(&steps_with_constraints)
        .cloned()
        .collect();

    let mut time_elapsed = 0;
    let mut completed_steps = String::new();
    let mut workers: [Worker; 5] = Default::default();

    while completed_steps.len() != all_steps.len() {
        available_steps.sort();

        let work_assignable = !available_steps.is_empty();

        for worker in &mut workers {
            worker.tick();
            if let Some(current) = worker.get_result() {
                completed_steps.push(current);

                for step in &mut parsed {
                    if step.constraints.remove(&current) && step.constraints.len() == 0 {
                        available_steps.push(step.name);
                    }
                }
            } else if worker.needs_task() && work_assignable && !available_steps.is_empty() {
                let new_task = available_steps.remove(0);
                worker.assign(&new_task);
            }
        }

        time_elapsed += 1;
    }

    time_elapsed
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

    time_it(|| println!("part 1: {}", part_one(&input)));
    time_it(|| println!("part 2: {}", part_two(&input)));

    Ok(())
}
