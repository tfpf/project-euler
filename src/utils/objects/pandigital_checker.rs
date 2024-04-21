use crate::utils;

/// Check whether a bunch of numbers are pandigital with respect to the given
/// range.
pub struct PandigitalChecker {
    seen: [bool; 10],
    min_digit: usize,
    max_digit: usize,
}
impl PandigitalChecker {
    pub fn new(min_digit: usize, max_digit: usize) -> PandigitalChecker {
        PandigitalChecker {
            seen: [false; 10],
            min_digit,
            max_digit,
        }
    }
    pub fn renew(&mut self) {
        self.seen = [false; 10];
    }
    /// Update the internal array with the digits of a given number.
    ///
    /// * `num` - Number to update with.
    ///
    /// -> Whether all digits of the number were in range and not seen earlier.
    pub fn update(&mut self, num: i64) -> bool {
        for digit in utils::Digits::new(num) {
            let digit = digit as usize;
            if digit < self.min_digit || digit > self.max_digit || self.seen[digit] {
                return false;
            }
            self.seen[digit] = true;
        }
        true
    }
    /// Check whether all digits in the range have been seen. This indicates
    /// pandigitality only if used in tandem with the above method.
    ///
    /// -> Pandigitality.
    pub fn check(&self) -> bool {
        self.seen
            .iter()
            .skip(self.min_digit)
            .take(self.max_digit - self.min_digit + 1)
            .all(|&digit_seen| digit_seen)
    }
}
