use std::env;
use std::fs;
use std::path::Path;

#[derive(Clone)]
struct Password(Vec<u8>);

impl std::fmt::Debug for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(std::str::from_utf8(&self.0).unwrap())
    }
}

impl Password {
    fn from(string: String) -> Password {
        Password(string.as_bytes().to_owned())
    }

    fn increment(&mut self) {
        for byte in self.0.iter_mut().rev() {
            *byte += 1;
            if *byte > b'z' {
                *byte = b'a';
                continue;
            } else if matches!(*byte, b'i' | b'o' | b'l') {
                *byte += 1;
            }

            break;
        }

        let mut should_fill = false;
        for byte in self.0.iter_mut() {
            if should_fill {
                *byte = b'a';
            } else if matches!(*byte, b'i' | b'o' | b'l') {
                *byte += 1;
                should_fill = true;
            }
        }
    }

    fn is_valid(&self) -> bool {
        let mut has_run = false;

        for i in 0..(self.0.len() - 2) {
            if self.0[i] + 1 == self.0[i + 1] && self.0[i] + 2 == self.0[i + 2] {
                has_run = true;
                break;
            }
        }

        if !has_run {
            return false;
        }

        let mut double_index = None;

        for i in 0..(self.0.len() - 1) {
            if self.0[i] == self.0[i + 1] {
                if double_index.is_none() {
                    double_index = Some(i);
                } else if i - double_index.unwrap() >= 2 {
                    return true;
                }
            }
        }

        false
    }
}

fn part_one(mut password: Password) -> String {
    while !password.is_valid() {
        password.increment();
    }

    format!("{:?}", password)
}

fn part_two(mut password: Password) -> String {
    password.increment();

    while !password.is_valid() {
        password.increment();
    }

    format!("{:?}", password)
}

fn main() -> std::io::Result<()> {
    let file_path = env::current_dir()?.join(Path::new("data.txt"));
    let input = Password::from(fs::read_to_string(file_path)?);

    let result = part_one(input);
    println!("part 1: {}", &result);
    let input_2 = Password::from(result);
    println!("part 2: {}", part_two(input_2));

    Ok(())
}
