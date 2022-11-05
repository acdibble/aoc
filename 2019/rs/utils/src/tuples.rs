use crate::transformations::*;
use std::ops;

const POINT: i32 = 1;
const VECTOR: i32 = 0;

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub struct Tuple {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub(crate) kind: i32,
}

impl std::fmt::Display for Tuple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}({}, {}, {})",
            if self.kind == POINT {
                "Point"
            } else {
                "Vector"
            },
            self.x,
            self.y,
            self.z
        )
    }
}

impl From<[i32; 4]> for Tuple {
    fn from([x, y, z, kind]: [i32; 4]) -> Self {
        Self::new(x, y, z, kind)
    }
}

impl Tuple {
    fn new(x: i32, y: i32, z: i32, kind: i32) -> Self {
        Self { x, y, z, kind }
    }

    pub fn point(x: i32, y: i32, z: i32) -> Self {
        Self::new(x, y, z, POINT)
    }

    pub fn vector(x: i32, y: i32, z: i32) -> Self {
        Self::new(x, y, z, VECTOR)
    }

    pub fn translate(self, x: i32, y: i32, z: i32) -> Self {
        translation(x, y, z) * self
    }

    pub fn rotate_x(self, radians: f64) -> Self {
        rotation(Axis::X, radians) * self
    }

    pub fn rotate_y(self, radians: f64) -> Self {
        rotation(Axis::Y, radians) * self
    }

    pub fn rotate_z(self, radians: f64) -> Self {
        rotation(Axis::Z, radians) * self
    }

    pub fn is_vector(&self) -> bool {
        self.kind == VECTOR
    }

    pub fn is_point(&self) -> bool {
        self.kind == POINT
    }
}

impl ops::Add<Self> for Tuple {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Tuple::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
            self.kind + other.kind,
        )
    }
}

impl ops::Sub<Self> for Tuple {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Tuple::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
            self.kind - other.kind,
        )
    }
}

impl ops::AddAssign<Self> for Tuple {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.kind += rhs.kind;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_point() {
        let point = Tuple::point(4, -4, 3);
        assert_eq!(4, point.x);
        assert_eq!(-4, point.y);
        assert_eq!(3, point.z);
        assert_eq!(POINT, point.kind);
        assert!(point.is_point());
        assert!(!point.is_vector());
    }

    #[test]
    fn test_vector() {
        let point = Tuple::vector(4, -4, 3);
        assert_eq!(4, point.x);
        assert_eq!(-4, point.y);
        assert_eq!(3, point.z);
        assert_eq!(VECTOR, point.kind);
        assert!(!point.is_point());
        assert!(point.is_vector())
    }

    #[test]
    fn test_add() {
        let a1 = Tuple::vector(3, -2, 5);
        let a2 = Tuple::vector(-2, 3, 1);

        assert_eq!(Tuple::vector(1, 1, 6), a1 + a2);
    }

    #[test]
    fn test_sub() {
        let p1 = Tuple::point(3, 2, 1);
        let p2 = Tuple::point(5, 6, 7);

        assert_eq!(Tuple::vector(-2, -4, -6), p1 - p2);

        let p = Tuple::point(3, 2, 1);
        let v = Tuple::vector(5, 6, 7);
        assert_eq!(Tuple::point(-2, -4, -6), p - v);

        let v1 = Tuple::vector(3, 2, 1);
        let v2 = Tuple::vector(5, 6, 7);
        assert_eq!(Tuple::vector(-2, -4, -6), v1 - v2);
    }
}
