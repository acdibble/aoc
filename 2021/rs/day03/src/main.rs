use std::cmp::Ordering;
use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

#[inline(always)]
fn bin_to_dec(bin: &str) -> i32 {
    let mut output = 0;

    let len = bin.len() - 1;
    for (shift, ch) in bin.char_indices() {
        if ch == '1' {
            output |= 1 << (len - shift)
        }
    }

    output
}

fn part_one(input: &str) -> i32 {
    let mut counts = Vec::<(i32, i32)>::new();

    for line in input.lines() {
        for (index, ch) in line.char_indices() {
            let tuple = if let Some(value) = counts.get_mut(index) {
                value
            } else {
                counts.push((0, 0));
                counts.last_mut().unwrap()
            };

            match ch {
                '0' => tuple.0 += 1,
                '1' => tuple.1 += 1,
                _ => unreachable!(),
            }
        }
    }

    let mask = (0..counts.len()).fold(0, |acc, n| acc | (1 << n));

    let mut buffer = String::with_capacity(counts.len());
    for (zero, one) in counts {
        if zero > one {
            buffer.push('0')
        } else {
            buffer.push('1');
        }
    }

    let gamma = bin_to_dec(&buffer);

    let epsilon = !gamma & mask;

    gamma * epsilon
}

enum Rating {
    Oxygen,
    Scrubber,
}

fn find_rating(kind: Rating, lines: &Vec<&str>) -> usize {
    let mut iterators: Vec<_> = lines
        .iter()
        .enumerate()
        .map(|(i, l)| (i, l.chars().peekable()))
        .collect();

    let mut zeros = Vec::with_capacity(iterators.len());
    let mut ones = Vec::with_capacity(iterators.len());

    while iterators.len() != 1 {
        while let Some(mut tuple) = iterators.pop() {
            match tuple.1.next() {
                Some('0') => zeros.push(tuple),
                Some('1') => ones.push(tuple),
                _ => unreachable!(),
            }
        }

        match zeros.len().cmp(&ones.len()) {
            Ordering::Equal | Ordering::Less => match kind {
                Rating::Oxygen => std::mem::swap(&mut ones, &mut iterators),
                Rating::Scrubber => std::mem::swap(&mut zeros, &mut iterators),
            },
            Ordering::Greater => match kind {
                Rating::Oxygen => std::mem::swap(&mut zeros, &mut iterators),
                Rating::Scrubber => std::mem::swap(&mut ones, &mut iterators),
            },
        }

        ones.clear();
        zeros.clear();
    }

    iterators.first().unwrap().0
}

fn part_two(input: &str) -> i32 {
    let lines: Vec<_> = input.lines().collect();

    let oxygen_index = find_rating(Rating::Oxygen, &lines);
    let scrubber_index = find_rating(Rating::Scrubber, &lines);

    match (lines.get(oxygen_index), lines.get(scrubber_index)) {
        (Some(oxygen_string), Some(scrubber_string)) => {
            bin_to_dec(oxygen_string) * bin_to_dec(scrubber_string)
        }
        _ => unreachable!(),
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

    time_it(|| println!("part 1: {}", part_one(&input)));
    time_it(|| println!("part 2: {}", part_two(&input)));

    Ok(())
}
