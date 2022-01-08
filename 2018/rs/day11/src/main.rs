use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

struct Grid {
    storage: Vec<Vec<i32>>,
    serial_number: i32,
}

impl Grid {
    fn new(capacity: usize, serial_number: i32) -> Self {
        Self {
            serial_number,
            storage: vec![Vec::with_capacity(capacity); capacity],
        }
    }

    fn get_memoized(&mut self, x: usize, y: usize) -> i32 {
        if let Some(num) = self.storage[y].get(x) {
            *num
        } else {
            let amount = self.get_power_level(x, y);
            self.storage[y].push(amount);
            amount
        }
    }
    fn get_power_level(&self, x: usize, y: usize) -> i32 {
        let rack_id = (x + 10) as i32;
        let mut power_level = rack_id * y as i32;
        power_level += self.serial_number;
        power_level *= rack_id;
        power_level = (power_level / 100) % 10;
        power_level - 5
    }
}

fn part_one(input: &str) {
    let mut grid = Grid::new(300, input.parse().unwrap());

    let mut result_x: usize = 0;
    let mut result_y: usize = 0;
    let mut largest_grid = 0;

    for y in 0..297 {
        for x in 0..297 {
            let mut grid_power_level = 0;
            for yy in 0..3 {
                for xx in 0..3 {
                    grid_power_level += grid.get_memoized(x + xx, y + yy);
                }
            }

            if grid_power_level > largest_grid {
                result_x = x;
                result_y = y;
                largest_grid = grid_power_level;
            }
        }
    }

    println!("( {}, {} )", result_x, result_y);
}

fn part_two(input: &str) {
    let mut grid = Grid::new(300, input.parse().unwrap());

    let mut result_x: usize = 0;
    let mut result_y: usize = 0;
    let mut largest_grid = 0;
    let mut result_size = 0;
    let mut unchanged_count = 0;

    for size in 0..300 {
        if unchanged_count == 10 {
            break;
        }
        for y in 0..(300 - size) {
            for x in 0..(300 - size) {
                let mut grid_power_level = 0;
                for yy in 0..size {
                    for xx in 0..size {
                        grid_power_level += grid.get_memoized(x + xx, y + yy);
                    }
                }

                if grid_power_level >= largest_grid {
                    result_x = x;
                    result_y = y;
                    largest_grid = grid_power_level;
                    result_size = size;
                    unchanged_count = 0;
                }
            }
        }
        unchanged_count += 1;
    }

    println!("( {}, {}, {} )", result_x, result_y, result_size);
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

    time_it(|| part_one(&input));
    time_it(|| part_two(&input));

    Ok(())
}
