use std::time::SystemTime;

const DATA: &'static str = include_str!("../data.txt");

fn sum_lines<F: Fn((usize, char), &str) -> Option<u32>>(fun: F) -> u32 {
    let mut sum = 0;

    for line in DATA.trim().lines() {
        let mut it = line.char_indices().flat_map(|ch| fun(ch, line));

        let first = it.next().unwrap();
        let last = it.last().unwrap_or(first);
        sum += first * 10 + last
    }

    sum
}

fn part_one() -> u32 {
    sum_lines(|(_, ch), _| ch.to_digit(10))
}

fn part_two() -> u32 {
    sum_lines(|(index, ch), line| {
        if let Some(value) = ch.to_digit(10) {
            return Some(value);
        }

        let line = &line[index..];
        match ch {
            'o' if line.starts_with("one") => Some(1),
            't' if line.starts_with("two") => Some(2),
            't' if line.starts_with("three") => Some(3),
            'f' if line.starts_with("four") => Some(4),
            'f' if line.starts_with("five") => Some(5),
            's' if line.starts_with("six") => Some(6),
            's' if line.starts_with("seven") => Some(7),
            'e' if line.starts_with("eight") => Some(8),
            'n' if line.starts_with("nine") => Some(9),
            _ => None,
        }
    })
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
