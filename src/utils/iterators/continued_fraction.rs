use crate::utils;

/// Generate the continued fraction representation of the square root of a
/// number. The elements generated after the first shall constitute the
/// repeating terms in the continued fraction.
pub struct ContinuedFraction {
    num: i64,
    a0: i64,
    numerator_addend: i64,
    denominator: i64,
}

impl ContinuedFraction {
    pub fn new(num: i64) -> ContinuedFraction {
        ContinuedFraction {
            num,
            a0: utils::isqrt(num),
            numerator_addend: 0,
            denominator: 1,
        }
    }
}

impl Iterator for ContinuedFraction {
    type Item = i64;
    fn next(&mut self) -> Option<i64> {
        // This will happen if the given number was a perfect square. We will
        // also use this as the condition for detecting repeating terms.
        if self.denominator == 0 {
            return None;
        }

        // If a part of the continued fraction is
        //     (num.sqrt() + numerator_addend) / denominator
        // then the next term of the continued fraction, `numerator_addend` and
        // `denominator` can be found using a recurrence relation.
        let a = (self.a0 + self.numerator_addend) / self.denominator;
        self.numerator_addend = a * self.denominator - self.numerator_addend;
        self.denominator = (self.num - self.numerator_addend.pow(2)) / self.denominator;

        // When this happens, the terms will start repeating.
        if a == self.a0 * 2 {
            self.denominator = 0;
        }
        Some(a)
    }
}
