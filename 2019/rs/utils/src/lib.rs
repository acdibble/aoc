struct PermutationIterator<const N: usize, T: Copy> {
    state: [T; N],
    stack: [usize; N],
    pointer: usize,
    initial_done: bool,
}

impl<const N: usize, T: Copy> Iterator for PermutationIterator<N, T> {
    type Item = [T; N];

    fn next(&mut self) -> Option<Self::Item> {
        if !self.initial_done {
            self.initial_done = true;
            return Some(self.state);
        }

        while self.pointer < N {
            if self.stack[self.pointer] < self.pointer {
                if self.pointer % 2 == 0 {
                    self.state.swap(0, self.pointer);
                } else {
                    self.state.swap(self.stack[self.pointer], self.pointer);
                }

                self.stack[self.pointer] += 1;
                self.pointer = 1;
                return Some(self.state);
            } else {
                self.stack[self.pointer] = 0;
                self.pointer += 1;
            }
        }

        None
    }
}

pub fn permute<const N: usize, T: Copy>(input: [T; N]) -> impl Iterator<Item = [T; N]> {
    PermutationIterator {
        state: input,
        stack: [0; N],
        pointer: 1,
        initial_done: false,
    }
}

// Euclid's two-thousand-year-old algorithm for finding the greatest common
// divisor.
fn gcd(x: i32, y: i32) -> i32 {
    let mut x = x.abs();
    let mut y = y.abs();
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}

pub mod fraction {
    #[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
    pub enum Fraction {
        Defined(i32, i32),
        NegativeUndefined,
        PositiveUndefined,
    }

    impl Into<f64> for &Fraction {
        fn into(self) -> f64 {
            match self {
                &Fraction::Defined(num, denom) => num as f64 / denom as f64,
                Fraction::PositiveUndefined => f64::INFINITY,
                Fraction::NegativeUndefined => f64::NEG_INFINITY,
            }
        }
    }

    impl Into<f64> for Fraction {
        fn into(self) -> f64 {
            match self {
                Self::Defined(num, denom) => num as f64 / denom as f64,
                Self::PositiveUndefined => f64::INFINITY,
                Self::NegativeUndefined => f64::NEG_INFINITY,
            }
        }
    }

    impl Ord for Fraction {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            if self == other {
                return std::cmp::Ordering::Equal;
            }

            let a: f64 = self.into();
            let b: f64 = other.into();

            if a < b {
                std::cmp::Ordering::Less
            } else if a > b {
                std::cmp::Ordering::Greater
            } else {
                match (self.reduce(), other.reduce()) {
                    (Self::Defined(num_a, den_a), Self::Defined(num_b, den_b)) => {
                        match num_a.cmp(&num_b) {
                            std::cmp::Ordering::Equal => den_a.cmp(&den_b),
                            other => other,
                        }
                    }
                    _ => std::cmp::Ordering::Equal,
                }
            }
        }
    }

    impl PartialOrd for Fraction {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Fraction {
        pub fn from(numerator: i32, denominator: i32) -> Self {
            if denominator == 0 {
                return if numerator.is_negative() {
                    Self::NegativeUndefined
                } else {
                    Self::PositiveUndefined
                };
            }

            Self::Defined(numerator, denominator)
        }

        pub fn reduce(&self) -> Self {
            match self {
                &Self::Defined(numerator, denominator) => {
                    let divisor = super::gcd(numerator, denominator);

                    Self::Defined(numerator / divisor, denominator / divisor)
                }
                _ => *self,
            }
        }

        pub fn normalize(&self) -> Self {
            use std::cmp::Ordering::*;

            match self {
                &Self::Defined(mut numerator, mut denominator) => {
                    match (numerator.cmp(&0), denominator.cmp(&0)) {
                        (Greater, Less) | (Less, Less) => {
                            numerator = -numerator;
                            denominator = -denominator;
                        }
                        _ => {}
                    }

                    Self::Defined(numerator, denominator)
                }
                _ => *self,
            }
        }

        pub fn normalized(numerator: i32, denominator: i32) -> Self {
            Self::from(numerator, denominator).reduce().normalize()
        }
    }
}

pub mod grid {
    use super::fraction::Fraction;

    #[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
    pub struct Coordinate {
        pub x: i32,
        pub y: i32,
    }

    impl Coordinate {
        pub fn new(x: i32, y: i32) -> Self {
            Self { x, y }
        }

        pub fn rotate_right(&self) -> Self {
            Self {
                x: self.y,
                y: -self.x,
            }
        }

        pub fn rotate_left(&self) -> Self {
            Self {
                x: -self.y,
                y: self.x,
            }
        }

        pub fn translate_up(&self) -> Self {
            Self {
                x: self.x,
                y: self.y + 1,
            }
        }

        pub fn translate_down(&self) -> Self {
            Self {
                x: self.x,
                y: self.y - 1,
            }
        }

        pub fn translate_left(&self) -> Self {
            Self {
                x: self.x - 1,
                y: self.y,
            }
        }

