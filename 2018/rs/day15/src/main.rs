use std::{
    cmp,
    collections::{BinaryHeap, HashSet, VecDeque},
    env, fs, iter,
    path::Path,
    time::SystemTime,
};

static mut QUEUE: Option<VecDeque<(Coord, Direction, usize)>> = None;
static mut SEEN: Option<HashSet<Coord>> = None;

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Left,
    Right,
    Down,
}

impl Direction {
    fn iter() -> impl Iterator<Item = Self> {
        iter::once(Self::Up)
            .chain(iter::once(Self::Left))
            .chain(iter::once(Self::Right))
            .chain(iter::once(Self::Down))
    }
}

#[derive(Debug)]
struct MinHeap<T: Ord + PartialOrd + Eq + PartialEq>(BinaryHeap<Min<T>>);

impl<T: Ord + PartialOrd + Eq + PartialEq> MinHeap<T> {
    fn new() -> Self {
        Self(BinaryHeap::new())
    }

    fn push(&mut self, value: T) {
        self.0.push(Min(value))
    }

    fn pop(&mut self) -> Option<T> {
        match self.0.pop() {
            Some(Min(value)) => Some(value),
            _ => None,
        }
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
struct Coord(usize, usize);

impl Coord {
    fn neighbors(&self) -> impl Iterator<Item = Coord> {
        let &Self(x, y) = self;
        iter::once(Self(x, y - 1))
            .chain(iter::once(Self(x - 1, y)))
            .chain(iter::once(Self(x + 1, y)))
            .chain(iter::once(Self(x, y + 1)))
    }
}

impl cmp::PartialOrd for Coord {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(match self.1.cmp(&other.1) {
            cmp::Ordering::Equal => self.0.cmp(&other.0),
            cmp => cmp,
        })
    }
}

impl cmp::Ord for Coord {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        match self.partial_cmp(&other) {
            Some(order) => order,
            None => unreachable!(),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Min<T: PartialOrd + Ord + Eq + PartialEq>(T);

impl<T: Ord> cmp::PartialOrd for Min<T> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl<T: Ord> cmp::Ord for Min<T> {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        other.0.cmp(&self.0)
    }
}

#[derive(Debug, Copy, Clone)]
enum Tile {
    Empty,
    Wall,
    Elf(usize),
    Goblin(usize),
}

impl Default for Tile {
    fn default() -> Self {
        Tile::Empty
    }
}

impl Tile {
    fn is_creature(&self) -> bool {
        matches!(self, Tile::Goblin(_) | Tile::Elf(_))
    }

    fn is_empty(&self) -> bool {
        matches!(self, Tile::Empty)
    }

    fn from_char(ch: char) -> Self {
        match ch {
            '.' => Self::Empty,
            '#' => Self::Wall,
            'E' => Self::Elf(200),
            'G' => Self::Goblin(200),
            _ => unreachable!(),
        }
    }

    #[cfg(debug_assertions)]
    fn char(&self) -> char {
        match self {
            Self::Empty => '.',
            Self::Wall => '#',
            Self::Elf(_) => 'E',
            Self::Goblin(_) => 'G',
        }
    }

    fn is_enemy(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::Goblin(_), Self::Elf(_)) | (Self::Elf(_), Self::Goblin(_))
        )
    }

    fn hp(&self) -> usize {
        match self {
            Self::Goblin(hp) | Self::Elf(hp) => *hp,
            _ => unreachable!(),
        }
    }

    fn damage(&mut self, amount: usize) {
        match self {
            Self::Goblin(hp) | Self::Elf(hp) => *hp = hp.saturating_sub(amount),
            _ => unreachable!(),
        }
    }

    fn is_dead(&self) -> bool {
        match self {
            Self::Goblin(0) | Self::Elf(0) => true,
            _ => false,
        }
    }
}

struct Cave {
    layout: Vec<Vec<Tile>>,
    heap: MinHeap<Coord>,
    locations: Vec<Coord>,
    ticks: usize,
    elf_power: usize,
}

