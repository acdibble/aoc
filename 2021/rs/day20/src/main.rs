use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

fn char_to_usize(ch: char) -> usize {
    match ch {
        '.' => 0,
        _ => 1,
    }
}

struct Picture {
    data: HashMap<(i32, i32), usize>,
    max_x: i32,
    max_y: i32,
    min_x: i32,
    min_y: i32,
    default: usize,
    alg: Vec<usize>,
}

impl Picture {
    fn from_str(string: &str) -> Self {
        let mut lines = string.lines();
        let alg = lines.next().unwrap().chars().map(char_to_usize).collect();
        lines.next();

        let data: HashMap<(i32, i32), _> = lines
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .map(char_to_usize)
                    .enumerate()
                    .map(move |(x, p)| ((x as i32, y as i32), p))
            })
            .collect();

        let mut max_x = 0;
        let mut max_y = 0;

        for i in 0.. {
            if data.contains_key(&(i, i)) {
                max_x = i;
                max_y = i;
            } else {
                break;
            }
        }

        Self {
            data,
            max_x,
            max_y,
            min_x: 0,
            min_y: 0,
            default: 0,
            alg,
        }
    }

    fn get_pixel(&self, x: i32, y: i32) -> usize {
        *self.data.get(&(x, y)).unwrap_or(&self.default)
    }

    fn grow(&mut self) {
        let mut new_data = HashMap::new();
        for y in (self.min_y - 1)..=self.max_y + 1 {
            for x in (self.min_x - 1)..=self.max_x + 1 {
                let mut index = 0;

                for dy in (y - 1)..=(y + 1) {
                    for dx in (x - 1)..=(x + 1) {
                        index = (index << 1) | self.get_pixel(dx, dy);
                    }
                }

                new_data.insert(
                    (x, y),
                    match self.alg.get(index) {
                        Some(pixel) => *pixel,
                        _ => unreachable!(),
                    },
                );
            }
        }

        self.default ^= 1;
        self.min_x -= 1;
        self.min_y -= 1;
        self.max_x += 1;
        self.max_y += 1;
        self.data = new_data;
    }
}

fn grow(input: &str, rounds: i32) -> usize {
    let mut picture = Picture::from_str(input);

    for _ in 0..rounds {
        picture.grow();
    }

    picture.data.values().sum()
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

    time_it(|| println!("part 1: {}", grow(&input, 2)));
    time_it(|| println!("part 2: {}", grow(&input, 50)));

    Ok(())
}
