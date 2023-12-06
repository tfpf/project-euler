use crate::utils;

/// Find the frequencies of all digits in the given number.
///
/// * `num` - Number to analyse.
///
/// -> Map between digits and frequencies.
fn digits_frequencies(num: i64) -> [u8; 10] {
    let mut freq = [0; 10];
    for digit in utils::Digits::new(num) {
        freq[digit as usize] += 1;
    }
    freq
}

/// Find three prime numbers which are in arithmetic progression and consist of
/// the same digits.
///
/// -> Tuple of prime numbers.
fn prime_permutations() -> (i64, i64, i64) {
    let sieve = utils::SieveOfEratosthenes::new(9999);
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
        let freqi = digits_frequencies(primes[i]);
        for j in (0..i).rev() {
            let candidate = 2 * primes[j] - primes[i];
            if candidate >= 1000
                && sieve.is_prime(candidate as usize)
                && digits_frequencies(primes[j]) == freqi
                && digits_frequencies(candidate) == freqi
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
