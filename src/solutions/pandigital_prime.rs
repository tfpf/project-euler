use crate::utils;

pub fn solve() -> i64 {
    // A pandigital prime number can have 4 or 7 digits. Any pandigital number
    // with a different number of digits is divisible by 3. We get lucky while
    // searching 7-digit numbers.
    let mut digits = [7, 6, 5, 4, 3, 2, 1];
    let result = loop {
        let value = digits.iter().fold(0, |value, digit| value * 10 + digit);
        if utils::is_prime(value) {
            break value;
        }
        utils::prev_permutation(&mut digits);
    };

    assert_eq!(result, 7652413);
    result
}
