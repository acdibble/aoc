use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

#[derive(Debug)]
struct Program<'a> {
    name: &'a str,
    weight: i32,
    holds: Option<Vec<&'a str>>,
}

fn parse_programs(input: &str) -> Vec<Program> {
    input
        .lines()
        .map(|line| {
            let mut words = line.split_ascii_whitespace();

            let name = words.next().unwrap();
            let weight = words.next().unwrap();
            let weight: i32 = weight[1..weight.len() - 1].parse().unwrap();

            match words.next() {
                None => {
                    return Program {
                        name,
                        weight,
                        holds: None,
                    }
                }
                _ => (),
            }

            let mut holds = vec![];
            while let Some(name) = words.next() {
                if name.ends_with(',') {
                    holds.push(&name[..name.len() - 1]);
                } else {
                    holds.push(name)
                }
            }

            Program {
                name,
                weight,
                holds: Some(holds),
            }
        })
        .collect()
}

fn part_one(input: &str) -> &str {
    let programs = parse_programs(input);

    let mut current = programs.first().unwrap();

    loop {
        match programs.iter().find(|p| match &p.holds {
            None => false,
            Some(list) => list.contains(&current.name),
        }) {
            Some(program) => current = program,
            _ => break,
        }
    }

    current.name
}

fn find_sum(programs: &HashMap<&str, Program>, name: &str) -> i32 {
    let program = programs.get(name).unwrap();

    program.weight
        + match &program.holds {
            None => 0,
            Some(list) => list
                .iter()
                .map(|held| {
                    let sum = find_sum(programs, held);
                    sum
                })
                .sum::<i32>(),
        }
}

fn part_two(input: &str, root: &str) -> i32 {
    let map: HashMap<&str, Program> = parse_programs(input)
        .into_iter()
        .map(|p| (p.name, p))
        .collect();
    let mut current = map.get(root).unwrap();
    let mut expected_weight = 0;
    let mut frequencies = HashMap::<i32, i32>::new();

    while let Some(list) = &current.holds {
        frequencies.clear();
        let sums: Vec<i32> = list
            .iter()
            .map(|name| {
                let sum = find_sum(&map, name);
                let entry = frequencies.entry(sum).or_default();
                *entry += 1;
                sum
            })
            .collect();

        let mut off_weight = None;
        if frequencies.len() != 1 {
            for (&weight, &freq) in frequencies.iter() {
                if freq != 1 {
                    expected_weight = weight;
                } else {
                    off_weight = Some(weight);
                }
            }
        }

        if let Some(off_weight) = off_weight {
            let index = sums
                .into_iter()
                .position(|weight| weight == off_weight)
                .unwrap();

            let name = list[index];
            current = map.get(name).unwrap();
        } else {
            return expected_weight - sums.into_iter().sum::<i32>();
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use crate::part_one;

    #[test]
    fn test_part_one() {
        assert_eq!(
            part_one(
                "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)"
            ),
            "tknk"
        )
    }
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

    let root = time_it(|| {
        let result = part_one(&input);
        println!("part 1: {}", result);
        result
    });
    time_it(|| println!("part 2: {}", part_two(&input, root)));

    Ok(())
}
