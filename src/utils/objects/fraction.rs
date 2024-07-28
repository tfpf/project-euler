use crate::utils;

/// Rational numbers stored as a pair of `Long`s.
#[derive(Clone)]
pub struct Fraction {
    numerator: utils::Long,
    denominator: utils::Long,
}

impl Fraction {

    /// Construct a rational number.
    ///
    /// * `numerator`
    /// * `denominator`
    ///
    /// Returns a rational number which is the ratio of the given numbers. It
    /// is not reduced to its lowest form.
    pub fn from(numerator: u32, denominator: u32) -> Fraction {
        Fraction {
            numerator: utils::Long::from(numerator),
            denominator: utils::Long::from(denominator),
        }
    }

    /// Convert the rational number into its reciprocal.
    pub fn invert(&mut self) {
        std::mem::swap(&mut self.numerator, &mut self.denominator);
    }

    /// Obtain the number of decimal digits of the numerator and denominator.
    /// Mirrors the method of the same name of `Long`.
    pub fn len(&self) -> (usize, usize) {
        (self.numerator.len(), self.denominator.len())
    }

    /// Calculate the sum of all decimal digits of the numerator and
    /// denominator. Mirrors the method of the same name of `Long`.
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
