use std::time::SystemTime;

const DATA: &'static str = include_str!("../data.txt");

fn all_unique_chars<const N: usize>(slice: &[char; N]) -> bool {
    for i in 0..N {
        for j in (i + 1)..N {
            if slice[i] == slice[j] {
                return false;
            }
        }
    }

    true
}

fn find_unique_sequence<const N: usize>() -> usize {
    let mut buffer = ['\0'; N];

    for (index, ch) in DATA.char_indices() {
        buffer[index % N] = ch;

        if index >= N && all_unique_chars(&buffer) {
            return index + 1;
        }
    }

    unreachable!()
}

fn part_one() -> usize {
    find_unique_sequence::<4>()
}

fn part_two() -> usize {
    find_unique_sequence::<14>()
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
