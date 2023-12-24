use std::{
    collections::{BinaryHeap, HashMap},
    time::SystemTime,
};
use utils::{Direction, Point, Translate};

const DATA: &'static str = include_str!("../data.txt");

fn parse_grid() -> Vec<Vec<usize>> {
    DATA.trim()
        .lines()
        .map(|l| l.split("").flat_map(|c| c.parse()).collect())
        .collect()
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Crucible {
    straights: usize,
    dir: Direction,
    pos: Point,
    heat_loss: usize,
}

impl Ord for Crucible {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.heat_loss.cmp(&self.heat_loss)
    }
}

impl PartialOrd for Crucible {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.heat_loss.partial_cmp(&self.heat_loss)
    }
}

fn push_crucible(min: usize, max: usize) -> usize {
    let tiles = parse_grid();
    let mut cache = HashMap::<(Point, Direction, usize), (bool, usize)>::new();

    let mut heap = BinaryHeap::from([
        Crucible {
            pos: Point::from((0, 0)),
            dir: Direction::East,
            straights: 0,
            heat_loss: 0,
        },
        Crucible {
            pos: Point::from((0, 0)),
            dir: Direction::South,
            straights: 0,
            heat_loss: 0,
        },
    ]);
    while let Some(c) = heap.pop() {
        cache
            .entry((c.pos, c.dir, c.straights))
            .or_insert((false, usize::MAX))
            .0 = true;

        for dir in [c.dir, c.dir.left(), c.dir.right()] {
            let same_dir = c.dir == dir;

            if (c.straights < min && !same_dir) || (c.straights > max - 1 && same_dir) {
                continue;
            }

            let pos = c.pos.translate(dir);
            let straights = 1 + if same_dir { c.straights } else { 0 };
            let heat_loss = c.heat_loss
                + if let Some(c) = tiles
                    .get(pos.y as usize)
                    .and_then(|row| row.get(pos.x as usize))
                {
                    c
                } else {
                    continue;
                };

            let (visited, min_heat_loss) = cache
                .entry((pos, dir, straights))
                .or_insert((false, usize::MAX));

            if *visited || *min_heat_loss <= heat_loss {
                continue;
            }
            *min_heat_loss = heat_loss;

            heap.push(Crucible {
                pos,
                dir,
                straights,
                heat_loss,
            })
        }
    }

    let target = Point::from((tiles[0].len() - 1, tiles.len() - 1));

    cache
        .into_iter()
        .filter_map(
            |((pos, _, _), (_visited, cost))| {
                if pos == target {
                    Some(cost)
                } else {
                    None
                }
            },
        )
        .min()
        .unwrap()
}

fn part_one() -> usize {
    push_crucible(0, 3)
}

fn part_two() -> usize {
    push_crucible(4, 10)
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
