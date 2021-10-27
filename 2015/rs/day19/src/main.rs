use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::path::Path;

type ParsedData<'a> = (HashMap<&'a str, Vec<&'a str>>, &'a str);

fn parse_replacements(input: &String) -> ParsedData {
    let mut counts = HashMap::<&str, Vec<&str>>::new();
    let mut molecule: &str = "";

    let mut line_it = input.lines();
    while let Some(line) = line_it.next() {
        let mut it = line.split_ascii_whitespace();
        if let Some(name) = it.next() {
            let entry = counts.entry(name).or_default();
            it.next();
            entry.push(it.next().unwrap());
        } else {
            molecule = line_it.next().unwrap();
        }
    }

    (counts, molecule)
}

fn part_one((map, molecule): &ParsedData) -> usize {
    let mut combinations = HashSet::new();

    let mut string = String::new();

    let mut it = molecule.chars().enumerate().peekable();
    while let Some((index, c)) = it.next() {
        let slice_until = index;
        let mut slice_from = index + 1;
        string.clear();
        string.push(c);

        let replacements = map
            .get(string.as_str())
            .or_else(|| {
                let (_, next) = it.peek()?;
                slice_from += 1;
                string.push(*next);
                map.get(string.as_str())
            })
            .map(|vec| vec.as_slice())
            .unwrap_or(&[]);

        for r in replacements {
            let mut new_variant = String::with_capacity(r.len() + molecule.len());
            new_variant.push_str(&molecule[..slice_until]);
            new_variant.push_str(*r);
            new_variant.push_str(&molecule[slice_from..]);
            {
                #[cfg(debug_assertions)]
                println!("{}", &new_variant);
            }
            combinations.insert(new_variant);
        }
    }

    {
        #[cfg(debug_assertions)]
        println!("{:?}", combinations);
    }

    combinations.len()
}

fn part_two(replacement_list: &mut Vec<(&str, &str)>, molecule: &str) -> i32 {
    replacement_list.shuffle(&mut thread_rng());
    let mut target = molecule.to_string();
    let mut steps = 0;

    while target != "e" {
        let steps_before = steps;
        for (element, replacement) in replacement_list.iter() {
            if target.contains(*element) {
                target = target.replacen(*element, replacement, 1);
                steps += 1;
            }
        }

        if steps_before == steps {
            return part_two(replacement_list, molecule);
        }
    }

    steps
}

fn main() -> std::io::Result<()> {
    let file_path = env::current_dir()?.join(Path::new("data.txt"));
    let input = fs::read_to_string(file_path)?;

    let inputs = parse_replacements(&input);

    println!("part 1: {}", part_one(&inputs));

    let mut replacement_list = Vec::new();
    for (key, values) in &inputs.0 {
        for value in values {
            replacement_list.push((*value, *key))
        }
    }
    println!("part 2: {}", part_two(&mut replacement_list, inputs.1));

    Ok(())
}