impl Cave {
    fn from_str(input: &str) -> Self {
        let layout: Vec<Vec<_>> = input
            .lines()
            .map(|line| line.chars().map(Tile::from_char).collect())
            .collect();

        Self {
            locations: layout
                .iter()
                .enumerate()
                .flat_map(|(y, row)| {
                    row.iter().enumerate().filter_map(move |(x, tile)| {
                        if tile.is_creature() {
                            Some(Coord(x, y))
                        } else {
                            None
                        }
                    })
                })
                .collect(),
            layout,
            heap: MinHeap::new(),
            ticks: 0,
            elf_power: 3,
        }
    }

    fn get(&self, Coord(x, y): Coord) -> &Tile {
        if let Some(row) = self.layout.get(y) {
            if let Some(tile) = row.get(x) {
                return tile;
            }
        }

        unreachable!()
    }

    fn get_mut(&mut self, Coord(x, y): Coord) -> &mut Tile {
        if let Some(row) = self.layout.get_mut(y) {
            if let Some(tile) = row.get_mut(x) {
                return tile;
            }
        }

        unreachable!()
    }

    fn find_path(&self, start: Coord, goal: Coord) -> Option<(Direction, usize)> {
        unsafe {
            let queue = QUEUE.get_or_insert_with(|| VecDeque::new());
            queue.clear();

            for (neighbor, direction) in start.neighbors().zip(Direction::iter()) {
                if self.get(neighbor).is_empty() {
                    queue.push_back((neighbor, direction, 1));
                }
            }

            let seen = SEEN.get_or_insert_with(|| HashSet::new());
            seen.clear();
            seen.insert(start);

            while let Some((a, direction, distance)) = queue.pop_front() {
                if a == goal {
                    return Some((direction, distance));
                }

                for neighbor in a.neighbors() {
                    if seen.contains(&neighbor) {
                        continue;
                    }
                    seen.insert(neighbor);
                    if self.get(neighbor).is_empty() {
                        queue.push_back((neighbor, direction, distance + 1));
                    }
                }
            }

            None
        }
    }

    fn insert_tile_at(&mut self, coord: Coord, tile: Tile) {
        self.layout[coord.1][coord.0] = tile;
        self.locations.push(coord);
    }

    fn remove_tile_at(&mut self, coord: &Coord) -> Tile {
        self.locations.retain(|c| *c != *coord);
        std::mem::take(&mut self.layout[coord.1][coord.0])
    }

    fn count_elves(&self) -> usize {
        let mut count = 0;

        for loc in &self.locations {
            if matches!(self.get(*loc), Tile::Elf(_)) {
                count += 1;
            }
        }

        count
    }

    fn only_goblins_or_elves(&self) -> bool {
        let elf_count = self.count_elves();

        elf_count == self.locations.len() || elf_count == 0
    }

    fn find_move(&self, coord: &Coord, tile: &Tile) -> Option<Direction> {
        for neighbor in coord.neighbors() {
            if self.get(neighbor).is_enemy(tile) {
                return None;
            }
        }

        let mut min_distance = usize::MAX;
        let mut destination_opt = None;
        let mut direction = None;

        for dest in &self.locations {
            if !self.get(*dest).is_enemy(tile) {
                continue;
            }
            for neighbor in dest.neighbors() {
                let neighbor_tile = self.get(neighbor);

                if !neighbor_tile.is_empty() {
                    continue;
                }

                let (dir, dist) = match self.find_path(*coord, neighbor) {
                    Some(result) => result,
                    None => continue,
                };

                match (dist.cmp(&min_distance), destination_opt) {
                    (cmp::Ordering::Less, _) | (_, None) => {
                        min_distance = dist;
                        destination_opt = Some(neighbor);
                        direction = Some(dir);
                    }
                    (cmp::Ordering::Equal, Some(destination)) => {
                        if neighbor < destination {
                            min_distance = dist;
                            destination_opt = Some(neighbor);
                            direction = Some(dir);
                        }
                    }
                    (cmp::Ordering::Greater, _) => {}
                }
            }
        }

        direction
    }

