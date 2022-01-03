use std::{
    cmp,
    collections::{BinaryHeap, HashMap},
    env, fs,
    path::Path,
    time::SystemTime,
};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Kind {
    Rocky,
    Wet,
    Narrow,
}

impl Kind {
    const fn from_erosion_level(erosion_level: usize) -> Self {
        match erosion_level % 3 {
            0 => Self::Rocky,
            1 => Self::Wet,
            2 => Self::Narrow,
            _ => unreachable!(),
        }
    }

    const fn risk_level(&self) -> usize {
        match self {
            Self::Rocky => 0,
            Self::Wet => 1,
            Self::Narrow => 2,
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Tile {
    kind: Kind,
    erosion_level: usize,
}

impl Tile {
    fn new(erosion_level: usize) -> Self {
        Self {
            erosion_level,
            kind: Kind::from_erosion_level(erosion_level),
        }
    }
}

fn parse_input(input: &str) -> (usize, (usize, usize)) {
    let mut lines = input.lines();
    let depth_line = lines.next().unwrap();

    let depth = depth_line
        .split_ascii_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap();

    let target_line = lines.next().unwrap();

    let mut coords = target_line
        .split_ascii_whitespace()
        .last()
        .unwrap()
        .split(',');

    (
        depth,
        (
            coords.next().unwrap().parse().unwrap(),
            coords.next().unwrap().parse().unwrap(),
        ),
    )
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
enum Tool {
    None,
    ClimbingGear,
    Torch,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Spelunker {
    location: (usize, usize),
    minutes_spent: usize,
    tool: Tool,
    goal: (usize, usize),
}

impl Spelunker {
    fn new(goal: (usize, usize)) -> Self {
        Self {
            location: (0, 0),
            minutes_spent: 0,
            tool: Tool::Torch,
            goal,
        }
    }

    fn move_to(&self, location: (usize, usize)) -> Self {
        let mut new = *self;
        new.minutes_spent += 1;
        new.location = location;
        new
    }

    fn switch_to(&self, tool: Tool) -> Self {
        let mut new = *self;
        new.tool = tool;
        new.minutes_spent += 7;
        new
    }

    fn distance_to_goal(&self) -> usize {
        let (x, y) = self.location;
        let (gx, gy) = self.goal;
        (x as i32 - gx as i32).abs() as usize + (y as i32 - gy as i32).abs() as usize
    }
}

impl std::hash::Hash for Spelunker {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        self.location.hash(state);
        self.tool.hash(state);
    }
}

impl PartialOrd for Spelunker {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Spelunker {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        (self.minutes_spent + self.distance_to_goal())
            .cmp(&(other.minutes_spent + other.distance_to_goal()))
            .reverse()
    }
}

struct Cave {
    tiles: HashMap<(usize, usize), Tile>,
    target: (usize, usize),
    depth: usize,
}

impl Cave {
    fn new(depth: usize, (x, y): (usize, usize)) -> Self {
        Self {
            tiles: HashMap::new(),
            target: (x, y),
            depth,
        }
    }

    fn get(&mut self, (x, y): (usize, usize)) -> Tile {
        if let Some(tile) = self.tiles.get(&(x, y)) {
            return *tile;
        }

        let geologic_index = match (x, y) {
            (0, 0) => 0,
            (_, 0) => x * 16807,
            (0, _) => y * 48271,
            coords if coords == self.target => 0,
            _ => self.get((x, y - 1)).erosion_level * self.get((x - 1, y)).erosion_level,
        };

        let erosion_level = (geologic_index + self.depth) % 20183;

        let tile = Tile::new(erosion_level);
        self.tiles.insert((x, y), tile);
        tile
    }

    fn risk_level(&mut self) -> usize {
        let mut risk_level = 0;

        for y in (0..=self.target.1).rev() {
            for x in (0..=self.target.0).rev() {
                risk_level += self.get((x, y)).kind.risk_level()
            }
        }

        risk_level
    }
}

fn solve(input: &str) -> (usize, usize) {
    // let (depth, target) = (510, (10, 10));
    let (depth, target) = parse_input(input);
    let mut cave = Cave::new(depth, target);

    let risk_level = cave.risk_level();

    let mut heap = BinaryHeap::from([Spelunker::new(target)]);
    let mut seen = HashMap::new();
    let mut min_minutes = usize::MAX;

    while let Some(current) = heap.pop() {
        let kind = cave.get(current.location).kind;

        if matches!(
            (kind, current.tool),
            (Kind::Rocky, Tool::None)
                | (Kind::Wet, Tool::Torch)
                | (Kind::Narrow, Tool::ClimbingGear)
        ) {
            continue;
        }

        let entry = seen
            .entry((current.location, current.tool))
            .or_insert(usize::MAX);

        if *entry <= current.minutes_spent {
            continue;
        }

        *entry = current.minutes_spent;

        if current.location == target && current.tool == Tool::Torch {
            min_minutes = current.minutes_spent;
            break;
        }

        let (x, y) = current.location;
        heap.push(current.move_to((x.saturating_sub(1), y)));
        heap.push(current.move_to((x + 1, y)));
        heap.push(current.move_to((x, y.saturating_sub(1))));
        heap.push(current.move_to((x, y + 1)));

        for tool in [Tool::None, Tool::Torch, Tool::ClimbingGear] {
            if current.tool != tool {
                heap.push(current.switch_to(tool));
            }
        }
    }

    (risk_level, min_minutes)
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

    time_it(|| println!("parts (1, 2): {:?}", solve(&input)));

    Ok(())
}
