use crypto::digest::Digest;
use crypto::md5::Md5;
use std::cmp;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

fn compute_md5(string: &str) -> String {
    let mut hash = Md5::new();
    hash.input_str(string);
    hash.result_str()
}

fn part_one(input: &String) -> String {
    let mut min_steps = i32::MAX;
    let mut shortest = String::new();

    let mut base = input.clone();

    let mut queue = VecDeque::new();
    queue.push_back((String::new(), (0, 0), 0));

    while let Some((mut path, (x, y), steps)) = queue.pop_front() {
        if x == 3 && y == 3 {
            min_steps = cmp::min(min_steps, steps);
            shortest = path;
            continue;
        }

        if steps > min_steps {
            continue;
        }

        base.push_str(&path);

        for tuple in compute_md5(&base).chars().enumerate().take(4) {
            let next_char;
            let mut next_x = x;
            let mut next_y = y;

            match tuple {
                (0, 'b'..='f') => {
                    next_y = cmp::max(0, y - 1);
                    next_char = 'U';
                }
                (1, 'b'..='f') => {
                    next_y = cmp::min(3, y + 1);
                    next_char = 'D';
                }
                (2, 'b'..='f') => {
                    next_x = cmp::max(0, x - 1);
                    next_char = 'L';
                }
                (3, 'b'..='f') => {
                    next_x = cmp::min(3, x + 1);
                    next_char = 'R';
                }
                _ => continue,
            }

            if next_x == x && next_y == y {
                continue;
            }

            path.push(next_char);
            queue.push_back((path.clone(), (next_x, next_y), steps + 1));
            path.pop();
        }

        base.replace_range(input.len().., "");
    }

    shortest
}

#[test]
fn test_part_one() {
    assert_eq!(part_one(&"ihgpwlah".to_owned()), "DDRRRD");
    assert_eq!(part_one(&"kglvqrro".to_owned()), "DDUDRLRRUDRD");
    assert_eq!(
        part_one(&"ulqzkmiv".to_owned()),
        "DRURDRUDDLLDLUURRDULRLDUUDDDRR"
    );
}

fn part_two(input: &String) -> i32 {
    let mut max_steps = 0;

    let mut base = input.clone();

    let mut queue = VecDeque::new();
    queue.push_back((String::new(), (0, 0), 0));

    while let Some((mut path, (x, y), steps)) = queue.pop_front() {
        if x == 3 && y == 3 {
            max_steps = cmp::max(max_steps, steps);
            continue;
        }

        base.push_str(&path);

        for tuple in compute_md5(&base).chars().enumerate().take(4) {
            let next_char;
            let mut next_x = x;
            let mut next_y = y;

            match tuple {
                (0, 'b'..='f') => {
                    next_y = cmp::max(0, y - 1);
                    next_char = 'U';
                }
                (1, 'b'..='f') => {
                    next_y = cmp::min(3, y + 1);
                    next_char = 'D';
                }
                (2, 'b'..='f') => {
                    next_x = cmp::max(0, x - 1);
                    next_char = 'L';
                }
                (3, 'b'..='f') => {
                    next_x = cmp::min(3, x + 1);
                    next_char = 'R';
                }
                _ => continue,
            }

            if next_x == x && next_y == y {
                continue;
            }

            path.push(next_char);
            queue.push_back((path.clone(), (next_x, next_y), steps + 1));
            path.pop();
        }

        base.replace_range(input.len().., "");
    }

    max_steps
}

#[test]
fn test_part_two() {
    assert_eq!(part_two(&"ihgpwlah".to_owned()), 370);
    assert_eq!(part_two(&"kglvqrro".to_owned()), 492);
    assert_eq!(part_two(&"ulqzkmiv".to_owned()), 830);
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
