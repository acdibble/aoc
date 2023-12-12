#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    fn translate(&self, dx: i32, dy: i32) -> Self {
        Self {
            x: self.x + dx,
            y: self.y + dy,
        }
    }

    pub fn translate_up(&self) -> Self {
        self.translate(0, -1)
    }

    pub fn translate_down(&self) -> Self {
        self.translate(0, 1)
    }

    pub fn translate_left(&self) -> Self {
        self.translate(-1, 0)
    }

    pub fn translate_right(&self) -> Self {
        self.translate(1, 0)
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
