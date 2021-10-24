use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::Path;

#[derive(Hash, Eq, PartialEq)]
struct House(i32, i32);

fn part_one(directions: &String) -> usize {
    let mut houses = HashSet::new();
    let mut x = 0;
    let mut y = 0;

    for dir in directions.chars() {
        match dir {
            'v' => y -= 1,
            '^' => y += 1,
            '>' => x += 1,
            '<' => x -= 1,
            _ => (),
        }

        houses.insert(House(x, y));
    }

    houses.len()
}

struct Santa(i32, i32);

fn part_two(directions: &String) -> usize {
    let mut houses = HashSet::new();
    let mut santa = Santa(0, 0);
    let mut robo = Santa(0, 0);
    let mut turn = 0;

    for dir in directions.chars() {
        let person = if turn == 0 { &mut santa } else { &mut robo };
        turn ^= 1;

        match dir {
            'v' => person.1 -= 1,
            '^' => person.1 += 1,
            '>' => person.0 += 1,
            '<' => person.0 -= 1,
            _ => (),
        }

        houses.insert(House(person.0, person.1));
    }

    houses.len()
}

fn main() -> std::io::Result<()> {
    let file_path = env::current_dir()?.join(Path::new("data.txt"));
    let directions = fs::read_to_string(file_path)?;

    println!("part 1: {}", part_one(&directions));
    println!("part 2: {}", part_two(&directions));

    Ok(())
}
