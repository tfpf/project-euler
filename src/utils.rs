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
    for candidate in (5i64..).step_by(6).take_while(|f| f * f <= num) {
        if num % candidate == 0 || num % (candidate + 2) == 0 {
            return false;
        }
    }
    true
}

/// Check whether a number is a palindrome or not.
///
/// * `num` - Number to check.
///
/// -> Whether `num` written using decimal digits is a palindrome.
pub fn is_palindrome<T>(num: T) -> bool where T: std::fmt::Display
{
    let num = num.to_string();
    num == num.chars().rev().collect::<String>()
}
