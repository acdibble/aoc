use crate::tuples::*;
use std::fmt::Debug;
use std::ops;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Matrix([[i32; 4]; 4]);

impl Matrix {
    pub const fn new(input: [[i32; 4]; 4]) -> Self {
        Self(input)
    }
}

impl ops::Index<usize> for Matrix {
    type Output = [i32; 4];

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl ops::Mul<Self> for Matrix {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        let mut output = [[0; 4]; 4];

        for row in 0..4 {
            for col in 0..4 {
                output[row][col] = (0..4).fold(0, |acc, n| acc + self[row][n] * other[n][col]);
            }
        }

        Matrix::new(output)
    }
}

impl ops::Mul<Tuple> for Matrix {
    type Output = Tuple;

    fn mul(self, Tuple { x, y, z, kind }: Tuple) -> Self::Output {
        let mut results = [0; 4];

        for (i, [a, b, c, d]) in self.0.into_iter().enumerate() {
            results[i] = a * x + b * y + c * z + d * kind;
        }

        Tuple::from(results)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_constructor() {
        let matrix = Matrix::new([
            [1, 2, 3, 4],
            [5, 6, 7, 8],
            [9, 10, 11, 12],
            [13, 14, 15, 16],
        ]);

        assert_eq!(1, matrix[0][0]);
        assert_eq!(4, matrix[0][3]);
        assert_eq!(5, matrix[1][0]);
        assert_eq!(7, matrix[1][2]);
        assert_eq!(11, matrix[2][2]);
        assert_eq!(13, matrix[3][0]);
        assert_eq!(15, matrix[3][2]);
    }

    #[test]
    fn test_equal() {
        let a = Matrix::new([[1, 2, 3, 4], [5, 6, 7, 8], [9, 8, 7, 6], [5, 4, 3, 2]]);
        let b = Matrix::new([[1, 2, 3, 4], [5, 6, 7, 8], [9, 8, 7, 6], [5, 4, 3, 2]]);

        assert_eq!(a, b);

        let a = Matrix::new([[1, 2, 3, 4], [5, 6, 7, 8], [9, 8, 7, 6], [5, 4, 3, 2]]);
        let b = Matrix::new([[5, 4, 3, 2], [9, 8, 7, 6], [5, 6, 7, 8], [1, 2, 3, 4]]);

        assert_ne!(a, b);
    }

    #[test]
    fn test_mul() {
        let a = Matrix::new([[1, 2, 3, 4], [5, 6, 7, 8], [9, 8, 7, 6], [5, 4, 3, 2]]);

        let b = Matrix::new([[-2, 1, 2, 3], [3, 2, 1, -1], [4, 3, 6, 5], [1, 2, 7, 8]]);

        let c = Matrix::new([
            [20, 22, 50, 48],
            [44, 54, 114, 108],
            [40, 58, 110, 102],
            [16, 26, 46, 42],
        ]);

        assert_eq!(c, a * b);
    }

    #[test]
    fn test_mul_tuple() {
        let matrix = Matrix::new([[1, 2, 3, 4], [2, 4, 4, 2], [8, 6, 4, 1], [0, 0, 0, 1]]);
        let tuple = Tuple::point(1, 2, 3);

        assert_eq!(Tuple::point(18, 24, 33), matrix * tuple)
    }

    #[test]
    fn test_identity() {
        let identity = [[1, 0, 0, 0], [0, 1, 0, 0], [0, 0, 1, 0], [0, 0, 0, 1]];

        let matrix = Matrix::new([[0, 1, 2, 4], [1, 2, 4, 8], [2, 4, 8, 16], [4, 8, 16, 32]]);

        assert_eq!(matrix.clone(), matrix * Matrix::new(identity));

        let tuple = Tuple::point(1, 2, 3);

        assert_eq!(tuple, Matrix::new(identity) * tuple);
    }
}
