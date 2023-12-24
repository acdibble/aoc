use std::time::SystemTime;
use utils::{Direction, Point, Translate};

const DATA: &'static str = include_str!("../data.txt");

struct Instruction {
    dir: Direction,
    count: i64,
}

impl Translate<&Instruction> for Point {
    fn translate(&self, change: &Instruction) -> Self {
        let amount = change.count as i32;

        match change.dir {
            Direction::North => self.translate((0, -amount)),
            Direction::South => self.translate((0, amount)),
            Direction::West => self.translate((-amount, 0)),
            Direction::East => self.translate((amount, 0)),
        }
    }
}

fn dig(instructions: &[Instruction]) -> i64 {
    let mut location = Point::from((0, 0));

    let mut area = 0;

    for inst in instructions {
        let next = location.translate(inst);

        area += location.x as i64 * next.y as i64 - next.x as i64 * location.y as i64;
        area += inst.count;

        location = next;
    }

    area / 2 + 1
}

fn part_one() -> i64 {
    let instructions: Vec<_> = DATA
        .trim()
        .lines()
        .map(|line| {
            let mut parts = line.split_ascii_whitespace();
            let dir = match parts.next().unwrap() {
                "U" => Direction::North,
                "R" => Direction::East,
                "D" => Direction::South,
                "L" => Direction::West,
                _ => unreachable!(),
            };
            let count = parts.next().unwrap().parse().unwrap();

            Instruction { dir, count }
        })
        .collect();

    dig(&instructions)
}

fn part_two() -> i64 {
    let instructions: Vec<_> = DATA
        .trim()
        .lines()
        .map(|line| {
            let code = line.split_ascii_whitespace().skip(2).next().unwrap();

            let mut count = 0;
            for n in code.chars().skip(2).take(5) {
                count *= 16;
                count += n.to_digit(16).unwrap() as i64;
            }

            let dir = match &code[7..8] {
                "0" => Direction::East,
                "1" => Direction::South,
                "2" => Direction::West,
                "3" => Direction::North,
                _ => unreachable!(),
            };

            Instruction { dir, count }
        })
        .collect();

    dig(&instructions)
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
