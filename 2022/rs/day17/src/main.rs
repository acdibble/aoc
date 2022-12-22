use std::time::SystemTime;
use utils::Coord;

const DATA: &'static str = include_str!("../data.txt");

#[derive(Clone, Copy)]
enum Shape {
    Bar,
    Plus,
    El,
    Pipe,
    Square,
}

struct Rock {
    coords: [Coord; 5],
    coord_count: usize,
    height: i64,
}

impl Rock {
    fn new(current_height: i64, shape: Shape) -> Self {
        let lowest_height = current_height + 3;

        let (coords, coord_count, height) = match shape {
            Shape::Bar => (
                [
                    Coord(2, lowest_height),
                    Coord(3, lowest_height),
                    Coord(4, lowest_height),
                    Coord(5, lowest_height),
                    Coord(-1, -1),
                ],
                4,
                1,
            ),
            Shape::Plus => (
                [
                    Coord(3, lowest_height + 2),
                    Coord(2, lowest_height + 1),
                    Coord(3, lowest_height + 1),
                    Coord(4, lowest_height + 1),
                    Coord(3, lowest_height),
                ],
                5,
                3,
            ),
            Shape::El => (
                [
                    Coord(4, lowest_height + 2),
                    Coord(4, lowest_height + 1),
                    Coord(2, lowest_height),
                    Coord(3, lowest_height),
                    Coord(4, lowest_height),
                ],
                5,
                3,
            ),
            Shape::Pipe => (
                [
                    Coord(2, lowest_height + 3),
                    Coord(2, lowest_height + 2),
                    Coord(2, lowest_height + 1),
                    Coord(2, lowest_height),
                    Coord(-1, -1),
                ],
                4,
                4,
            ),
            Shape::Square => (
                [
                    Coord(2, lowest_height + 1),
                    Coord(3, lowest_height + 1),
                    Coord(2, lowest_height),
                    Coord(3, lowest_height),
                    Coord(-1, -1),
                ],
                4,
                2,
            ),
        };

        Self {
            coord_count,
            coords,
            height,
        }
    }

    fn iter(&self) -> impl Iterator<Item = &Coord> {
        self.coords.iter().take(self.coord_count)
    }

    fn can_move_down(&self, shaft: &Vec<[char; 7]>) -> bool {
        for coord in self.iter().map(|coord| coord.translate_y(-1)) {
            if coord.1 < 0 || shaft[coord.1 as usize][coord.0 as usize] == '#' {
                return false;
            }
        }

        true
    }

    fn can_move_left(&self, shaft: &Vec<[char; 7]>) -> bool {
        for coord in self.iter().map(|coord| coord.translate_x(-1)) {
            if coord.0 < 0 || shaft[coord.1 as usize][coord.0 as usize] == '#' {
                return false;
            }
        }

        true
    }

    fn can_move_right(&self, shaft: &Vec<[char; 7]>) -> bool {
        for coord in self.iter().map(|coord| coord.translate_x(1)) {
            if coord.0 > 6 || shaft[coord.1 as usize][coord.0 as usize] == '#' {
                return false;
            }
        }

        true
    }

    fn move_down(&mut self) {
        for coord in self.coords.iter_mut().take(self.coord_count) {
            coord.1 -= 1;
        }
    }

    fn move_left(&mut self) {
        for coord in self.coords.iter_mut().take(self.coord_count) {
            coord.0 -= 1;
        }
    }

    fn move_right(&mut self) {
        for coord in self.coords.iter_mut().take(self.coord_count) {
            coord.0 += 1;
        }
    }
}

const SHAPE_ORDER: [Shape; 5] = [
    Shape::Bar,
    Shape::Plus,
    Shape::El,
    Shape::Pipe,
    Shape::Square,
];

fn drop_rocks(count: usize, print: bool) -> i64 {
    let mut jet = DATA.trim().chars().cycle();
    let mut shape = SHAPE_ORDER.iter().copied().cycle();
    let mut shaft = Vec::from([['.'; 7]; 4]);
    let mut current_height = 0;

    for _ in 0..count {
        let mut rock = Rock::new(current_height, shape.next().unwrap());

        while (shaft.len() as i64) < current_height + 3 + rock.height {
            shaft.push(['.'; 7]);
        }

        loop {
            // for (y, row) in shaft.iter().enumerate().rev() {
            //     for (x, ch) in row.iter().enumerate() {
            //         if rock.coords.contains(&Coord(x as i64, y as i64)) {
            //             print!("@")
            //         } else {
            //             print!("{ch}")
            //         }
            //     }

            //     println!()
            // }
            // println!();

            match jet.next() {
                Some('<') => {
                    if rock.can_move_left(&shaft) {
                        rock.move_left()
                    }
                }
                Some('>') => {
                    if rock.can_move_right(&shaft) {
                        rock.move_right()
                    }
                }
                _ => unreachable!(),
            }

            if !rock.can_move_down(&shaft) {
                break;
            }
            rock.move_down();
        }

        for coord in rock.iter() {
            shaft[coord.1 as usize][coord.0 as usize] = '#';
            current_height = current_height.max(coord.1 + 1);
        }
    }

    if print {
        for row in shaft.iter().rev() {
            for ch in row {
                print!("{ch}")
            }

            println!()
        }
    }

    current_height
}

fn part_one() -> i64 {
    drop_rocks(2022, false)
}

fn part_two() -> usize {
    let runup_block_count = 80;
    let end_of_first_cycle_block_count = 1820;
    let cycle_block_count = end_of_first_cycle_block_count - runup_block_count;

    let runup_height = drop_rocks(80, false);

    let runup_and_first_cycle_height = drop_rocks(end_of_first_cycle_block_count, false);
    let cycle_height = runup_and_first_cycle_height - runup_height;

    let remaining_blocks = 1000000000000usize - 80;

    let remainder = remaining_blocks % cycle_block_count as usize;
    let cycles = remaining_blocks / cycle_block_count;

    let remainder_height = drop_rocks(end_of_first_cycle_block_count + remainder, false)
        - runup_and_first_cycle_height;

    runup_height as usize + (cycle_height as usize * cycles) + remainder_height as usize
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

fn main() {
    time_it(|| println!("part 1: {}", part_one()));
    time_it(|| println!("part 2: {}", part_two()));
}
