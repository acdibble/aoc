use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let reader = BufReader::new(File::open("data.txt")?);

    let mut numbers: Vec<i32> = Vec::with_capacity(200);
    for line in reader.lines() {
        numbers.push(line?.parse().unwrap());
    }

    for num in &numbers {
        for num2 in &numbers {
            if num + num2 == 2020 {
                println!("{}", num * num2);
                return Ok(());
            }
        }
    }

    unreachable!()
}
