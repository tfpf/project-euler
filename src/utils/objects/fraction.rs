use crate::utils;

/// Rational numbers.
#[derive(Clone)]
pub struct Fraction {
    numerator: utils::Long,
    denominator: utils::Long,
}
impl Fraction {
    pub fn from(numerator: u32, denominator: u32) -> Fraction {
        Fraction {
            numerator: utils::Long::from(numerator),
            denominator: utils::Long::from(denominator),
        }
    }
    pub fn invert(&mut self) {
        std::mem::swap(&mut self.numerator, &mut self.denominator);
    }
    pub fn len(&self) -> (usize, usize) {
        (self.numerator.len(), self.denominator.len())
    }
    pub fn sum(&self) -> (i64, i64) {
        (self.numerator.sum(), self.denominator.sum())
    }
}
impl std::ops::AddAssign<u32> for Fraction {
    fn add_assign(&mut self, other: u32) {
        self.numerator += &(&self.denominator * other);
    }
}
impl std::ops::Add<u32> for &Fraction {
    type Output = Fraction;
    fn add(self, other: u32) -> Fraction {
        let mut result = self.clone();
        result += other;
        result
    }
}
impl std::fmt::Display for Fraction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}
