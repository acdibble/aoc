use fraction::Fraction;
use regex::Regex;
use std::time::SystemTime;

const DATA: &'static str = include_str!("../data.txt");

#[derive(Debug, Clone, Copy)]
struct Button {
    dx: i32,
    dy: i32,
}

impl From<Button> for Fraction {
    fn from(value: Button) -> Self {
        Self::from(value.dy) / value.dx
    }
}

impl From<&str> for Button {
    fn from(value: &str) -> Self {
        let button_re = Regex::new(r"Button [AB]: X\+(\d+), Y\+(\d+)").unwrap();
        let caps = button_re.captures(value).unwrap();
        Button {
            dx: caps.get(1).unwrap().as_str().parse().unwrap(),
            dy: caps.get(2).unwrap().as_str().parse().unwrap(),
        }
    }
}

fn parse_data() -> impl Iterator<Item = (Button, Button, (u64, u64))> {
    let prize_re = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

    DATA.trim().split("\n\n").map(move |game| {
        let mut split = game.split("\n");
        let a = Button::from(split.next().unwrap());
        let b = Button::from(split.next().unwrap());
        let caps = prize_re.captures(split.next().unwrap()).unwrap();
        (
            a,
            b,
            (
                caps.get(1).unwrap().as_str().parse().unwrap(),
                caps.get(2).unwrap().as_str().parse().unwrap(),
            ),
        )
    })
}

fn solve<F>(fun: F) -> u64
where
    F: Fn((u64, u64)) -> (u64, u64),
{
    parse_data()
        .map(|(a, b, target)| (a, b, fun(target)))
        .flat_map(|(a, b, target)| {
            let a_y_intercept = find_y_intercept(a, (0, 0));
            let b_y_intercept = find_y_intercept(b, target);

            let x = (b_y_intercept - a_y_intercept) / (Fraction::from(a) - Fraction::from(b));
            if x.denom().copied()? != 1 {
                return None;
            }

            let a_presses = x / a.dx;
            if a_presses.denom().copied()? != 1 {
                return None;
            }

            let b_presses = (Fraction::from(target.0) - x) / b.dx;
            if b_presses.denom().copied()? != 1 {
                return None;
            }

            debug_assert!(a_presses * a.dy + b_presses * b.dy == Fraction::from(target.1));

            Some(a_presses.numer().copied()? * 3 + b_presses.numer().copied()? * 1)
        })
        .sum()
}

fn part_one() -> u64 {
    solve(|el| el)
}

fn find_y_intercept(button: Button, target: (u64, u64)) -> Fraction {
    Fraction::from(target.1) - Fraction::from(button.dy) * target.0 / button.dx
}

fn part_two() -> u64 {
    solve(|el| (el.0 + 10000000000000, el.1 + 10000000000000))
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
