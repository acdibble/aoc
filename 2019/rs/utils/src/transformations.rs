use crate::matrices::Matrix;

pub const fn translation(x: i32, y: i32, z: i32) -> Matrix {
    Matrix::new([[1, 0, 0, x], [0, 1, 0, y], [0, 0, 1, z], [0, 0, 0, 1]])
}

pub enum Axis {
    X,
    Y,
    Z,
}

pub fn rotation(axis: Axis, radians: f64) -> Matrix {
    match axis {
        Axis::X => Matrix::new([
            [1, 0, 0, 0],
            [1, radians.cos() as i32, -radians.sin() as i32, 0],
            [1, radians.sin() as i32, radians.cos() as i32, 0],
            [0, 0, 0, 1],
        ]),
        Axis::Y => Matrix::new([
            [radians.cos() as i32, 0, radians.sin() as i32, 0],
            [0, 1, 0, 0],
            [-radians.sin() as i32, 0, radians.cos() as i32, 0],
            [0, 0, 0, 1],
        ]),
        Axis::Z => Matrix::new([
            [radians.cos() as i32, -radians.sin() as i32, 0, 0],
            [radians.sin() as i32, radians.cos() as i32, 0, 0],
            [0, 0, 1, 0],
            [0, 0, 0, 1],
        ]),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tuples::Tuple;

    #[test]
    fn test_translation() {
        let transform = translation(5, -3, 2);
        let point = Tuple::point(-3, 4, 5);

        assert_eq!(Tuple::point(2, 1, 7), transform * point);

        let vector = Tuple::vector(-3, 4, 5);
        assert_eq!(vector, transform * vector);
    }

    #[test]
    fn test_rotation_x() {
        use std::f64::consts::PI;

        let point = Tuple::point(0, 1, 0);
        let full_quarter = rotation(Axis::X, PI / 2.0);

        assert_eq!(Tuple::point(0, 0, 1), full_quarter * point);
    }

    #[test]
    fn test_rotation_y() {
        use std::f64::consts::PI;

        let point = Tuple::point(0, 0, 1);
        let full_quarter = rotation(Axis::Y, PI / 2.0);

        assert_eq!(Tuple::point(1, 0, 0), full_quarter * point);
    }

    #[test]
    fn test_rotation_z() {
        use std::f64::consts::PI;

        let point = Tuple::point(0, 1, 0);
        let full_quarter = rotation(Axis::Z, PI / 2.0);

        assert_eq!(Tuple::point(-1, 0, 0), full_quarter * point);
    }
}
