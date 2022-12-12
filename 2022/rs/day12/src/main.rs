use std::time::SystemTime;

const DATA: &'static str = include_str!("../data.txt");

struct Tile {
    ch: char,
    seen: bool,
}

impl Tile {
    fn new(ch: char) -> Self {
        Self { ch, seen: false }
    }
}

fn walk_trail(starting_char: char) -> i32 {
    let mut queue = Vec::new();

    let mut grid: Vec<Vec<_>> = DATA
        .lines()
        .enumerate()
        .map(|(y, line)| {
            let mut tiles = Vec::with_capacity(line.len());

            for (x, ch) in line.char_indices() {
                let mut tile = Tile::new(ch);

                if ch == starting_char {
                    queue.push(((x, y), 0, ch));
                    tile.seen = true;
                }

                tiles.push(tile);
            }

            tiles
        })
        .collect();

    while let Some(((x, y), count, current)) = queue.pop() {
        for (new_x, new_y) in [
            (x + 1, y),
            (x.saturating_sub(1), y),
            (x, y + 1),
            (x, y.saturating_sub(1)),
        ] {
            if let Some(tile) = grid.get_mut(new_y).and_then(|row| row.get_mut(new_x)) {
                if tile.ch == 'E' {
                    return count + 1;
                }

                if tile.seen {
                    continue;
                }

                if current == 'S' || (current as u32 + 1) >= tile.ch as u32 {
                    tile.seen = true;
                    queue.push(((new_x, new_y), count + 1, tile.ch));
                }
            }
        }
    }

    unreachable!()
}

fn part_one() -> i32 {
    walk_trail('S')
}

fn part_two() -> i32 {
    walk_trail('a')
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
