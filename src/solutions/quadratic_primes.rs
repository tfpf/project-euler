use crate::utils;

pub fn solve() -> i64 {
    // The polynomial must produce at least 40 primes, so:
    // * `b` (the value of the polynomial at 0) must be prime.
    // * `b` must be odd (else, the polynomial produces at most 2 primes).
    // * `a` must be odd (else, the polynomial produces at most 3 primes).
    // * `a` must be at least `b` (else, the polynomial produces negatives).
    let mut max_primes = 0;
    let mut a_times_b = 0;
    for b in utils::Primes::new(999).iter().skip(1) {
        for a in (-b..=999).step_by(2) {
            let primes = (0i64..)
                .map(|n| n.pow(2) + a * n + b)
                .take_while(|num| utils::is_prime(*num))
                .count();
            if primes > max_primes {
                (a_times_b, max_primes) = (a * b, primes);
            }
        }
    }

    assert_eq!(a_times_b, -59231);
    a_times_b
}
