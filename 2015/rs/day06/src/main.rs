use std::env;
use std::fs;
use std::path::Path;

fn str_to_usize(str: &str) -> usize {
    str.parse().expect("could not convert str to usize")
}

type Coords = (usize, usize);

fn parse_coords(coords: Option<&str>) -> Result<Coords, ()> {
    let mut parts = coords.expect("coords were empty").split(',');

    let x = parts.next().map(str_to_usize).unwrap();
    let y = parts.next().map(str_to_usize).unwrap();

    Ok((x, y))
}

#[derive(Debug)]
enum Action {
    Toggle,
    TurnOn,
    TurnOff,
}

struct Instruction {
    action: Action,
    start: Coords,
    end: Coords,
}

fn parse_instruction(input: &str) -> Instruction {
    let mut parts = input.split(' ');

    let action = match parts.next().expect("failed to retrieve action") {
        "turn" => match parts.next() {
            Some("on") => Action::TurnOn,
            Some("off") => Action::TurnOff,
            _ => unreachable!("mode was not on or off"),
        },
        "toggle" => Action::Toggle,
        _ => unreachable!("action was not turn or toggle"),
    };

    let start = parse_coords(parts.next()).expect("failed to parse start");
    parts.next().expect("consuming through failed");
    let end = parse_coords(parts.next()).expect("failed to parse end");

    Instruction { action, start, end }
}

#[derive(Copy, Clone)]
enum State {
    Off,
    On,
}

fn part_one(input: &Vec<Instruction>) -> i32 {
    let mut grid = [[State::Off; 1000]; 1000];
    let mut lit = 0;

    for inst in input {
        let fun: fn(State) -> State = match inst.action {
            Action::Toggle => |state| match state {
                State::On => State::Off,
                _ => State::On,
            },
            Action::TurnOff => |_| State::Off,
            Action::TurnOn => |_| State::On,
        };

        for x in inst.start.0..=inst.end.0 {
            for y in inst.start.1..=inst.end.1 {
                let row = grid.get_mut(y).unwrap();
                let light = row.get_mut(x).unwrap();
                let new_state = fun(*light);
                match (*light, new_state) {
                    (State::On, State::Off) => lit -= 1,
                    (State::Off, State::On) => lit += 1,
                    _ => (),
                }
                *light = new_state;
            }
        }
    }

    lit
}

fn part_two(input: &Vec<Instruction>) -> i32 {
    let mut grid = [[0; 1000]; 1000];
    let mut lit = 0;

    for inst in input {
        let modifier: i32 = match inst.action {
            Action::TurnOn => 1,
            Action::TurnOff => -1,
            Action::Toggle => 2,
        };

        for x in inst.start.0..=inst.end.0 {
            for y in inst.start.1..=inst.end.1 {
                let row = grid.get_mut(y).unwrap();
                let light = row.get_mut(x).unwrap();
                let old = *light;
                *light = match *light + modifier {
                    n if n < 0 => 0,
                    n => n,
                };
                lit += *light - old;
            }
        }
    }

    lit
}

fn main() -> std::io::Result<()> {
    let file_path = env::current_dir()?.join(Path::new("data.txt"));
    let input: Vec<Instruction> = fs::read_to_string(file_path)?
        .lines()
        .map(parse_instruction)
        .collect();

    println!("part 1: {}", part_one(&input));
    println!("part 2: {}", part_two(&input));

    Ok(())
}
