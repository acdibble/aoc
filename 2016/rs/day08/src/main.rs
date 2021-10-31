use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

struct Screen([[char; 50]; 6]);

#[derive(Debug)]
enum RotationType {
    Row,
    Column,
}

#[derive(Debug)]
enum Command {
    Rect(usize, usize),
    Rotate(RotationType, usize, usize),
}

impl Screen {
    fn new() -> Self {
        Screen([['.'; 50]; 6])
    }

    fn print(&self) {
        println!("{:=<50}", "");
        for line in self.0 {
            for pixel in line {
                print!("{}", pixel);
            }
            println!("");
        }
        println!("{:=<50}", "")
    }

    fn process(&mut self, line: &str) {
        let mut it = line.split_ascii_whitespace();

        let command = match it.next().unwrap() {
            "rect" => {
                let mut dimesions = it.next().unwrap().split("x");
                let x = dimesions.next().unwrap().parse().unwrap();
                let y = dimesions.next().unwrap().parse().unwrap();
                Command::Rect(x, y)
            }
            "rotate" => {
                let rotation_type = match it.next().unwrap() {
                    "row" => RotationType::Row,
                    "column" => RotationType::Column,
                    _ => unreachable!(),
                };

                let number = it.next().unwrap()[2..].parse().unwrap();
                it.next();
                let amount = it.next().unwrap().parse().unwrap();

                Command::Rotate(rotation_type, number, amount)
            }
            _ => unreachable!(),
        };

        match command {
            Command::Rect(x, y) => self.make_rectangle(x, y),
            Command::Rotate(RotationType::Row, number, amount) => self.rotate_row(number, amount),
            Command::Rotate(RotationType::Column, number, amount) => {
                self.rotate_column(number, amount)
            }
        }
    }

    fn make_rectangle(&mut self, x: usize, y: usize) {
        for y in 0..y {
            for x in 0..x {
                self.0[y][x] = '#'
            }
        }
    }

    fn rotate_row(&mut self, number: usize, amount: usize) {
        let row = &mut self.0[number];
        let row_len = row.len();
        for _ in 0..amount {
            let temp = row[row_len - 1];
            for i in (0..row_len - 1).rev() {
                let copy = row[i];
                row[i + 1] = copy;
            }
            row[0] = temp;
        }
    }

    fn rotate_column(&mut self, number: usize, amount: usize) {
        let row_count = self.0.len();
        for _ in 0..amount {
            let temp = self.0[row_count - 1][number];

            for i in (0..row_count - 1).rev() {
                let copy = self.0[i][number];
                self.0[i + 1][number] = copy;
            }

            self.0[0][number] = temp;
        }
    }
}

fn process(input: &String) -> (i32, Screen) {
    let mut screen = Screen::new();
    for line in input.lines() {
        screen.process(line);
    }

    let mut total = 0;

    for line in &screen.0 {
        for c in line {
            if *c == '#' {
                total += 1;
            }
        }
    }

    (total, screen)
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

    time_it(|| {
        let (amount, screen) = process(&input);
        println!("part 1: {}", amount);
        println!("part 2:");
        screen.print();
    });
    // time_it(|| println!("part 2: {}", part_two(&input)));

    Ok(())
}
