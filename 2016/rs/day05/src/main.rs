use crypto::digest::Digest;
use crypto::md5::Md5;
use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

fn compute_md5(string: String) -> String {
    let mut hash = Md5::new();
    hash.input_str(string.as_str());
    hash.result_str()
}

fn part_one(door_id: &String) -> String {
    let mut password = String::new();

    for i in 0.. {
        let result = compute_md5(format!("{}{}", door_id, i));
        if result.starts_with("00000") {
            password.push(result.chars().nth(5).unwrap());

            if password.len() == 8 {
                return password;
            }
        }
    }

    unreachable!()
}

fn part_two(door_id: &String) -> String {
    let mut password = [None; 8];

    for i in 1.. {
        let result = compute_md5(format!("{}{}", door_id, i));
        if result.starts_with("00000") {
            let index = match result.chars().nth(5).unwrap() {
                '0' => 0,
                '1' => 1,
                '2' => 2,
                '3' => 3,
                '4' => 4,
                '5' => 5,
                '6' => 6,
                '7' => 7,
                _ => continue,
            };

            password[index].get_or_insert(result.chars().nth(6).unwrap());

            if password.iter().all(|c| c.is_some()) {
                break;
            }
        }
    }

    password.map(|opt| opt.unwrap()).into_iter().collect()
}

fn time_it<F, T>(fun: F) -> T
where
    F: Fn() -> T,
{
    let start = SystemTime::now();
    let result = fun();
    println!("Time elapsed: {} s", start.elapsed().unwrap().as_secs_f64());
    result
}

fn main() -> std::io::Result<()> {
    let file_path = env::current_dir()?.join(Path::new("data.txt"));
    let input = fs::read_to_string(file_path)?;

    time_it(|| println!("part 1: {}", part_one(&input)));
    time_it(|| println!("part 2: {}", part_two(&input)));

    Ok(())
}
