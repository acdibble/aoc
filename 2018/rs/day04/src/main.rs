use chrono::Timelike;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct SleepLog {
    pub asleep_at: chrono::NaiveDateTime,
    pub awake_at: chrono::NaiveDateTime,
}

#[derive(Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
struct GuardLog {
    pub id: i32,
    pub minutes_asleep: i64,
    pub sleep_logs: Vec<SleepLog>,
}

fn parse_date(date: &String) -> chrono::NaiveDateTime {
    chrono::NaiveDateTime::parse_from_str(date, "%Y-%m-%d %H:%M").unwrap()
}

fn parse_logs(lines: Vec<&str>) -> Vec<GuardLog> {
    let mut logs: Vec<GuardLog> = Vec::new();
    let mut date = String::new();
    let mut asleep_at: Option<chrono::NaiveDateTime> = None;
    let mut token = String::new();
    let mut pointer: usize = 0;

    for line in lines {
        token.clear();
        let mut id_found = false;
        for ch in line.chars() {
            match ch {
                '[' => continue,
                '#' => {
                    token.clear();
                    id_found = true;
                }
                ']' => {
                    date = token;
                    token = String::new();
                }
                ' ' => {
                    if id_found {
                        id_found = false;
                        let parsed_id: i32 = token.parse().unwrap();
                        pointer = if let Some(index) = logs.iter().position(|l| l.id == parsed_id) {
                            index
                        } else {
                            logs.push(GuardLog {
                                id: parsed_id,
                                minutes_asleep: 0,
                                sleep_logs: Vec::new(),
                            });
                            logs.len() - 1
                        };
                        token.clear();
                    } else if token.len() > 0 {
                        token.push(ch);
                    }
                }
                _ => {
                    token.push(ch);
                }
            }
        }

        match token.as_str() {
            "falls asleep" => {
                asleep_at = Some(parse_date(&date));
                date.clear();
            }
            "wakes up" => {
                let awake_at = parse_date(&date);
                let mut log = &mut logs[pointer];
                log.minutes_asleep += awake_at
                    .signed_duration_since(asleep_at.unwrap())
                    .num_minutes();
                log.sleep_logs.push(SleepLog {
                    asleep_at: asleep_at.unwrap(),
                    awake_at: awake_at,
                });
                date.clear();
            }
            _ => (),
        }
    }

    logs.sort_by(|a, b| b.minutes_asleep.cmp(&a.minutes_asleep));
    logs
}

fn part_one(input: &str) -> i32 {
    let mut lines: Vec<_> = input.lines().collect();

    lines.sort();

    let mut logs = parse_logs(lines);
    let log = logs.remove(0);

    let mut minute_map: HashMap<u32, i32> = HashMap::new();
    let mut most_frequent: u32 = 60;
    let mut count = 0;

    for sleep_log in log.sleep_logs {
        for minute in sleep_log.asleep_at.minute()..sleep_log.awake_at.minute() {
            let counter = minute_map.entry(minute).or_insert(0);
            *counter += 1;
            if count < *counter {
                most_frequent = minute;
                count = *counter;
            }
        }
    }

    log.id * most_frequent as i32
}

fn part_two(input: &str) -> i32 {
    let mut lines: Vec<_> = input.lines().collect();

    lines.sort();

    let logs = parse_logs(lines);

    let mut output_id: i32 = 0;
    let mut most_frequent: u32 = 60;
    let mut count = 0;

    for log in logs {
        let mut minute_map: HashMap<u32, i32> = HashMap::new();

        for sleep_log in log.sleep_logs {
            for minute in sleep_log.asleep_at.minute()..sleep_log.awake_at.minute() {
                let counter = minute_map.entry(minute).or_insert(0);
                *counter += 1;
                if count < *counter {
                    output_id = log.id;
                    most_frequent = minute;
                    count = *counter;
                }
            }
        }
    }

    output_id * most_frequent as i32
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
