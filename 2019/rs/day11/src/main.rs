use intcode::VM;
use std::{collections::HashMap, time::SystemTime};
use utils::grid::{Coordinate, Direction};

const DATA: &'static str = include_str!("../data.txt");

#[derive(Clone, Copy)]
enum Color {
    Black,
    White,
}

impl From<i64> for Color {
    fn from(value: i64) -> Self {
        match value {
            0 => Self::Black,
            1 => Self::White,
            _ => unreachable!(),
        }
    }
}

impl Into<i64> for Color {
    fn into(self) -> i64 {
        match self {
            Self::Black => 0,
            Self::White => 1,
        }
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::White => '#',
                Self::Black => ' ',
            }
        )
    }
}

fn get_panels(starting_color: Color) -> HashMap<Coordinate, Color> {
    let mut vm = VM::from(DATA);

    let mut location = Coordinate::new(0, 0);
    let mut direction = Direction::Up;
    let mut panels = HashMap::new();
    panels.insert(location, starting_color);

    loop {
        let panel = panels.entry(location).or_insert(Color::Black);

        vm.write_input((*panel).into());

        vm.run();

        match (vm.read_output(), vm.read_output()) {
            (Some(color), Some(turn)) => {
                *panel = Color::from(color);
                match turn {
                    0 => direction = direction.left(),
                    1 => direction = direction.right(),
                    _ => unreachable!(),
                }
                location = location.translate(&direction);
            }
            _ => unreachable!(),
        }

        if vm.halted {
            break;
        }
    }

    panels
}

fn part_one() -> usize {
    get_panels(Color::Black).len()
}

fn part_two() {
    let panels = get_panels(Color::White);

    let mut max_x = 0;
    let mut max_y = 0;
    let mut min_x = 0;
    let mut min_y = 0;

    for (key, _) in &panels {
        max_x = max_x.max(key.x);
        max_y = max_y.max(key.y);
        min_x = min_x.min(key.x);
        min_y = min_y.min(key.y);
    }

    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            print!(
                "{}",
                panels.get(&Coordinate::new(x, y)).unwrap_or(&Color::White)
            )
        }

        println!("");
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

fn main() {
    time_it(|| println!("part 1: {}", part_one()));
    time_it(|| println!("part 2: {:?}", part_two()));
}
