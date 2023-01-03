use std::{fmt::Display, ops::Add, time::SystemTime};

const DATA: &'static str = include_str!("../data.txt");

#[derive(Debug)]
struct Snafu {
    digits: Vec<i64>,
}

impl Snafu {
    fn from(digits: &str) -> Self {
        Self {
            digits: digits
                .chars()
                .map(|ch| match ch {
                    '2' => 2,
                    '1' => 1,
                    '0' => 0,
                    '-' => -1,
                    '=' => -2,
                    _ => unreachable!(),
                })
                .collect(),
        }
    }
}

impl Add<Snafu> for Snafu {
    type Output = Self;

    fn add(self, other: Snafu) -> Self::Output {
        let mut result = Snafu {
            digits: Vec::with_capacity(self.digits.len().max(other.digits.len())),
        };

        let mut iter_a = self.digits.iter().rev();
        let mut iter_b = other.digits.iter().rev();
        let mut carry = 0;

        let mut can_loop = true;
        while can_loop {
            let mut sum = match (iter_a.next(), iter_b.next()) {
                (Some(a), Some(b)) => a + b,
                (None, Some(value)) | (Some(value), None) => *value,
                (None, None) => {
                    can_loop = false;
                    0
                }
            } + carry;

            carry = if sum > 2 {
                sum -= 5;
                1
            } else if sum < -2 {
                sum += 5;
                -1
            } else {
                0
            };

            if can_loop || sum != 0 {
                result.digits.push(sum);
            }
        }

        result.digits.reverse();

        result
    }
}

impl Display for Snafu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.digits.len() == 0 {
            write!(f, "0")?;
        } else {
            for digit in &self.digits {
                write!(
                    f,
                    "{}",
                    match *digit {
                        -2 => '=',
                        -1 => '-',
                        0 => '0',
                        1 => '1',
                        2 => '2',
                        _ => unreachable!(),
                    }
                )?;
            }
        }

        Ok(())
    }
}

fn part_one() -> Snafu {
    DATA.lines()
        .map(Snafu::from)
        .fold(Snafu::from("0"), |acc, snafu| acc + snafu)
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
}
