use intcode::VM;
use std::time::SystemTime;

const DATA: &'static str = include_str!("../data.txt");

#[derive(Debug, Clone, Copy)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl From<i64> for Tile {
    fn from(value: i64) -> Self {
        match value {
            0 => Self::Empty,
            1 => Self::Wall,
            2 => Self::Block,
            3 => Self::Paddle,
            4 => Self::Ball,
            _ => unreachable!(),
        }
    }
}

fn part_one() -> i32 {
    let mut vm = VM::from(DATA);

    vm.run();

    let mut buffer = [[Tile::Empty; 40]; 25];

    loop {
        match (vm.read_output(), vm.read_output(), vm.read_output()) {
            (Some(x), Some(y), Some(value)) => buffer[y as usize][x as usize] = value.into(),
            (None, None, None) => break,
            _ => unreachable!(),
        }
    }

    buffer
        .into_iter()
        .flat_map(|line| {
            line.into_iter().filter_map(|tile| match tile {
                Tile::Block => Some(1),
                _ => None,
            })
        })
        .sum()
}

fn part_two() -> i32 {
    let mut intcodes = VM::parse_intcodes(DATA);
    intcodes[0] = 2;

    let mut vm = VM::from(intcodes);
    let mut buffer = [[Tile::Empty; 40]; 25];

    vm.run();

    let mut score = 0;
    let mut ball_x = 0;
    let mut paddle_x = 0;

    loop {
        while let (Some(x), Some(y), Some(value)) =
            (vm.read_output(), vm.read_output(), vm.read_output())
        {
            if x == -1 && y == 0 {
                score = value as i32;
            } else {
                let tile = value.into();
                match tile {
                    Tile::Ball => ball_x = x,
                    Tile::Paddle => paddle_x = x,
                    _ => {}
                }
                buffer[y as usize][x as usize] = tile;
            }
        }

        if vm.halted {
            break;
        }

        vm.write_input(match ball_x.cmp(&paddle_x) {
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => 1,
        });

        vm.run();
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
