use std::time::SystemTime;

const DATA: &'static str = include_str!("../data.txt");

fn parse_sequences() -> Vec<Vec<i32>> {
    DATA.trim()
        .lines()
        .map(|l| l.split_ascii_whitespace().flat_map(|n| n.parse()).collect())
        .collect()
}

fn get_differences(seq: &Vec<i32>) -> Vec<i32> {
    seq.windows(2).map(|window| window[1] - window[0]).collect()
}

fn build_sequences(start: Vec<i32>) -> Vec<Vec<i32>> {
    let mut current = get_differences(&start);
    let mut subsequences = vec![start];

    loop {
        let next = get_differences(&current);
        subsequences.push(current);
        if next.iter().all(|n| *n == 0) {
            break;
        }
        current = next;
    }

    subsequences
}

fn calculate_total<F: Copy + Fn(i32, Vec<i32>) -> i32>(fun: F) -> i32 {
    parse_sequences().into_iter().fold(0, |total, seq| {
        total + build_sequences(seq).into_iter().rev().fold(0, fun)
    })
}

fn part_one() -> i32 {
    calculate_total(|acc, seq| acc + seq.last().unwrap())
}

fn part_two() -> i32 {
    calculate_total(|acc, seq| seq.first().unwrap() - acc)
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
