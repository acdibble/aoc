#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub trait Translate<T> {
    fn translate(&self, change: T) -> Self;
}

impl Translate<(i32, i32)> for Point {
    fn translate(&self, change: (i32, i32)) -> Self {
        Self {
            x: self.x + change.0,
            y: self.y + change.1,
        }
    }
}

impl Translate<Direction> for Point {
    fn translate(&self, direction: Direction) -> Self {
        match direction {
            Direction::North => self.translate_up(),
            Direction::East => self.translate_right(),
            Direction::South => self.translate_down(),
            Direction::West => self.translate_left(),
        }
    }
}

impl Point {
    pub fn translate_up(&self) -> Self {
        self.translate((0, -1))
    }

    pub fn translate_down(&self) -> Self {
        self.translate((0, 1))
    }

    pub fn translate_left(&self) -> Self {
        self.translate((-1, 0))
    }

    pub fn translate_right(&self) -> Self {
        self.translate((1, 0))
    }

    pub fn manhattan_distance(&self, other: &Self) -> i32 {
        (self.x - other.x).abs() + (other.y - self.y).abs()
    }
}

impl From<(usize, usize)> for Point {
    fn from((x, y): (usize, usize)) -> Self {
        Self {
            x: x as i32,
            y: y as i32,
        }
    }
}

impl From<(i32, i32)> for Point {
    fn from((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }
}

pub struct LineSegment {
    pub start: Point,
    pub end: Point,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn all() -> [Self; 4] {
        [Self::North, Self::East, Self::South, Self::West]
    }

    pub fn rev(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
        }
    }

    pub fn left(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    pub fn right(&self) -> Self {
        match self {
            Self::North => Self::West,
            Self::East => Self::North,
            Self::South => Self::East,
            Self::West => Self::South,
        }
    }
}

pub mod math {
    pub mod traits {
        pub trait GCD {
            fn gcd(self, other: Self) -> Self;
        }

        pub trait LCM {
            fn lcm(self, other: Self) -> Self;
        }

        macro_rules! impl_gcd_lcm_traits {
            ($($t:ty),*) => ($(
                impl GCD for $t {
                    fn gcd(self, other: Self) -> Self {
                        let mut a = self;
                        let mut b = other;
                        while b != 0 {
                            let t = b;
                            b = a % b;
                            a = t;
                        }
                        a
                    }
                }

                impl LCM for $t {
                    fn lcm(self, other: Self) -> Self {
                        self * other / self.gcd(other)
                    }
                }
            )*)
        }

        impl_gcd_lcm_traits!(usize);
    }
}
