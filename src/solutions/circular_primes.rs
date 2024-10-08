use crate::utils;

/// Check whether the given number is a circular prime number.
///
/// * `num`
/// * `sieve` Sieve of Atkin.
fn is_circular_prime(mut num: i64, sieve: &utils::SieveOfAtkin) -> bool {
    // Since the number is prime, only its rotations have to be checked.
    let passes = utils::Digits::new(num).count() - 1;
    let multiplier = 10i64.pow(passes as u32);
    for _ in 0..passes {
        num = num / 10 + num % 10 * multiplier;
        if !sieve.is_prime(num as usize) {
            return false;
        }
    }
    true
}

pub fn solve() -> i64 {
    let sieve = utils::SieveOfAtkin::new(1000000);
    let result = sieve.iter().filter(|&num| is_circular_prime(num, &sieve)).count();

    assert_eq!(result, 55);
    result as i64
}
