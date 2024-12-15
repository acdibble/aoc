use std::fmt::Debug;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn turn_right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    pub fn turn_left(self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Right => Self::Up,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
        }
    }

    pub const fn all() -> [Self; 4] {
        [Self::Up, Self::Down, Self::Left, Self::Right]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Coord {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn translate(self, direction: Direction) -> Self {
        match direction {
            Direction::Up => Self::new(self.x, self.y - 1),
            Direction::Down => Self::new(self.x, self.y + 1),
            Direction::Left => Self::new(self.x - 1, self.y),
            Direction::Right => Self::new(self.x + 1, self.y),
        }
    }

    pub fn manhattan_distance(self, other: Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Debug)]
pub struct Map<T: Debug> {
    data: Vec<Vec<T>>,
}

impl<T> From<Vec<Vec<T>>> for Map<T>
where
    T: Debug,
{
    fn from(data: Vec<Vec<T>>) -> Self {
        Self { data }
    }
}

impl<T: Debug> Map<T> {
    pub fn get(&self, coord: Coord) -> Option<&T> {
        self.data
            .get(coord.y as usize)
            .and_then(|row| row.get(coord.x as usize))
    }

    pub fn iter(&self) -> impl Iterator<Item = (Coord, &T)> + '_ {
        self.data.iter().enumerate().flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, value)| (Coord::new(x as i32, y as i32), value))
        })
    }

    pub fn iter_coords(&self) -> impl Iterator<Item = Coord> + '_ {
        (0..self.data.len() as i32)
            .flat_map(move |y| (0..self.data[0].len() as i32).map(move |x| Coord::new(x, y)))
    }
}

impl std::ops::Sub<Coord> for Coord {
    type Output = Self;

    fn sub(self, rhs: Coord) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl std::ops::Add<Coord> for Coord {
    type Output = Self;

    fn add(self, rhs: Coord) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::AddAssign<Coord> for Coord {
    fn add_assign(&mut self, rhs: Coord) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl std::ops::Neg for Coord {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CopyRange<T> {
    pub start: T,
    pub end: T,
}

impl<T: PartialOrd<T>> CopyRange<T> {
    pub fn contains(&self, n: &T) -> bool {
        *n >= self.start && *n <= self.end
    }
}

impl<T> CopyRange<T> {
    pub fn new(start: T, end: T) -> Self {
        Self { start, end }
    }
}

impl<T> From<std::ops::Range<T>> for CopyRange<T> {
    fn from(range: std::ops::Range<T>) -> Self {
        Self {
            start: range.start,
            end: range.end,
        }
    }
}