    fn get_damage(&self, tile: &Tile) -> usize {
        match tile {
            Tile::Goblin(_) => 3,
            Tile::Elf(_) => self.elf_power,
            _ => unreachable!(),
        }
    }

    fn tick(&mut self) -> bool {
        for &Coord(x, y) in &self.locations {
            self.heap.push(Coord(x, y))
        }

        while let Some(mut coord) = self.heap.pop() {
            let tile = self.remove_tile_at(&coord);
            if !tile.is_creature() {
                continue;
            }

            if let Some(direction) = self.find_move(&coord, &tile) {
                coord = match direction {
                    Direction::Up => Coord(coord.0, coord.1 - 1),
                    Direction::Right => Coord(coord.0 + 1, coord.1),
                    Direction::Down => Coord(coord.0, coord.1 + 1),
                    Direction::Left => Coord(coord.0 - 1, coord.1),
                };
            };

            let mut target = None;
            let mut lowest_hp = usize::MAX;
            for neighbor in coord.neighbors() {
                let neighbor_tile = self.get(neighbor);
                if tile.is_enemy(neighbor_tile) {
                    let neighbor_hp = neighbor_tile.hp();
                    match (neighbor_hp.cmp(&lowest_hp), target) {
                        (cmp::Ordering::Less, _) | (_, None) => {
                            lowest_hp = neighbor_hp;
                            target = Some(neighbor);
                        }
                        (cmp::Ordering::Equal, Some(other)) => {
                            if neighbor < other {
                                lowest_hp = neighbor_hp;
                                target = Some(neighbor);
                            }
                        }
                        (cmp::Ordering::Greater, _) => {}
                    }
                }
            }

            if let Some(target) = target {
                let amount = self.get_damage(&tile);
                let enemy = self.get_mut(target);
                enemy.damage(amount);
                if enemy.is_dead() {
                    self.remove_tile_at(&target);

                    if self.only_goblins_or_elves() {
                        self.insert_tile_at(coord, tile);
                        break;
                    }
                }
            }

            self.insert_tile_at(coord, tile);
        }

        if self.heap.len() == 0 {
            self.ticks += 1;
        }

        self.heap.len() == 0 && !self.only_goblins_or_elves()
    }

    #[cfg(debug_assertions)]
    fn print(&self) {
        let mut vec = vec![];
        println!("{:?}", self.locations);
        for row in &self.layout {
            for tile in row {
                print!("{}", tile.char());
                if tile.is_creature() {
                    vec.push(tile);
                }
            }

            print!("   ");
            for creature in &vec {
                print!("{}({}), ", creature.char(), creature.hp());
            }
            vec.clear();
            println!();
        }
        println!();
    }
}

fn part_one(input: &str) -> usize {
    let mut cave = Cave::from_str(input);

    #[cfg(debug_assertions)]
    cave.print();

    while cave.tick() {
        #[cfg(debug_assertions)]
        cave.print();
    }

    cave.ticks
        * cave
            .locations
            .iter()
            .fold(0, |acc, coord| acc + cave.get(*coord).hp())
}

fn part_two(input: &str) -> usize {
    let mut cave = Cave::from_str(input);

    let mut elf_power = 4;

    let starting_elf_count = cave.count_elves();

    loop {
        #[cfg(debug_assertions)]
        println!("elf power: {}", elf_power);
        cave.elf_power = elf_power;
        while cave.tick() {}

        if cave.count_elves() == starting_elf_count {
            break;
        }

        elf_power += 1;
        cave = Cave::from_str(input)
    }

    cave.ticks
        * cave
            .locations
            .iter()
            .fold(0, |acc, coord| acc + cave.get(*coord).hp())
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
