use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

fn part_one(input: &str) -> i32 {
    let target: i32 = input.parse().unwrap();
    let mut starting_point = (0, 0);
    let mut current_value = 1;
    let mut edge_length = 1;

    for i in 0i32.. {
        let area = (1 + i * 2).pow(2);
        if area >= target {
            starting_point = (i, -i);
            current_value = area;
            edge_length = 1 + i * 2;
            break;
        }
    }

    let mut subtractions = 0;
    while target < current_value {
        subtractions += 1;
        match subtractions {
            1 | 3 => {
                starting_point = (-starting_point.0, starting_point.1);
                current_value -= edge_length - 1;
            }
            2 | 4 => {
                starting_point = (starting_point.0, -starting_point.1 + subtractions / 4);
                current_value -= edge_length - subtractions / 2;
            }
            _ => unreachable!(),
        }
    }

    let diff = target - current_value;
    match subtractions {
        0 => (),
        1 => starting_point = (starting_point.0 + diff, starting_point.1),
        2 => starting_point = (starting_point.0, starting_point.1 - diff),
        3 => starting_point = (starting_point.0 - diff, starting_point.1),
        4 => starting_point = (starting_point.0, starting_point.1 + diff),
        _ => unreachable!(),
    }

    starting_point.0.abs() + starting_point.1.abs()
}

const NEIGHBORS: [(i32, i32); 8] = [
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1),
    (1, 1),
    (-1, 1),
    (-1, -1),
    (1, -1),
];

struct Neighbors {
    coord: Coord,
    index: usize,
}

impl Iterator for Neighbors {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        let offset = NEIGHBORS.get(self.index)?;
        self.index += 1;
        Some(Coord(self.coord.0 + offset.0, self.coord.1 + offset.1))
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Coord(i32, i32);

impl Coord {
    fn neighbors(&self) -> Neighbors {
        Neighbors {
            coord: *self,
            index: 0,
        }
    }

    fn get_value(&self, grid: &HashMap<Coord, i32>) -> i32 {
        let mut location_value = 0;

        for neighbor in self.neighbors() {
            location_value += match grid.get(&neighbor) {
                Some(value) => *value,
                _ => 0,
            };
        }

        location_value
    }
}

fn part_two(input: &str) -> i32 {
    let target = input.parse().unwrap();
    let mut current_location = Coord(0, 0);
    let mut grid = HashMap::from([(current_location, 1i32)]);

    for i in 1i32.. {
        current_location.0 += 1;
        let new_value = current_location.get_value(&grid);
        if new_value > target {
            return new_value;
        }
        grid.insert(current_location, new_value);

        while current_location.1 != i {
            current_location.1 += 1;
            let new_value = current_location.get_value(&grid);
            if new_value > target {
                return new_value;
            }
            grid.insert(current_location, new_value);
        }

        while current_location.0 != -i {
            current_location.0 -= 1;
            let new_value = current_location.get_value(&grid);
            if new_value > target {
                return new_value;
            }
            grid.insert(current_location, new_value);
        }

        while current_location.1 != -i {
            current_location.1 -= 1;
            let new_value = current_location.get_value(&grid);
            if new_value > target {
                return new_value;
            }
            grid.insert(current_location, new_value);
        }

        while current_location.0 != i {
            current_location.0 += 1;
            let new_value = current_location.get_value(&grid);
            if new_value > target {
                return new_value;
            }
            grid.insert(current_location, new_value);
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use crate::part_one;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("1"), 0);
        assert_eq!(part_one("12"), 3);
        assert_eq!(part_one("23"), 2);
        assert_eq!(part_one("1024"), 31);
    }
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
