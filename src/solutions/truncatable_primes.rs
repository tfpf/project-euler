use crate::utils;

/// Check whether the number is a left-truncatable prime number.
///
/// * `digits` - Digits of the number (in big-endian order) to check.
///
/// -> Whether the number is a left-truncatable prime.
fn left_truncatable(digits: &[i32]) -> bool {
    // Instead of chopping off digits from the left, prepend digits to the
    // left. This avoids division.
    let mut multiplier = 1;
    let mut value = 0;
    for digit in digits.iter().rev() {
        value += digit * multiplier;
        multiplier *= 10;
        if !utils::is_prime(value as i64) {
            return false;
        }
    }
    true
}

/// Build truncatable prime numbers.
///
/// * `digits` - Digits of a partially-built number in big-endian order.
/// * `value` - The partially-built number represented by `digits`.
/// * `idx` - Index of the next digit to build.
///
/// -> Sum of all truncatable primes numbers having `value` as a prefix.
fn truncatable_sum(digits: &mut Vec<i32>, value: i32, idx: usize) -> i32 {
    let available_digits = [1, 2, 3, 5, 7, 9];

    // Build a right-truncatable prime number by appending digits to the right
    // and ensuring that the result is prime after each such operation. We'll
    // check whether it is left-truncatable separately. If it is true there are
    // only 11 truncatable prime numbers, the recursion is guaranteed to
    // terminate.
    digits.push(0);
    let last = digits.len() - 1;
    let sum = available_digits
        .map(|digit| {
            // The most significant digit must be 2, 3, 5 or 7. No other digit can
            // be 2 or 5.
            if idx == 0 && (digit == 1 || digit == 9) || idx != 0 && (digit == 2 || digit == 5) {
                return 0;
            }

            digits[last] = digit;
            let value = value * 10 + digit;
            if !utils::is_prime(value as i64) {
                return 0;
            }

            // The least significant digit must be 3 or 7. Hence, if we find either
            // of those, check whether we have found a truncatable prime and then
            // continue the search for more truncatable primes.
            if (digit == 3 || digit == 7) && left_truncatable(digits) {
                // Single-digit numbers do not count.
                if idx == 0 {
                    return truncatable_sum(digits, value, idx + 1);
                } else {
                    return value + truncatable_sum(digits, value, idx + 1);
                }
            }

            // Any other digit must be 1, 3, 7 or 9. No need to check, because
            // there are no other choices.
            truncatable_sum(digits, value, idx + 1)
        })
        .iter()
        .sum();
    digits.pop();
    sum
}

pub fn solve() -> i64 {
    let mut digits = vec![];
    let sum = truncatable_sum(&mut digits, 0, 0);

    assert_eq!(sum, 748317);
    sum as i64
}
