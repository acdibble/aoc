use std::time::SystemTime;

const DATA: &'static str = include_str!("../data.txt");

#[derive(Debug)]
struct Value {
    number: i64,
    index: usize,
}

impl Value {
    fn from((index, number): (usize, i64)) -> Self {
        Self { number, index }
    }
}

fn decode(key: i64, rounds: usize) -> i64 {
    let mut numbers: Vec<_> = DATA
        .lines()
        .flat_map(|n| n.parse())
        .map(|n: i64| n * key)
        .enumerate()
        .map(Value::from)
        .collect();
    let len = numbers.len();

    for _ in 0..rounds {
        for next_index in 0..numbers.len() {
            let current_index = numbers
                .iter()
                .position(|value| value.index == next_index)
                .unwrap();

            let value = numbers.remove(current_index);
            let new_index =
                (current_index as i64 + value.number).rem_euclid(len as i64 - 1) as usize;

            numbers.insert(new_index, value);
        }
    }

    let index_of_zero = numbers.iter().position(|val| val.number == 0).unwrap();
    let a = (index_of_zero + 1000) % len;
    let b = (index_of_zero + 2000) % len;
    let c = (index_of_zero + 3000) % len;

    numbers[a].number + numbers[b].number + numbers[c].number
}

fn part_one() -> i64 {
    decode(1, 1)
}

fn part_two() -> i64 {
    decode(811589153, 10)
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
