use std::{
    collections::{HashMap, VecDeque},
    fmt::{Debug, Display},
    hash::Hash,
};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Coord(pub i64, pub i64);

impl Coord {
    pub fn distance_to(&self, other: &Self) -> i64 {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }

    pub fn translate_y(&self, distance: i64) -> Self {
        Self(self.0, self.1 + distance)
    }

    pub fn translate_x(&self, distance: i64) -> Self {
        Self(self.0 + distance, self.1)
    }

    pub fn step(&self, direction: &Direction) -> Self {
        match direction {
            Direction::North => Self(self.0, self.1 + 1),
            Direction::East => Self(self.0 + 1, self.1),
            Direction::South => Self(self.0, self.1 - 1),
            Direction::West => Self(self.0 - 1, self.1),
        }
    }
}

impl std::ops::Add<Coord> for Coord {
    type Output = Self;

    fn add(self, rhs: Coord) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Debug for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
pub struct Graph<T>
where
    T: PartialEq + Eq + Hash + Debug + Copy,
{
    edges: HashMap<T, Vec<T>>,
    distance_cache: HashMap<(T, T), i32>,
}

impl<T> Graph<T>
where
    T: PartialEq + Eq + Hash + Debug + Copy,
{
    pub fn new() -> Self {
        Self {
            edges: HashMap::new(),
            distance_cache: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, value: T) {
        self.edges.insert(value, vec![]);
    }

    pub fn add_edge(&mut self, from: T, to: T) {
        let entry = self.edges.entry(from).or_default();
        entry.push(to);
    }

    pub fn get_edges(&mut self, from: T) -> &Vec<T> {
        self.edges.get(&from).unwrap()
    }

    pub fn distance_between(&mut self, from: T, to: T) -> i32 {
        if let Some(distance) = self.distance_cache.get(&(from, to)) {
            return *distance;
        }

        let mut queue: VecDeque<_> = self
            .edges
            .get(&from)
            .unwrap()
            .iter()
            .map(|loc| (loc, 1))
            .collect();

        while let Some((location, steps)) = queue.pop_front() {
            let entry = self
                .distance_cache
                .entry((from, *location))
                .or_insert(i32::MAX);
            *entry = steps.min(*entry);

            if *location == to {
                self.distance_cache.insert((from, to), steps);
                self.distance_cache.insert((to, from), steps);
                return steps;
            }

            for next in self.edges.get(location).unwrap() {
                queue.push_back((next, steps + 1))
            }
        }

        unreachable!()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord3D(pub i64, pub i64, pub i64);

impl Coord3D {
    pub fn adjacent(&self, other: &Self) -> bool {
        (self.0 - other.0).abs() + (self.1 - other.1).abs() + (self.2 - other.2).abs() == 1
    }
}

impl Coord3D {
    pub fn translate(&self, [a, b, c]: [i64; 3]) -> Self {
        Self(self.0 + a, self.1 + b, self.2 + c)
    }
}

impl std::ops::Add<Coord3D> for Coord3D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Debug for Coord3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.0, self.1, self.2)
    }
}

impl Display for Coord3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod test {
    use super::Coord3D;

    #[test]
    fn test_coord_3d_adjacency() {
        assert!(Coord3D(1, 1, 1).adjacent(&Coord3D(2, 1, 1)));
        assert!(!Coord3D(1, 1, 1).adjacent(&Coord3D(3, 1, 1)));
        assert!(Coord3D(1, 1, 1).adjacent(&Coord3D(0, 1, 1)));
    }
}

pub struct Chart<T>
where
    T: Default + Clone + Copy + PartialEq + Eq + Into<char>,
{
    data: HashMap<Coord, T>,
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
}

impl<T> Display for Chart<T>
where
    T: Default + Clone + Copy + PartialEq + Eq + Into<char>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let y_range: Box<dyn Iterator<Item = i64>> = if self.min_y.is_negative() {
            Box::from((self.min_y..=self.max_y).rev())
        } else {
            Box::from(self.min_y..=self.max_y)
        };

        for y in y_range {
            for x in self.min_x..=self.max_x {
                write!(
                    f,
                    "{}",
                    self.get(&Coord(x, y)).copied().unwrap_or_default().into() as char
                )?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

impl<'a, T> IntoIterator for &'a Chart<T>
where
    T: Default + Clone + Copy + PartialEq + Eq + Into<char>,
{
    type Item = (&'a Coord, &'a T);

    type IntoIter = std::collections::hash_map::Iter<'a, Coord, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl<T> Chart<T>
where
    T: Default + Clone + Copy + PartialEq + Eq + Into<char>,
{
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            min_x: 0,
            min_y: 0,
            max_x: 0,
            max_y: 0,
        }
    }

    pub fn get(&self, coord: &Coord) -> Option<&T> {
        self.data.get(coord)
    }

    pub fn get_mut(&mut self, coord: &Coord) -> Option<&mut T> {
        self.data.get_mut(coord)
    }

    pub fn overwrite(&mut self, coord: &Coord, value: T) -> Option<T> {
        self.min_x = self.min_x.min(coord.0);
        self.max_x = self.max_x.max(coord.0);
        self.min_y = self.min_y.min(coord.1);
        self.max_y = self.max_y.max(coord.1);
        self.data.insert(*coord, value)
    }

    pub fn set(&mut self, coord: &Coord, mut value: T) -> Option<T> {
        match self.get_mut(coord) {
            Some(entry) if *entry == Default::default() => {
                std::mem::swap(entry, &mut value);
                Some(value)
            }
            None => self.overwrite(coord, value),
            _ => None,
        }
    }

    pub fn print(&self) {
        println!("{}", self)
    }

    pub fn top(&self) -> i64 {
        if self.min_y < 0 {
            self.max_y
        } else {
            self.min_y
        }
    }

    pub fn bottom(&self) -> i64 {
        if self.min_y < 0 {
            self.min_y
        } else {
            self.max_y
        }
    }

    pub fn left(&self) -> i64 {
        self.min_x
    }

    pub fn right(&self) -> i64 {
        self.max_x
    }

    pub fn iter_grid<'a>(&'a self) -> impl Iterator<Item = (Coord, T)> + 'a {
        let y_range: Box<dyn Iterator<Item = i64>> = if self.min_y.is_negative() {
            Box::from((self.min_y..=self.max_y).rev())
        } else {
            Box::from(self.min_y..=self.max_y)
        };

        y_range.into_iter().flat_map(move |y| {
            (self.min_x..=self.max_x).map(move |x| {
                let coord = Coord(x, y);
                (coord, self.data.get(&coord).copied().unwrap_or_default())
            })
        })
    }
}
