use std::time::SystemTime;

const DATA: &'static str = include_str!("../data.txt");

fn part_one() -> i32 {
    let mut score = 0;

    for line in DATA.lines() {
        let mut parts = line.split_ascii_whitespace();
        let opponent = parts.next().unwrap().chars().next().unwrap();
        let me = parts.next().unwrap().chars().next().unwrap();

        match me {
            'X' => {
                score += 1 + match opponent {
                    'A' => 3,
                    'B' => 0,
                    'C' => 6,
                    _ => unreachable!(),
                }
            }
            'Y' => {
                score += 2 + match opponent {
                    'A' => 6,
                    'B' => 3,
                    'C' => 0,
                    _ => unreachable!(),
                }
            }
            'Z' => {
                score += 3 + match opponent {
                    'A' => 0,
                    'B' => 6,
                    'C' => 3,
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
    }

    score
}

fn part_two() -> i32 {
    let mut score = 0;

    for line in DATA.lines() {
        let mut parts = line.split_ascii_whitespace();
        let opponent = parts.next().unwrap().chars().next().unwrap();
        let me = parts.next().unwrap().chars().next().unwrap();

        match me {
            'X' => {
                score += match opponent {
                    'A' => 3,
                    'B' => 1,
                    'C' => 2,
                    _ => unreachable!(),
                }
            }
            'Y' => {
                score += 3 + match opponent {
                    'A' => 1,
                    'B' => 2,
                    'C' => 3,
                    _ => unreachable!(),
                }
            }
            'Z' => {
                score += 6 + match opponent {
                    'A' => 2,
                    'B' => 3,
                    'C' => 1,
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
    }

    score
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
