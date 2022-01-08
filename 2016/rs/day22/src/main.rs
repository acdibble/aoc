use std::collections::{HashSet, VecDeque};
use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

type Grid = Vec<Vec<Node>>;

#[derive(Debug)]
struct Node {
    x: usize,
    y: usize,
    used: usize,
    avail: usize,
    size: usize,
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Node {
    fn new(x: usize, y: usize, used: usize, avail: usize) -> Self {
        Self {
            x,
            y,
            used,
            avail,
            size: used + avail,
        }
    }

    fn is_empty(&self) -> bool {
        self.used == 0
    }

    fn can_contain(&self, other: &Self) -> bool {
        self.avail > other.used
    }

    fn could_contain(&self, other: &Self) -> bool {
        self.size > other.used
    }
}

fn parse_grid(input: &String) -> Vec<Vec<Node>> {
    let mut grid = Vec::new();

    let mut lines = input.lines();

    lines.next();
    lines.next();

    for line in lines {
        let mut words = line.split_ascii_whitespace();

        let mut id_parts = words.next().unwrap().split('-');
        id_parts.next();

        let x: usize = id_parts.next().unwrap()[1..].parse().unwrap();
        let y: usize = id_parts.next().unwrap()[1..].parse().unwrap();

        let row = match grid.get_mut(y) {
            Some(row) => row,
            None => {
                grid.push(Vec::new());
                grid.get_mut(y).unwrap()
            }
        };

        words.next();
        let used = words.next().unwrap();
        let avail = words.next().unwrap();

        row.push(Node::new(
            x,
            y,
            used[..used.len() - 1].parse().unwrap(),
            avail[..avail.len() - 1].parse().unwrap(),
        ))
    }

    grid
}

fn part_one(grid: &Grid) -> i32 {
    let mut viable_pairs = 0;

    for row1 in grid.iter() {
        for node1 in row1 {
            for row2 in grid.iter() {
                for node2 in row2 {
                    if !node1.is_empty() && node1 != node2 && node2.can_contain(&node1) {
                        viable_pairs += 1;
                    }
                }
            }
        }
    }

    viable_pairs
}

struct State {
    x: usize,
    y: usize,
    steps: usize,
}

fn reduce_coord(x_or_y: usize, diff: i32) -> usize {
    match diff {
        -1 => x_or_y.saturating_sub(1),
        0 => x_or_y,
        1 => x_or_y + 1,
        _ => unreachable!(),
    }
}

fn get_next_coords(grid: &Grid, state: &State, x: i32, y: i32) -> Option<(usize, usize)> {
    if x == 0 && y == 0 {
        return None;
    }

    let new_x = reduce_coord(state.x, x);
    let new_y = reduce_coord(state.y, y);

    if new_y == state.y && new_x == state.x {
        return None;
    }

    if let Some(row) = grid.get(new_y) {
        if let Some(neighbor) = row.get(new_x) {
            if grid[state.y][state.x].could_contain(neighbor) {
                return Some((new_x, new_y));
            }
        }
    }

    None
}

const NEIGHBORS: [[i32; 2]; 4] = [[1, 0], [-1, 0], [0, 1], [0, -1]];

fn part_two(grid: Grid) -> usize {
    let mut empty_location = None;

    for row in &grid {
        for node in row {
            if node.used == 0 {
                empty_location = Some((node.x, node.y));
                break;
            }
        }
    }

    let (x, y) = empty_location.expect("failed to find starting location");
    let initial_state = State { x, y, steps: 0 };
    let mut queue = VecDeque::from([initial_state]);
    let mut seen = HashSet::from([(x, y)]);
    let mut steps_taken_to_corner = None;

    let max_x = grid[0].len() - 2;
    while let Some(state) = queue.pop_front() {
        if state.x == max_x && state.y == 0 {
            steps_taken_to_corner = Some(state.steps);
            break;
        }

        for [x, y] in NEIGHBORS {
            if let Some(coords) = get_next_coords(&grid, &state, x, y) {
                let coords = coords;
                if seen.contains(&coords) {
                    continue;
                }
                seen.insert(coords);
                queue.push_back(State {
                    x: coords.0,
                    y: coords.1,
                    steps: state.steps + 1,
                })
            }
        }
    }

    steps_taken_to_corner.expect("failed to find corner") // steps to spot just before corner
        + 1 // move to corner
        + max_x * 5 // it takes 5 steps per movement of the target data
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
    let grid = parse_grid(&input);

    time_it(|| println!("part 1: {}", part_one(&grid)));
    let start = SystemTime::now();
    println!("part 2: {}", part_two(grid));
    println!("Time elapsed: {} µs", start.elapsed().unwrap().as_micros());

    Ok(())
}
