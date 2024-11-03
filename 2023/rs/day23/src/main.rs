use std::{
    collections::{HashMap, HashSet, VecDeque},
    time::SystemTime,
};
use utils::{Chart, Direction, Point, Translate};

const DATA: &str = include_str!("../data.txt");

#[derive(Debug, Clone, Copy)]
enum Tile {
    Path,
    Forest,
    SlopeUp,
    SlopeDown,
    SlopeLeft,
    SlopeRight,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Forest,
            '.' => Self::Path,
            '^' => Self::SlopeUp,
            'v' => Self::SlopeDown,
            '>' => Self::SlopeRight,
            '<' => Self::SlopeLeft,
            _ => unreachable!("unexpected char: '{value}'"),
        }
    }
}

impl From<Tile> for char {
    fn from(value: Tile) -> Self {
        match value {
            Tile::Forest => '#',
            Tile::Path => '.',
            Tile::SlopeUp => '^',
            Tile::SlopeDown => 'v',
            Tile::SlopeRight => '>',
            Tile::SlopeLeft => '<',
        }
    }
}

impl Tile {
    fn is_passable(&self, direction_of_travel: Option<Direction>) -> bool {
        match (self, direction_of_travel) {
            (Self::Forest, _) => false,
            (Self::Path, _) => true,
            (Self::SlopeUp, Some(Direction::North) | None) => true,
            (Self::SlopeUp, _) => false,
            (Self::SlopeDown, Some(Direction::South) | None) => true,
            (Self::SlopeDown, _) => false,
            (Self::SlopeLeft, Some(Direction::West) | None) => true,
            (Self::SlopeLeft, _) => false,
            (Self::SlopeRight, Some(Direction::East) | None) => true,
            (Self::SlopeRight, _) => false,
        }
    }
}

type Graph = HashMap<Point, Vec<(Point, usize)>>;

fn build_graph(slopes: bool) -> (Graph, Point) {
    let chart = Chart::from(
        DATA.lines()
            .map(|line| line.chars().map(|ch| Tile::from(ch)).collect())
            .collect::<Vec<Vec<_>>>(),
    );

    let pos = Point::from((1, 0));
    let mut visited = HashSet::from([pos]);
    let mut nodes = vec![pos];
    let mut queue = VecDeque::from([pos]);

    while let Some(pos) = queue.pop_front() {
        let mut count = 0;

        for dir in Direction::all() {
            let next_pos = pos.translate(dir);

            if visited.contains(&next_pos) {
                continue;
            }

            match chart.get(next_pos) {
                Some(tile) if tile.is_passable(None) => {
                    queue.push_back(next_pos);
                    visited.insert(next_pos);
                    count += 1;
                }
                _ => continue,
            }
        }

        if count > 1 {
            nodes.push(pos);
        }
    }

    let y = chart.height() - 1;

    let mut target = Point::from((0, 0));

    for x in 0..chart.width() {
        let pos = Point::from((x, y));
        match chart.get(pos) {
            Some(Tile::Path) => {
                nodes.push(pos);
                target = pos;
                break;
            }
            Some(_) => continue,
            None => unreachable!(),
        }
    }

    let mut graph = HashMap::new();
    let mut queue = VecDeque::new();
    for node in &nodes {
        visited.clear();
        visited.insert(*node);
        queue.push_back((*node, 0));

        while let Some((pos, steps)) = queue.pop_front() {
            if pos != *node && nodes.contains(&pos) {
                let entry = graph.entry(*node).or_insert(Vec::new());
                entry.push((pos, steps));

                continue;
            }

            for next_dir in Direction::all() {
                let next_pos = pos.translate(next_dir);
                if visited.contains(&next_pos) {
                    continue;
                }
                visited.insert(next_pos);

                let check = slopes.then(|| next_dir);

                match chart.get(next_pos) {
                    Some(tile) if tile.is_passable(check) => {
                        queue.push_back((pos.translate(next_dir), steps + 1))
                    }
                    _ => {}
                }
            }
        }
    }

    (graph, target)
}

fn hike(slopes: bool) -> usize {
    let start = Point::from((1, 0));

    let (graph, exit) = build_graph(slopes);

    let visited = HashSet::from([start]);
    let mut queue = VecDeque::from([(start, 0, visited)]);

    let mut max = 0;

    while let Some((pos, steps, visited)) = queue.pop_front() {
        if pos == exit {
            max = max.max(steps);
            continue;
        }

        for &(next, weight) in graph.get(&pos).unwrap() {
            if visited.contains(&next) {
                continue;
            }
            let mut visited = visited.clone();
            visited.insert(next);
            queue.push_back((next, steps + weight, visited));
        }
    }

    max
}

fn part_one() -> usize {
    hike(true)
}

fn part_two() -> usize {
    hike(false)
}

fn time_it<F, T>(fun: F) -> T
where
    F: Fn() -> T,
{
    let start = SystemTime::now();
    let result = fun();
    println!("Time elapsed: {} ms", start.elapsed().unwrap().as_millis());
    result
}

fn main() {
    time_it(|| println!("part 1: {}", part_one()));
    time_it(|| println!("part 2: {}", part_two()));
}
