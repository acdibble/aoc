use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

#[derive(Debug)]
struct Rule {
    root: char,
    outputs: Vec<(char, char)>,
}

type RuleMap = HashMap<char, Vec<(char, char)>>;

fn parse_rules(lines: std::str::Lines) -> RuleMap {
    let mut rules: RuleMap = HashMap::new();

    for line in lines {
        let mut chars = line.chars();
        let first = chars.next().unwrap();
        let second = chars.next().unwrap();
        let output = chars.nth(4).unwrap();

        let outputs = rules.entry(first).or_default();
        outputs.push((second, output));
    }

    rules
}

fn find_outputs((a, b): (char, char), rules: &RuleMap) -> ((char, char), (char, char)) {
    if let Some(outputs) = rules.get(&a) {
        for &(second, output) in outputs {
            if second == b {
                return ((a, output), (output, b));
            }
        }
    }

    unreachable!()
}

fn solve(input: &str, iterations: i32) -> u64 {
    let mut lines = input.lines();

    let polymer_template = lines.next().unwrap();

    lines.next();

    let rules = parse_rules(lines);

    // maps inputs to outputs, e.g. CH -> (CB, BH)
    let mut input_output_map = HashMap::new();
    // counts yields, e.g. CH -> { C: 1, B: 1 }
    // only count the first element of the polymer to prevent double counting
    let mut one_round_totals = HashMap::new();

    // for each possible pair of elements
    for (a, outputs) in &rules {
        for (b, _) in outputs {
            let pair = (*a, *b);
            // find its two outputs
            let (one, two) = find_outputs(pair, &rules);

            let mut output_totals = HashMap::new();

            // track what is produced
            input_output_map.insert(pair, (one, two));

            // and how much is produced
            *output_totals.entry(one.0).or_insert(0) += 1;
            *output_totals.entry(two.0).or_insert(0) += 1;

            one_round_totals.insert(pair, output_totals);
        }
    }

    let mut pair_counts = HashMap::new();

    // parse the polymer template into pairs of elements
    for chars in polymer_template
        .chars()
        .zip(polymer_template.chars().skip(1))
    {
        // and count how often each pair appears
        *pair_counts.entry(chars).or_insert(0) += 1;
    }

    let mut temporary_pair_counts = HashMap::new();

    // as many times as necessary
    for _ in 0..(iterations - 1) {
        // for each pair of elements currently present
        for (pair, count) in &pair_counts {
            // find its outputs
            if let Some((output_one, output_two)) = input_output_map.get(pair) {
                // and track how much is produced
                *temporary_pair_counts.entry(*output_one).or_insert(0) += count;
                *temporary_pair_counts.entry(*output_two).or_insert(0) += count;
            }
        }

        // swap the memory for the next iteration
        std::mem::swap(&mut temporary_pair_counts, &mut pair_counts);
        temporary_pair_counts.clear();
    }
    // after calculating how much of each pair of elements is produced

    let mut grand_total_map = HashMap::new();
    // for each pair produced
    for (pair, count) in pair_counts {
        // find out what elements are produced by that pair
        for (ch, amount) in one_round_totals.get(&pair).unwrap() {
            // and calculate how much is produced
            *grand_total_map.entry(*ch).or_insert(0) += count * amount;
        }
    }

    // only the first elements of pairs were counted, which means the last
    // element in the polymer template was never counted
    *grand_total_map
        .entry(polymer_template.chars().last().unwrap())
        .or_default() += 1;

    let mut min = u64::MAX;
    let mut max = u64::MIN;

    for (_, count) in grand_total_map {
        min = count.min(min);
        max = count.max(max);
    }

    max - min
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

    time_it(|| println!("part 1: {}", solve(&input, 10)));
    time_it(|| println!("part 2: {}", solve(&input, 40)));

    Ok(())
}
