use crate::utils;

/// Find the digits of a number in sorted order.
///
/// * `num` - Number to analyse.
///
/// -> Digits in sorted order.
fn digits(num: i64) -> Vec<i64> {
    let mut digits = utils::Digits::new(num).collect::<Vec<i64>>();
    digits.sort();
    digits
}

/// Find three prime numbers which are in arithmetic progression and consist of
/// the same digits.
///
/// -> Tuple of prime numbers.
fn prime_permutations() -> (i64, i64, i64) {
    let primes = utils::Primes::new(9999);
    let sieve = primes.sieve();
    let primes = primes
        .iter()
        .skip_while(|&prime| prime < 1000)
        .collect::<Vec<i64>>();

    // The outer index will find the largest required number. The inner index
    // will find the middle number. Hence, it makes sense to go in reverse.
    for i in (1..primes.len()).rev() {
        if primes[i] == 8147 {
            continue;
        }

        let di = digits(primes[i]);
        for j in (0..i).rev() {
            let candidate = 2 * primes[j] - primes[i];
            if candidate >= 1000
                && sieve[candidate as usize]
                && digits(primes[j]) == di
                && digits(candidate) == di
            {
                return (candidate, primes[j], primes[i]);
            }
        }
    }
    unreachable!();
}

pub fn solve() -> i64 {
    let result = prime_permutations();
    let result = result.0 * 100000000 + result.1 * 10000 + result.2;

    assert_eq!(result, 296962999629);
    result
}
