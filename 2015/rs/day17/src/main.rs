use std::cmp;
use std::env;
use std::fs;
use std::path::Path;

fn find_solutions(sizes: &[i32]) -> (i32, i32) {
    let mut stack = Vec::new();

    let mut i = 0;
    let mut part_one_solutions = 0;
    let mut part_two_solutions = 0;

    let mut min_size = sizes.len();

    loop {
        let sum = stack.iter().map(|(_, value)| value).sum::<i32>();

        match sum.cmp(&150) {
            cmp::Ordering::Equal => {
                part_one_solutions += 1;
                let stack_size = stack.len();

                match stack_size.cmp(&min_size) {
                    cmp::Ordering::Equal => part_two_solutions += 1,
                    cmp::Ordering::Less => {
                        part_two_solutions = 1;
                        min_size = stack_size
                    }
                    cmp::Ordering::Greater => (),
                }

                let (index, _) = stack.pop().unwrap();
                i = index + 1;
            }
            cmp::Ordering::Greater => {
                let (index, _) = stack.pop().unwrap();
                i = index + 1;
            }
            cmp::Ordering::Less => {
                if i == sizes.len() {
                    let (index, _) = stack.pop().unwrap();
                    i = index + 1;
                } else {
                    stack.push((i, sizes[i]));
                    i += 1;
                }
            }
        };

        if i == sizes.len() && stack.is_empty() {
            break;
        }
    }

    (part_one_solutions, part_two_solutions)
}

fn main() -> std::io::Result<()> {
    let file_path = env::current_dir()?.join(Path::new("data.txt"));
    let sizes: Vec<i32> = fs::read_to_string(file_path)?
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect();

    println!("part (1, 2): {:?}", find_solutions(&sizes));

    Ok(())
}
