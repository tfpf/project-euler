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
pub fn count_divisors(num: i32) -> i32 {
    (1i32..)
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

/******************************************************************************
 * Objects.
 *****************************************************************************/
pub struct Long {
    digits: Vec<i32>,
}
impl Long {
    pub fn new(s: &str) -> Self {
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
    fn adc(a: i32, b: i32, carry: i32) -> (i32, i32) {
        let sum = a + b + carry;
        if sum >= 1_000_000_000 {
            (sum - 1_000_000_000, 1)
        } else {
            (sum, 0)
        }
    }
}
impl std::ops::AddAssign<&Self> for Long {
    fn add_assign(&mut self, other: &Long) {
        let mut carry = 0;
        for (sd, od) in self.digits.iter_mut().zip(other.digits.iter()) {
            (*sd, carry) = Self::adc(*sd, *od, carry);
        }
        for od in other.digits.iter().skip(self.digits.len()) {
            let sum_carry = Self::adc(0, *od, carry);
            self.digits.push(sum_carry.0);
            carry = sum_carry.1;
        }
        if carry > 0 {
            self.digits.push(carry);
        }
    }
}
impl std::fmt::Display for Long {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.digits.iter().rev().fold(Ok(()), |result, digit| {
            result.and_then(|_| write!(f, "{:0>9}", digit))
        })
    }
}

/******************************************************************************
 * Iterators.
 *****************************************************************************/

pub struct Fibonacci {
    a: i64,
    b: i64,
}
impl Fibonacci {
    pub fn new(a: i64, b: i64) -> Self {
        Fibonacci { a: a, b: b }
    }
}
impl Iterator for Fibonacci {
    type Item = i64;
    fn next(&mut self) -> Option<i64> {
        let a = self.a;
        self.a = self.b;
        self.b += a;
        Some(a)
    }
}
