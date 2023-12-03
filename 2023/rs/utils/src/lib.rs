#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point {
    pub x: i32,
    pub y: i32,
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
