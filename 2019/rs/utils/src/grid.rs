use crate::fractions::Fraction;

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

#[cfg(test)]
mod test {
    use super::*;

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
