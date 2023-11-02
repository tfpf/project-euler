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
    if num % 2 == 0 || num % 3 == 0 {
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

/// Construct the sieve of Eratosthenes.
///
/// * `limit` - Number up to which the sieve should be constructed.
///
/// -> Vector in which each element indicates the primality of its index.
pub fn sieve_of_eratosthenes(limit: usize) -> Vec<bool> {
    let mut prime = vec![true; limit + 1];
    prime[0] = false;
    prime[1] = false;
    for num in (2usize..).take_while(|num| num * num <= limit) {
        // If this number is prime, mark its multiples starting from its square
        // as composite. (Smaller multiples have already been marked as
        // composite.)
        if prime[num] {
            for multiple in (num * num..=limit).step_by(num) {
                prime[multiple] = false;
            }
        }
    }
    prime
}

/// Check whether a number is a palindrome or not.
///
/// * `num` - Number to check for palindromicity.
///
/// -> Whether `num` written using decimal digits is a palindrome.
pub fn is_palindrome<T>(num: T) -> bool
where
    T: std::fmt::Display,
{
    let num = num.to_string();
    num == num.chars().rev().collect::<String>()
}

/// Count the divisors of the given number.
///
/// * `num` - Number to count the divisors of.
///
/// -> Number of divisors.
pub fn count_divisors(num: i64) -> i64 {
    (1i64..)
        .take_while(|candidate| candidate.pow(2) <= num)
        .map(|candidate| {
            if num % candidate == 0 {
                // This number is a divisor, which means we have potentially
                // found another divisor as well.
                if num / candidate != candidate {
                    2
                } else {
                    1
                }
            } else {
                0
            }
        })
        .sum()
}

/// Calculate the greatest common divisor of two numbers.
///
/// * `a`
/// * `b`
///
/// -> GCD of the inputs.
pub fn gcd(a: i64, b: i64) -> i64 {
    let (a, b) = if a > b { (a, b) } else { (b, a) };
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
    /// Obtain the number of decimal digits in this number.
    pub fn len(&self) -> usize {
        match self.digits.len() {
            0 => 0,
            1 if self.digits[0] == 0 => 0,
            len => (len - 1) * 9 + self.digits.last().unwrap().to_string().len(),
        }
    }
    /// Obtain the bottom 64 bits of this number as a signed integer.
    pub fn get(&self) -> i64 {
        match self.digits.len() {
            0 => 0,
            1 => self.digits[0] as i64,
            _ => self.digits[0] as i64 + 2i64.pow(32) * self.digits[1] as i64,
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
impl std::iter::Sum for Long {
    fn sum<I>(iter: I) -> Long
    where
        I: Iterator<Item = Long>,
    {
        iter.fold(Long::new("0"), |sum, element| &sum + &element)
    }
}
impl std::cmp::PartialEq<i64> for Long {
    fn eq(&self, other: &i64) -> bool {
        self.eq(&Long::new(&other.to_string()))
    }
}
impl std::cmp::PartialOrd<i64> for Long {
    fn partial_cmp(&self, other: &i64) -> Option<std::cmp::Ordering> {
        self.partial_cmp(&Long::new(&other.to_string()))
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

/******************************************************************************
 * Iterators.
 *****************************************************************************/

/// Fibonacci sequence iterator which produces items containing the values
/// rather than references to the values. Positive numbers only!
pub struct Fibonacci {
    a: Long,
    b: Long,
}
impl Fibonacci {
    pub fn new(a: Long, b: Long) -> Fibonacci {
        Fibonacci { a: a, b: b }
    }
}
impl Iterator for Fibonacci {
    type Item = Long;
    fn next(&mut self) -> Option<Long> {
        let a = self.a.clone();
        self.a = self.b.clone();
        self.b += &a;
        Some(a)
    }
}

/// Triangular number iterator which produces items containing the values
/// rather than references to the values.
pub struct Triangular {
    idx: i64,
    num: i64,
}
impl Triangular {
    pub fn new() -> Triangular {
        Triangular { idx: 1, num: 0 }
    }
}
impl Iterator for Triangular {
    type Item = i64;
    fn next(&mut self) -> Option<i64> {
        self.num += self.idx;
        self.idx += 1;
        Some(self.num)
    }
}

/// Collatz sequence iterator which produces items containing the values rather
/// than references to the values. Positive numbers only!
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
