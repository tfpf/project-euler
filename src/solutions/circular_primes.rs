use crate::utils;

/// Check whether the given number is a circular prime number.
///
/// * `num` - Prime number.
/// * `sieve` - Sieve of eratosthenes.
///
/// -> Whether the number is a circular prime.
fn is_circular_prime(mut num: i64, sieve: &Vec<bool>) -> bool {
    // Since the number is prime, only its rotations have to be checked.
    let passes = utils::Digits::new(num).count() - 1;
    for _ in 0..passes {
        num = num / 10 + num % 10 * 10i64.pow(passes as u32);
        if !sieve[num as usize] {
            return false;
        }
    }
    true
}

pub fn solve() {
    let sieve = utils::sieve_of_eratosthenes(1000000);
    let result = utils::primes(1000000)
        .filter(|&num| is_circular_prime(num, &sieve))
        .count();

    print!("{}", result);
    assert_eq!(result, 55);
}
