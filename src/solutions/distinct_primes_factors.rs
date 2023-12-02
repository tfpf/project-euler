use crate::utils;

fn four_distinct() -> i64 {
    let primes = utils::Primes::new(1000).iter().collect::<Vec<i64>>();
    let mut num = 644;
    let mut obtained = 0;
    let mut required = 4;
    loop {
        // Search backwards (similar to how it's done in the Boyer-Moore
        // substring searching algorithm) so that we can skip forwards,
        // avoiding unnecessary computations.
        for n in (num..num + 4).rev() {
            // Try to terminate the iterator as early as possible by taking
            // only 4 prime divisors.
            if primes
                .iter()
                .filter(|&&prime| n % prime == 0)
                .take(4)
                .count()
                == 4
            {
                required -= 1;
                if required == 0 {
                    return num;
                }
            } else {
                num += obtained + required;
                required += obtained;
                obtained = 4 - required;
                break;
            }
        }
    }
}

pub fn solve() -> i64 {
    let result = four_distinct();

    assert_eq!(result, 134043);
    result
}
