/******************************************************************************
 * Functions.
 *****************************************************************************/

/// Check whether the given number is prime by searching for any prime factors.
/// Use the fact that a prime factor, by virtue of being prime, is 2 or 3, or
/// differs from 6 by exactly 1.
///
/// * `num` - Number to check for primality.
///
/// -> Whether `num` is prime.
pub fn is_prime(num: i64) -> bool {
    if num == 2 || num == 3 {
        return true;
    }
    if num < 2 || num % 2 == 0 || num % 3 == 0 {
        return false;
    }
    for candidate in (5i64..)
        .step_by(6)
        .take_while(|candidate| candidate.pow(2) <= num)
    {
        if num % candidate == 0 || num % (candidate + 2) == 0 {
            return false;
        }
    }
    true
}

/// Check whether a number is a palindrome or not.
///
/// * `num` - Number to check for palindromicity.
/// * `radix` - Base to use to represent the number: 2, 10 or 16.
///
/// -> Whether `num` is a palindrome in the specified base.
pub fn is_palindrome(num: i64, radix: i32) -> bool {
    let num = match radix {
        2 => format!("{:b}", num),
        10 => format!("{}", num),
        16 => format!("{:x}", num),
        _ => return false,
    };
    num == num.chars().rev().collect::<String>()
}

/// Calculate the greatest common divisor of two numbers.
///
/// * `a`
/// * `b`
///
/// -> GCD of the inputs.
pub fn gcd(a: i64, b: i64) -> i64 {
    let (a, b) = if a > b { (a, b) } else { (b, a) };
    if b == 0 {
        return a;
    }
    let remainder = a % b;
    if remainder == 0 {
        b
    } else {
        gcd(b, remainder)
    }
}

/// Check for leap years.
///
/// * `year`
///
/// -> Whether the input is a leap year.
pub fn is_leap(year: i64) -> bool {
    year % 4 == 0 && year % 100 != 0 || year % 400 == 0
}

/// Determine the number of days in the given month of the given year.
///
/// * `year`
/// * `month` - Number from 1 to 12.
///
/// -> Number of days, or 0 if the month is invalid.
pub fn days_in(year: i64, month: i64) -> i64 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap(year) {
                29
            } else {
                28
            }
        }
        _ => 0,
    }
}

/// Find the length of the repeating part of the decimal representation of the
/// reciprocal of a prime number.
///
/// * `prime` - Prime number.
///
/// -> Recurrence cycle length.
pub fn recurrence_length(prime: i64) -> i64 {
    // The digits (i.e. the sequence of quotients) will start repeating when
    // the remainder becomes 1 for the second time.
    let mut rem = 1;
    for length in 1.. {
        rem = rem * 10 % prime;
        if rem == 1 {
            return length;
        }
    }
    unreachable!();
}

/// Construct the sieve of Eratosthenes.
///
/// * `limit` - Number up to which the sieve should be constructed.
///
/// -> Boolean vector indicating the primality of its indices.
pub fn sieve_of_eratosthenes(limit: usize) -> Vec<bool> {
    let mut sieve = vec![true; limit + 1];
    sieve[0] = false;
    sieve[1] = false;
    for num in (2..).take_while(|num| num * num <= limit) {
        // If this number is prime, mark its multiples starting from its square
        // as composite. (Smaller multiples have already been marked as
        // composite.)
        if sieve[num] {
            for multiple in (num * num..=limit).step_by(num) {
                sieve[multiple] = false;
            }
        }
    }
    sieve
}

/// Generate the next permutation.
///
/// * `slice` - Object containing the unique items to permute.
///
/// -> Whether the next permutation was generated.
pub fn next_permutation<T: Copy + std::cmp::Ord>(slice: &mut [T]) -> bool {
    // Locate an inversion from the right.
    let mut sorted_until: usize = usize::MAX;
    for idx in (1..slice.len()).rev() {
        if slice[idx - 1] < slice[idx] {
            sorted_until = idx;
            break;
        }
    }
    if sorted_until == usize::MAX {
        return false;
    }

    // Find out where it can be placed while maintaining sorted order from the
    // right. (That means reverse order when looking from the left.)
    // Decrementing the search result is necessary to reorient ourselves from
    // looking from the right to looking from the left.
    let search_key = slice[sorted_until - 1];
    let target = sorted_until
        + match slice[sorted_until..].binary_search_by(|element| element.cmp(&search_key).reverse())
        {
            Ok(idx) => idx,
            Err(idx) => idx,
        }
        - 1;
    (slice[sorted_until - 1], slice[target]) = (slice[target], slice[sorted_until - 1]);
    slice[sorted_until..].reverse();
    true
}

