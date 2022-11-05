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
                let divisor = gcd(numerator, denominator);

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

#[cfg(test)]
mod test {
    use super::*;

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
}
