use crate::utils;

pub fn solve() {
    // The polynomial must produce at least 40 primes, so:
    // * `b` (the value of the polynomial at 0) must be prime.
    // * `b` must be odd (else, the polynomial produces at most 2 primes).
    // * `a` must be odd (else, the polynomial produces at most 3 primes).
    // * `a` must be at least `b` (else, the polynomial produces negatives).
    for b in utils::primes(999).skip(1).map(|b| b as i64) {
        for a in (-b..=999).step_by(2) {
            let primes = (0i64..)
                .map(|n| n.pow(2) + a * n + b)
                .take_while(|num| utils::is_prime(*num))
                .count();
            println!("{} {} {} {}", primes, a, b, a * b);
        }
    }
}