        pub fn translate_right(&self) -> Self {
            Self {
                x: self.x + 1,
                y: self.y,
            }
        }

        pub fn translate(&self, direction: &Direction) -> Self {
            match direction {
                Direction::Up => self.translate_up(),
                Direction::Right => self.translate_right(),
                Direction::Down => self.translate_down(),
                Direction::Left => self.translate_left(),
            }
        }

        pub fn slope(&self) -> Fraction {
            Fraction::from(self.y, self.x).reduce()
        }

        pub fn manhattan_distance(&self, other: &Self) -> i32 {
            (self.x - other.x).abs() + (self.y - other.y).abs()
        }

        pub fn manhattan_distance_to_origin(&self) -> i32 {
            self.manhattan_distance(&Coordinate::new(0, 0))
        }
    }

    impl std::ops::Sub<Coordinate> for Coordinate {
        type Output = Coordinate;

        fn sub(self, rhs: Coordinate) -> Self::Output {
            let Coordinate { x, y } = self;
            let Coordinate {
                x: other_x,
                y: other_y,
            } = rhs;
            Coordinate {
                x: x - other_x,
                y: y - other_y,
            }
        }
    }

    impl std::ops::Sub<&Coordinate> for &Coordinate {
        type Output = Coordinate;

        fn sub(self, rhs: &Coordinate) -> Self::Output {
            *self - *rhs
        }
    }

    impl std::ops::Add<Coordinate> for Coordinate {
        type Output = Coordinate;

        fn add(self, rhs: Coordinate) -> Self::Output {
            let Coordinate { x, y } = self;
            let Coordinate {
                x: other_x,
                y: other_y,
            } = rhs;
            Coordinate {
                x: x + other_x,
                y: y + other_y,
            }
        }
    }

    impl std::ops::Add<&Coordinate> for &Coordinate {
        type Output = Coordinate;

        fn add(self, rhs: &Coordinate) -> Self::Output {
            *self + *rhs
        }
    }

    impl std::ops::AddAssign<Coordinate> for Coordinate {
        fn add_assign(&mut self, rhs: Coordinate) {
            self.x += rhs.x;
            self.y += rhs.y;
        }
    }

    impl std::fmt::Display for Coordinate {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    #[derive(Clone, Copy)]
    pub enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    impl Direction {
        pub fn right(&self) -> Self {
            match self {
                Self::Up => Self::Right,
                Self::Right => Self::Down,
                Self::Down => Self::Left,
                Self::Left => Self::Up,
            }
        }

        pub fn left(&self) -> Self {
            match self {
                Self::Up => Self::Left,
                Self::Right => Self::Up,
                Self::Down => Self::Right,
                Self::Left => Self::Down,
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::grid::Coordinate;

    use super::{fraction::*, *};

    #[test]
    fn test_fraction() {
        assert_eq!(Fraction::normalized(-1, -1), Fraction::from(1, 1));
        assert_eq!(Fraction::normalized(-1, 1), Fraction::from(-1, 1));
        assert_eq!(Fraction::normalized(1, 1), Fraction::from(1, 1));
        assert_eq!(Fraction::normalized(1, -1), Fraction::from(-1, 1));
        assert_eq!(Fraction::normalized(2, -2), Fraction::from(-1, 1));
        assert_eq!(Fraction::normalized(30, 5), Fraction::from(6, 1));
        assert_eq!(Fraction::normalized(5, 50), Fraction::from(1, 10));
        assert_eq!(Fraction::normalized(1, 0), Fraction::PositiveUndefined);
        assert_eq!(Fraction::normalized(-1, 0), Fraction::NegativeUndefined);
        assert_eq!(Fraction::PositiveUndefined, Fraction::PositiveUndefined);
        assert_eq!(Fraction::NegativeUndefined, Fraction::NegativeUndefined);
    }

    #[test]
    fn test_coordinate() {
        let coord = Coordinate::new(1, 2);

        assert_eq!(coord.rotate_right(), Coordinate::new(2, -1));
        assert_eq!(coord.rotate_right().rotate_right(), Coordinate::new(-1, -2));
        assert_eq!(
            coord.rotate_right().rotate_right().rotate_right(),
            Coordinate::new(-2, 1)
        );
        assert_eq!(
            coord
                .rotate_right()
                .rotate_right()
                .rotate_right()
                .rotate_right(),
            Coordinate::new(1, 2)
        );
        assert_eq!(coord.rotate_left(), Coordinate::new(-2, 1));
        assert_eq!(coord.rotate_left().rotate_left(), Coordinate::new(-1, -2));
        assert_eq!(
            coord.rotate_left().rotate_left().rotate_left(),
            Coordinate::new(2, -1)
        );
        assert_eq!(
            coord
                .rotate_left()
                .rotate_left()
                .rotate_left()
                .rotate_left(),
            Coordinate::new(1, 2)
        );

        assert_eq!(Coordinate::new(2, 6).slope(), Fraction::normalized(3, 1));

        assert_eq!(
            Coordinate::new(2, 6).manhattan_distance(&Coordinate::new(0, 0)),
            8
        )
    }
}
