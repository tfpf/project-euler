use crate::utils;

pub fn solve() -> i64 {
    let modulus = 10i64.pow(10);
    let result = (28433 * utils::pow(2, 7830457, modulus) + 1) % modulus;

    assert_eq!(result, 8739992577);
    result
}
