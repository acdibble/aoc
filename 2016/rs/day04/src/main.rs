use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

#[derive(Debug)]
struct RoomName<'a> {
    raw: &'a str,
    char_counts: HashMap<char, i32>,
    sector_id: i32,
    checksum: &'a str,
}

impl<'a> RoomName<'a> {
    fn from(line: &'a str) -> Self {
        let mut frequencies = HashMap::new();
        let mut sector_id = String::new();
        let mut it = line.chars().enumerate();
        while let Some((_, c)) = it.next() {
            match c {
                'a'..='z' => *frequencies.entry(c).or_default() += 1,
                '0'..='9' => sector_id.push(c),
                '[' => break,
                _ => (),
            }
        }

        let (checksum_start, _) = it.next().unwrap();

        Self {
            raw: line,
            char_counts: frequencies,
            sector_id: sector_id.parse().unwrap(),
            checksum: &line[checksum_start..checksum_start + 5],
        }
    }

    fn is_valid(&self) -> bool {
        let mut entries: Vec<(char, i32)> =
            self.char_counts.iter().map(|(k, v)| (*k, *v)).collect();
        entries.sort_by(|a, b| match b.1.cmp(&a.1) {
            result @ (Ordering::Less | Ordering::Greater) => result,
            _ => a.0.cmp(&b.0),
        });

        for (index, c) in self.checksum.chars().enumerate() {
            if entries[index].0 != c {
                return false;
            }
        }

        true
    }

    fn decrypt(&self) -> String {
        let mut result = String::new();

        for mut c in self.raw.chars() {
            if matches!(c, '0'..='9') {
                break;
            }
            for _ in 0..self.sector_id {
                c = match c {
                    'a'..='y' => (c as u8 + 1) as char,
                    'z' => 'a',
                    '-' => ' ',
                    _ => c,
                };
            }

            result.push(c)
        }

        result
    }
}

fn part_one(input: &Vec<RoomName>) -> i32 {
    input.iter().fold(0, |acc, name| {
        acc + if name.is_valid() { name.sector_id } else { 0 }
    })
}

fn part_two(input: &Vec<RoomName>) -> i32 {
    for room_name in input {
        if room_name.is_valid() {
            let decrypted = room_name.decrypt();
            if decrypted.starts_with("northpole object storage") {
                return room_name.sector_id;
            }
        }
    }

    unreachable!()
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

    let room_names = time_it(|| input.lines().map(RoomName::from).collect());

    time_it(|| println!("part 1: {}", part_one(&room_names)));
    time_it(|| println!("part 2: {}", part_two(&room_names)));

    Ok(())
}
