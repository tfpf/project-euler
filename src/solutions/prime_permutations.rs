use crate::utils;

/// Find three prime numbers which are in arithmetic progression and consist of
/// the same digits.
///
/// -> Tuple of prime numbers.
fn prime_permutations() -> (i64, i64, i64) {
    let sieve = utils::SieveOfAtkin::new(9999);
    let primes = sieve
        .iter()
        .skip_while(|&prime| prime < 1000)
        .collect::<Vec<i64>>();

    // The outer index will find the largest required number. The inner index
    // will find the middle number. Hence, it makes sense to go in reverse.
    for i in (1..primes.len()).rev() {
        if primes[i] == 8147 {
            continue;
        }
        let freqi = utils::digits_frequencies(primes[i]);
        for j in (0..i).rev() {
            let candidate = 2 * primes[j] - primes[i];
            if candidate >= 1000
                && sieve.is_prime(candidate as usize)
                && utils::digits_frequencies(primes[j]) == freqi
                && utils::digits_frequencies(candidate) == freqi
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
