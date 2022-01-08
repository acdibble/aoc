use std::collections::VecDeque;
use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

enum Move {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

fn chars_to_usize(chars: &mut std::str::Chars) -> usize {
    let mut amount = 0;

    while let Some(c) = chars.next() {
        match c {
            '0'..='9' => {
                amount *= 10;
                amount += match c.to_digit(10) {
                    Some(value) => value as usize,
                    _ => unreachable!(),
                }
            }
            _ => break,
        }
    }

    amount
}

impl std::str::FromStr for Move {
    type Err = String;
    fn from_str(string: &str) -> Result<Self, <Self as std::str::FromStr>::Err> {
        let mut chars = string.chars();

        match chars.next() {
            Some('s') => Ok(Self::Spin(chars_to_usize(&mut chars))),
            Some('x') => Ok(Move::Exchange(
                chars_to_usize(&mut chars),
                chars_to_usize(&mut chars),
            )),
            Some('p') => {
                let a = chars.next().ok_or_else(|| string.to_owned())?;
                chars.next();

                Ok(Move::Partner(
                    a,
                    chars.next().ok_or_else(|| string.to_owned())?,
                ))
            }
            _ => Err(string.to_owned()),
        }
    }
}

fn dance(input: &str, rounds: usize) -> String {
    let moves: Vec<Move> = input.split(',').map(|part| part.parse().unwrap()).collect();
    let mut dancers: VecDeque<char> = ('a'..='p').into_iter().collect();

    let mut seen = Vec::from([dancers.iter().collect()]);

    for _ in 0..rounds {
        for mv in &moves {
            match mv {
                &Move::Spin(amount) => {
                    for _ in 0..amount {
                        let temp = dancers.pop_back().unwrap();
                        dancers.push_front(temp);
                    }
                }
                &Move::Exchange(a, b) => dancers.swap(a, b),
                &Move::Partner(a, b) => {
                    let pos_a = dancers.iter().position(|c| *c == a).unwrap();
                    let pos_b = dancers.iter().position(|c| *c == b).unwrap();

                    dancers.swap(pos_a, pos_b)
                }
            }
        }

        let order = dancers.iter().collect();
        if seen.contains(&order) {
            break;
        }
        seen.push(order);
    }

    seen.remove(rounds % seen.len())
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

    time_it(|| println!("part 1: {}", dance(&input, 1)));
    time_it(|| println!("part 2: {}", dance(&input, 1_000_000_000)));

    Ok(())
}
