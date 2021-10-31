use std::cmp;
use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

fn part_one(input: &String) -> String {
    const KEYPAD: [[char; 3]; 3] = [['1', '2', '3'], ['4', '5', '6'], ['7', '8', '9']];

    let mut result = String::new();

    let mut x: i32 = 1;
    let mut y: i32 = 1;
    for line in input.lines() {
        for c in line.chars() {
            match c {
                'U' => y = cmp::max(0, y - 1),
                'D' => y = cmp::min(2, y + 1),
                'R' => x = cmp::min(2, x + 1),
                'L' => x = cmp::max(0, x - 1),
                _ => unreachable!(),
            }
        }

        result.push(KEYPAD[y as usize][x as usize]);
    }

    result
}

fn part_two(input: &String) -> String {
    const KEYPAD: [[Option<char>; 5]; 5] = [
        [None, None, Some('1'), None, None],
        [None, Some('2'), Some('3'), Some('4'), None],
        [Some('5'), Some('6'), Some('7'), Some('8'), Some('9')],
        [None, Some('A'), Some('B'), Some('C'), None],
        [None, None, Some('D'), None, None],
    ];
    let mut result = String::new();

    let mut x: i32 = 0;
    let mut y: i32 = 2;
    for line in input.lines() {
        for c in line.chars() {
            match c {
                'U' => {
                    let new_y = cmp::max(0, y - 1);
                    if KEYPAD[new_y as usize][x as usize].is_some() {
                        y = new_y
                    }
                }
                'D' => {
                    let new_y = cmp::min(4, y + 1);
                    if KEYPAD[new_y as usize][x as usize].is_some() {
                        y = new_y
                    }
                }
                'R' => {
                    let new_x = cmp::min(4, x + 1);
                    if KEYPAD[y as usize][new_x as usize].is_some() {
                        x = new_x
                    }
                }
                'L' => {
                    let new_x = cmp::max(0, x - 1);
                    if KEYPAD[y as usize][new_x as usize].is_some() {
                        x = new_x
                    }
                }
                _ => unreachable!(),
            }
        }

        result.push(KEYPAD[y as usize][x as usize].unwrap())
    }

    result
}

fn time_it<F>(fun: F)
where
    F: Fn() -> (),
{
    let start = SystemTime::now();
    fun();
    println!("Time elapsed: {} µs", start.elapsed().unwrap().as_micros())
}

fn main() -> std::io::Result<()> {
    let file_path = env::current_dir()?.join(Path::new("data.txt"));
    let input = fs::read_to_string(file_path)?;

    time_it(|| println!("part 1: {}", part_one(&input)));
    time_it(|| println!("part 2: {}", part_two(&input)));

    Ok(())
}
