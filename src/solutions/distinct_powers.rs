use crate::utils;

/// Find the greatest exponent a number can be raised to so that the resultant
/// value is equal to given number. For instance, given 32, this function
/// should return 5, and given 36, it should return 2.
///
/// * `num`
/// * `primes` - Prime numbers to use to calculate the exponent.
///
/// -> Exponent.
fn exponent(mut num: i64, primes: &Vec<i64>) -> i64 {
    // Calculate the greatest exponent of every prime in the given number.
    let mut exponents = vec![0i64; primes.len()];
    let mut gcd = 0;
    for (prime, exponent) in primes.iter().zip(exponents.iter_mut()) {
        while num % prime == 0 {
            num /= prime;
            *exponent += 1;
        }
        gcd = std::cmp::max(gcd, *exponent);
    }

    // Calculate the greatest common divisor of those.
    exponents
        .iter()
        .fold(gcd, |gcd, exponent| utils::gcd(gcd, *exponent))
}

pub fn solve() -> i64 {
    let primes = utils::SieveOfAtkin::new(100).iter().collect::<Vec<i64>>();
    let result: i64 = (2..=100)
        .map(|a| {
            let exp = exponent(a, &primes);
            if exp == 1 {
                // This is a first-power number. Can raise it to each exponent
                // from 2 to 100. Count all of them.
                99
            } else {
                // This is a square or cube or something like that. Some of its
                // exponents have already been counted in the other branch.
                // Count the rest.
                (2..=100)
                    // If this number were to be raised to this power, what
                    // would be the power of its corresponding first-power
                    // number? If that power has already been counted, ignore
                    // it.
                    .filter_map(|b| {
                        let num = b * exp;
                        for e in 1..exp {
                            if num % e == 0 && num <= 100 * e {
                                return None;
                            }
                        }
                        Some(num)
                    })
                    .count() as i64
            }
        })
        .sum();

    assert_eq!(result, 9183);
    result
}
