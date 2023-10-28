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
