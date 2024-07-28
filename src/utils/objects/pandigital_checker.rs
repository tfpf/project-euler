use crate::utils;

/// Record the occurrences of all digits in a given range. Used to check
/// whether a bunch of numbers are pandigital with respect to that range.
pub struct PandigitalChecker {
    seen: [bool; 10],
    min_digit: usize,
    max_digit: usize,
}

impl PandigitalChecker {

    /// Construct a pandigital checker for digits in the given range.
    ///
    /// * `min_digit`
    /// * `max_digit`
    pub fn new(min_digit: usize, max_digit: usize) -> PandigitalChecker {
        PandigitalChecker {
            seen: [false; 10],
            min_digit,
            max_digit,
        }
    }

    /// Clear the state of the pandigital checker, so that it behaves as if it
    /// were newly constructed.
    pub fn renew(&mut self) {
        self.seen = [false; 10];
    }

    /// Update the internal array with the digits of a given number.
    ///
    /// * `num` Number to update with.
    ///
    /// Returns `true` if all digits of the number were in range and not seen
    /// earlier. Returns `false` otherwise.
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

    /// Check whether all digits in the range have been seen. The result
    /// indicates pandigitality only all previous calls to `update` returned
    /// `true` and this method returns `true`.
    pub fn check(&self) -> bool {
        self.seen
            .iter()
            .skip(self.min_digit)
            .take(self.max_digit - self.min_digit + 1)
            .all(|&digit_seen| digit_seen)
    }
}
