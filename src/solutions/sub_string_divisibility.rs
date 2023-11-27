use crate::utils;

pub fn solve() -> i64 {
    let mut digits = [1, 0, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut sum = 0;

    // Can afford to skip the first permutation, because we know it does not
    // have the required property.
    while utils::next_permutation(&mut digits) {
        // Get the easiest divisibility tests (those for 2, 3, 5 and 11) out of
        // the way first.
        if digits[3] & 1 != 0
            || (digits[2] + digits[3] + digits[4]) % 3 != 0
            || (digits[5] != 0 && digits[5] != 5)
            || (digits[5] - digits[6] + digits[7]) % 11 != 0
        {
            continue;
        }
        if digits[4..=6].iter().fold(0, |value, d| value * 10 + d) % 7 != 0 {
            continue;
        }
        if digits[6..=8].iter().fold(0, |value, d| value * 10 + d) % 13 != 0 {
            continue;
        }
        if digits[7..=9].iter().fold(0, |value, d| value * 10 + d) % 17 != 0 {
            continue;
        }
        sum += digits.iter().fold(0, |value, d| value * 10 + d);
    }

    assert_eq!(sum, 16695334890);
    sum
}