use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

fn part_one(input: &str) -> String {
    let mut recipes = vec![3, 7];

    let mut elf_one = 0usize;
    let mut elf_two = 1usize;

    let amount = input.parse().unwrap();

    while recipes.len() < amount + 10 {
        match (recipes.get(elf_one), recipes.get(elf_two)) {
            (Some(&value_one), Some(&value_two)) => {
                let mut sum = value_one + value_two;

                match sum / 10 {
                    0 => (),
                    value => {
                        recipes.push(value);
                        sum %= 10;
                    }
                }

                recipes.push(sum);
                elf_one = (elf_one + 1 + value_one) % recipes.len();
                elf_two = (elf_two + 1 + value_two) % recipes.len();
            }
            _ => unreachable!(),
        }
    }

    recipes[amount..(amount + 10)]
        .iter()
        .flat_map(|v| char::from_digit(*v as u32, 10))
        .collect()
}

fn part_two(input: &str) -> usize {
    let mut recipes = vec![3, 7];

    let mut elf_one = 0usize;
    let mut elf_two = 1usize;

    let target: &[usize] = &input
        .chars()
        .map(|ch| ch.to_digit(10).unwrap() as usize)
        .collect::<Vec<_>>()[..];
    let target_len = target.len();

    loop {
        match (recipes.get(elf_one), recipes.get(elf_two)) {
            (Some(&value_one), Some(&value_two)) => {
                let mut sum = value_one + value_two;

                match sum / 10 {
                    0 => (),
                    value => {
                        recipes.push(value);
                        sum %= 10;
                    }
                }

                recipes.push(sum);
                elf_one = (elf_one + 1 + value_one) % recipes.len();
                elf_two = (elf_two + 1 + value_two) % recipes.len();
            }
            _ => unreachable!(),
        }

        let len = recipes.len();
        if len > target_len + 1 {
            if &recipes[len - target_len..len] == target {
                return len - target_len;
            }

            if &recipes[len - target_len - 1..len - 1] == target {
                return len - target_len - 1;
            }
        }
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

fn main() -> std::io::Result<()> {
    let file_path = env::current_dir()?.join(Path::new("data.txt"));
    let input = fs::read_to_string(file_path)?;

    time_it(|| println!("part 1: {}", part_one(&input)));
    time_it(|| println!("part 2: {}", part_two(&input)));

    Ok(())
}
