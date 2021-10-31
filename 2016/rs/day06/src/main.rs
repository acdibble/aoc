use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

type Comparer = dyn Fn(&(&char, &i32), &(&char, &i32)) -> std::cmp::Ordering;

fn decipher(input: &String, cmp: &Comparer) -> String {
    let mut repetitions: HashMap<usize, HashMap<char, i32>> = HashMap::new();
    let mut max_len = 0;

    for line in input.lines() {
        max_len = line.len();
        for (index, c) in line.char_indices() {
            let sub_map = repetitions.entry(index).or_default();
            *sub_map.entry(c).or_default() += 1;
        }
    }

    println!("{:?}", repetitions);

    (0..max_len)
        .into_iter()
        .map(|index| {
            repetitions
                .get(&index)
                .unwrap()
                .iter()
                .max_by(cmp)
                .map(|(k, _)| *k)
                .unwrap()
        })
        .collect()
}

fn part_one(input: &String) -> String {
    decipher(input, &|a: &(&char, &i32), b: &(&char, &i32)| a.1.cmp(&b.1))
}

fn part_two(input: &String) -> String {
    decipher(input, &|a: &(&char, &i32), b: &(&char, &i32)| b.1.cmp(&a.1))
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