/// Generate the previous permutation.
///
/// * `slice` - Object containing the unique items to permute.
///
/// -> Whether the previous permutation was generated.
pub fn prev_permutation<T: Copy + std::cmp::Ord>(slice: &mut [T]) -> bool {
    // Locate an anti-inversion from the right.
    let mut sorted_until: usize = usize::MAX;
    for idx in (1..slice.len()).rev() {
        if slice[idx - 1] > slice[idx] {
            sorted_until = idx;
            break;
        }
    }
    if sorted_until == usize::MAX {
        return false;
    }

    // Find out where it can be placed while maintaining reverse order from the
    // right. (That means sorted order when looking from the left.)
    // Decrementing the search result is necessary to avoid overshooting.
    let search_key = slice[sorted_until - 1];
    let target = sorted_until
        + match slice[sorted_until..].binary_search(&search_key) {
            Ok(idx) => idx,
            Err(idx) => idx,
        }
        - 1;
    (slice[sorted_until - 1], slice[target]) = (slice[target], slice[sorted_until - 1]);
    slice[sorted_until..].reverse();
    true
}

/******************************************************************************
 * Objects.
 *****************************************************************************/

/// Arbitrary-precision integer type which stores digits of a positive number
/// in base 1_000_000_000. Implements addition by reference.
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Long {
    digits: Vec<i32>,
}
impl Long {
    pub fn new(s: &str) -> Long {
        let mut long = Long { digits: vec![] };
        for chunk in s.bytes().rev().collect::<Vec<u8>>().chunks(9) {
            let digit = chunk
                .iter()
                .rev()
                .fold(0i32, |digit, &element| digit * 10 + element as i32 - 0x30);
            long.digits.push(digit);
        }
        long
    }
    /// Obtain the number of decimal digits in this number (i.e. its length).
    ///
    /// -> Length.
    pub fn len(&self) -> usize {
        match self.digits.len() {
            0 => 0,
            1 if self.digits[0] == 0 => 0,
            len => (len - 1) * 9 + self.digits.last().unwrap().to_string().len(),
        }
    }
    fn adc(a: i32, b: i32, carry: i32) -> (i32, i32) {
        let sum = a + b + carry;
        if sum >= 1_000_000_000 {
            (sum - 1_000_000_000, 1)
        } else {
            (sum, 0)
        }
    }
    fn mlc(a: i32, b: i32, carry: i32) -> (i32, i32) {
        let product = a as i64 * b as i64 + carry as i64;
        (
            (product % 1_000_000_000) as i32,
            (product / 1_000_000_000) as i32,
        )
    }
}
impl std::ops::AddAssign<&Long> for Long {
    fn add_assign(&mut self, other: &Long) {
        let mut carry = 0;
        for (sd, od) in self.digits.iter_mut().zip(other.digits.iter()) {
            (*sd, carry) = Long::adc(*sd, *od, carry);
        }
        for od in other.digits.iter().skip(self.digits.len()) {
            let sum_carry = Long::adc(0, *od, carry);
            self.digits.push(sum_carry.0);
            carry = sum_carry.1;
        }
        if carry > 0 {
            self.digits.push(carry);
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
impl std::ops::MulAssign<i32> for Long {
    fn mul_assign(&mut self, other: i32) {
        let mut carry = 0;
        for sd in self.digits.iter_mut() {
            (*sd, carry) = Long::mlc(*sd, other, carry);
        }
        if carry > 0 {
            self.digits.push(carry);
        }
    }
}
impl std::ops::Mul<i32> for &Long {
    type Output = Long;
    fn mul(self, other: i32) -> Long {
        let mut result = self.clone();
        result *= other;
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
            min_digit: min_digit,
            max_digit: max_digit,
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
        for digit in Digits::new(num) {
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
            .fold(true, |pandigital, &digit_seen| pandigital && digit_seen)
    }
}

/******************************************************************************
 * Iterators.
 *****************************************************************************/

/// Fibonacci sequence iterator.
pub struct Fibonacci {
    a: i64,
    b: i64,
}
impl Fibonacci {
    pub fn new(a: i64, b: i64) -> Fibonacci {
        Fibonacci { a: a, b: b }
    }
}
impl Iterator for Fibonacci {
    type Item = i64;
    fn next(&mut self) -> Option<i64> {
        let a = self.a;
        (self.a, self.b) = (self.b, self.a + self.b);
        Some(a)
    }
}

/// Triangular, pentagonal, hexagonal, etc. number iterator. Specify the
/// number of sides of the polygon as the argument to the constructor.
pub struct Polygonal {
    increment: i64,
    offset: i64,
    num: i64,
}
impl Polygonal {
    pub fn new(sides: i64) -> Polygonal {
        Polygonal {
            increment: sides - 2,
            offset: 1,
            num: 0,
        }
    }
}
impl Iterator for Polygonal {
    type Item = i64;
    fn next(&mut self) -> Option<i64> {
        self.num += self.offset;
        self.offset += self.increment;
        Some(self.num)
    }
}

/// Collatz sequence iterator.
pub struct Collatz {
    num: i64,
    done: bool,
}
impl Collatz {
    pub fn new(num: i64) -> Collatz {
        Collatz {
            num: num,
            done: false,
        }
    }
}
impl Iterator for Collatz {
    type Item = i64;
    fn next(&mut self) -> Option<i64> {
        if self.done {
            return None;
        }
        let num = self.num;
        self.num = if self.num % 2 == 0 {
            self.num / 2
        } else {
            3 * self.num + 1
        };
        self.done = num == 1;
        Some(num)
    }
}

/// Divisors iterator. Generates all divisors of a number in an unspecified
/// order. Positive numbers only!
pub struct Divisors {
    dividend: i64,
    limit: i64,
    current: i64,
    other: i64,
}
impl Divisors {
    pub fn new(dividend: i64) -> Divisors {
        Divisors {
            dividend: dividend,
            limit: (dividend as f64).sqrt() as i64,
            current: 0,
            other: 0,
        }
    }
}
impl Iterator for Divisors {
    type Item = i64;
    fn next(&mut self) -> Option<i64> {
        if self.other > 0 {
            let other = self.other;
            self.other = 0;
            return Some(other);
        }
        loop {
            self.current += 1;
            if self.current > self.limit {
                return None;
            }
            if self.dividend % self.current == 0 {
                let other = self.dividend / self.current;
                if other != self.current {
                    self.other = other;
                }
                return Some(self.current);
            }
        }
    }
}

/// Primes iterator. Generates prime numbers by internally constructing the
/// sieve of Eratosthenes. Differs from other iterators in that its method must
/// be called in order to obtain an iterator.
pub struct Primes {
    sieve: Vec<bool>,
}
impl Primes {
    pub fn new(limit: usize) -> Primes {
        Primes {
            sieve: sieve_of_eratosthenes(limit),
        }
    }
    pub fn sieve(&self) -> &Vec<bool> {
        &self.sieve
    }
    pub fn iter(&self) -> impl Iterator<Item = i64> + '_ {
        self.sieve
            .iter()
            .enumerate()
            .filter(|(_, is_prime)| **is_prime)
            .map(|(num, _)| num as i64)
    }
}

/// Digits iterator. Generates the decimal digits of a number from least
/// significant to most significant. Positive numbers only!
pub struct Digits {
    num: i64,
}
impl Digits {
    pub fn new(num: i64) -> Digits {
        Digits { num: num }
    }
}
impl Iterator for Digits {
    type Item = i64;
    fn next(&mut self) -> Option<i64> {
        if self.num == 0 {
            None
        } else {
            let digit = self.num % 10;
            self.num /= 10;
            Some(digit)
        }
    }
}

/// Bits iterator. Generates the binary digits of a number from least
/// significant to most significant. Positive numbers only!
pub struct Bits {
    num: i64,
}
impl Bits {
    pub fn new(num: i64) -> Bits {
        Bits { num: num }
    }
}
impl Iterator for Bits {
    type Item = i64;
    fn next(&mut self) -> Option<i64> {
        if self.num == 0 {
            None
        } else {
            let bit = self.num & 1;
            self.num >>= 1;
            Some(bit)
        }
    }
}

/// Pythagorean triplets iterator. Generates all Pythagorean triplets with the
/// given sum. The triplets are generated using a well-known parametrisation
/// which can represent any primitive triplet. Positive numbers only!
pub struct PythagoreanTriplets {
    semiperimeter: i64,
    m_ub: i64,
    m: i64,
}
impl PythagoreanTriplets {
    pub fn new(perimeter: i64) -> PythagoreanTriplets {
        let semiperimeter = perimeter / 2;
        // Non-strict upper bound for the parameter `m`.
        let m_ub = (semiperimeter as f64).sqrt() as i64;
        PythagoreanTriplets {
            semiperimeter: semiperimeter,
            m_ub: m_ub,
            m: 1,
        }
    }
}
impl Iterator for PythagoreanTriplets {
    type Item = (i64, i64, i64);
    fn next(&mut self) -> Option<(i64, i64, i64)> {
        loop {
            self.m += 1;
            if self.m > self.m_ub {
                return None;
            }
            if self.semiperimeter % self.m != 0 {
                continue;
            }
            let m = self.m;
            let remaining_term = self.semiperimeter / m;
            let m_plus_n_lb = m + 1 + (m & 1);
            for m_plus_n in (m_plus_n_lb..)
                .step_by(2)
                .take_while(|&m_plus_n| m_plus_n < 2 * m)
            {
                if remaining_term % m_plus_n == 0 && gcd(m_plus_n, m) == 1 {
                    let d = remaining_term / m_plus_n;
                    let n = m_plus_n - m;
                    let a = (m.pow(2) - n.pow(2)) * d;
                    let b = 2 * m * n * d;
                    let c = (m.pow(2) + n.pow(2)) * d;
                    return Some((a, b, c));
                }
            }
        }
    }
}
