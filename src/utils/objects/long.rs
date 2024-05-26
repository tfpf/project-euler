#![allow(clippy::manual_try_fold)]
#![allow(clippy::len_without_is_empty)]

use crate::utils;

/// Arbitrary-precision integer type which stores digits of a positive number
/// in base 1_000_000_000.
#[derive(Clone, Eq, PartialEq)]
pub struct Long {
    digits: Vec<u32>,
}
impl Long {
    /// Construct an arbitrary-precision integer from a big-endian string of
    /// decimal digits.
    ///
    /// * `s`
    ///
    /// -> Arbitrary-precision integer.
    pub fn new(s: &str) -> Long {
        let mut long = Long { digits: vec![] };
        let mut idx = s.len();
        loop {
            let (lower, upper) = (std::cmp::max(idx, 9) - 9, idx);
            long.digits.push(s[lower..upper].parse().unwrap());
            if lower == 0 {
                break;
            }
            idx = lower;
        }
        long
    }
    pub fn from(digit: u32) -> Long {
        if digit >= 1_000_000_000 {
            panic!("argument is too large to be a digit of this arbitrary-precision type");
        }
        Long { digits: vec![digit] }
    }
    pub fn reverse(&self) -> Long {
        Long::new(&self.to_string().chars().rev().collect::<String>())
    }
    /// Calculate the factorial of a non-negative number.
    ///
    /// * `num` Number whose factorial is to be calculated.
    pub fn factorial(num: u32) -> Long {
        if num == 0 || num == 1 {
            return Long::from(1);
        }
        if num == 2 {
            return Long::from(2);
        }

        // Multiply the extremes, converging towards the centre. For example,
        // 9! is computed as (9 × 1) × (8 × 2) × (7 × 3) × (6 × 4) × 5, whereas
        // 10! is computed as (10 × 1) × (9 × 2) × (8 × 3) × (7 × 4) × (6 × 5).
        let partials = num / 2;
        let mut result = if num % 2 == 1 {
            Long::from(partials + 1)
        } else {
            Long::from(1)
        };
        let (mut multiplicand, mut delta) = (num, num);
        for _ in 0..partials {
            result *= multiplicand;
            delta -= 2;
            multiplicand += delta;
        }
        result
    }
    /// Obtain the number of decimal digits of this number (i.e. its length).
    ///
    /// -> Length.
    pub fn len(&self) -> usize {
        match self.digits.len() {
            0 => 0,
            1 if self.digits[0] == 0 => 0,
            len => (len - 1) * 9 + self.digits.last().unwrap().to_string().len(),
        }
    }
    /// Calculate the sum of all decimal digits of this number.
    ///
    /// -> Sum.
    pub fn sum(&self) -> i64 {
        self.digits
            .iter()
            .map(|&digit| utils::Digits::new(digit as i64).sum::<i64>())
            .sum()
    }
    /// Raise this number to the given power.
    ///
    /// * `exp` Power.
    ///
    /// -> Value of this number raised to the given power.
    pub fn pow(&self, mut exp: u32) -> Long {
        // Multiplication is expensive, so these checks will improve
        // performance.
        let mut multiplier = Long::from(1);
        if exp == 0 {
            return multiplier;
        }
        let mut base = self.clone();
        if exp == 1 {
            return base;
        }
        loop {
            if exp % 2 == 1 {
                multiplier = &multiplier * &base;
            }
            if exp == 1 {
                return multiplier;
            }
            exp /= 2;
            base = &base * &base;
        }
    }
    fn adc(a: u32, b: u32, carry: bool) -> (u32, bool) {
        let sum = a + b + carry as u32;
        if sum >= 1_000_000_000 {
            (sum - 1_000_000_000, true)
        } else {
            (sum, false)
        }
    }
    fn mlc(a: u32, b: u32, carry: u32) -> (u32, u32) {
        let product = a as u64 * b as u64 + carry as u64;
        ((product % 1_000_000_000) as u32, (product / 1_000_000_000) as u32)
    }
}
impl std::ops::AddAssign<&Long> for Long {
    fn add_assign(&mut self, other: &Long) {
        self.digits
            .resize(std::cmp::max(self.digits.len(), other.digits.len()), 0);
        let mut carry = false;
        for (sd, od) in self
            .digits
            .iter_mut()
            .zip(other.digits.iter().chain(std::iter::repeat(&0)))
        {
            (*sd, carry) = Long::adc(*sd, *od, carry);
        }
        if carry {
            self.digits.push(1);
        }
    }
}
impl std::ops::Add<&Long> for &Long {
    type Output = Long;
    fn add(self, other: &Long) -> Long {
        let mut result = self.clone();
        result += other;
        result
    }
}
impl std::ops::MulAssign<u32> for Long {
    fn mul_assign(&mut self, other: u32) {
        let mut carry = 0;
        for sd in self.digits.iter_mut() {
            (*sd, carry) = Long::mlc(*sd, other, carry);
        }
        while carry > 0 {
            self.digits.push(carry % 1_000_000_000);
            carry /= 1_000_000_000;
        }
    }
}
impl std::ops::Mul<u32> for &Long {
    type Output = Long;
    fn mul(self, other: u32) -> Long {
        let mut result = self.clone();
        result *= other;
        result
    }
}
impl std::ops::Mul<&Long> for &Long {
    type Output = Long;
    fn mul(self, other: &Long) -> Long {
        let mut result = Long { digits: vec![] };
        for (pad, od) in other.digits.iter().enumerate() {
            let mut partial_product = Long { digits: vec![0; pad] };
            let mut carry = 0;
            for sd in self.digits.iter() {
                let sum_carry = Long::mlc(*sd, *od, carry);
                partial_product.digits.push(sum_carry.0);
                carry = sum_carry.1;
            }
            if carry > 0 {
                partial_product.digits.push(carry);
            }
            result += &partial_product;
        }
        result
    }
}
impl std::fmt::Display for Long {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.digits
            .iter()
            .rev()
            .enumerate()
            .fold(Ok(()), |result, (index, digit)| {
                result.and_then(|_| {
                    if index == 0 {
                        write!(f, "{}", digit)
                    } else {
                        write!(f, "{:0>9}", digit)
                    }
                })
            })
    }
}
impl std::fmt::Debug for Long {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let result = write!(f, "digits = |");
        self.digits
            .iter()
            .rev()
            .fold(result, |result, &digit| result.and_then(|_| write!(f, "{}|", digit)))
    }
}
