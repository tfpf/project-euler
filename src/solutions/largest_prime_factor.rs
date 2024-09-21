use crate::utils;

/// Determine the largest prime factor of the given number.
///
/// * `num`
fn largest_prime_factor(mut num: i64) -> i64 {
    // Not divisible by 2, 3 or 5. Look for other factors.
    utils::PotentialPrimes::new(num)
        .map_while(|potential_prime| {
            if potential_prime > num {
                None
            } else if num % potential_prime != 0 {
                // Dummy result so that the loop continues looking for factors and
                // does not terminate.
                Some(0)
            } else {
                // Found a potential prime factor. Eliminate it. Doing this ensures
                // that all factors found are actually prime factors.
                while num % potential_prime == 0 {
                    num /= potential_prime;
                }
                Some(potential_prime)
            }
        })
        .max()
        .unwrap()
}

pub fn solve() -> i64 {
    let num: i64 = 600851475143;
    let largest_pf = largest_prime_factor(num);

    assert_eq!(largest_pf, 6857);
    largest_pf
}
