use serde_json::Value;
use std::env;
use std::fs;
use std::path::Path;

fn part_one(input: &Value) -> i64 {
    match input {
        Value::Number(number) => number.as_i64().unwrap(),
        Value::Array(array) => array.iter().fold(0, |acc, value| acc + part_one(value)),
        Value::Object(object) => object.values().fold(0, |acc, value| acc + part_one(value)),
        _ => 0,
    }
}

fn part_two(input: &Value) -> i64 {
    match input {
        Value::Number(number) => number.as_i64().unwrap(),
        Value::Array(array) => array.iter().fold(0, |acc, value| acc + part_two(value)),
        Value::Object(object) => {
            let contains_red = object
                .values()
                .any(|value| matches!(value.as_str(), Some("red")));
            if contains_red {
                object.values().fold(0, |acc, value| acc + part_two(value))
            } else {
                0
            }
        }
        _ => 0,
    }
}

fn main() -> std::io::Result<()> {
    let file_path = env::current_dir()?.join(Path::new("data.txt"));
    let input = fs::read_to_string(file_path)?;
    let json: Value = serde_json::from_str(&input)?;

    println!("part 1: {}", part_one(&json));
    println!("part 2: {}", part_two(&json));

    Ok(())
}
