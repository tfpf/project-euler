use crate::utils;

/// Determine whether the given number has digits we've already seen earlier.
///
/// * `digits` - Indicator which specifies the digits already seen earlier.
/// * `num` - Number to check.
///
/// -> Whether any digit is found for the second time.
fn has_repetition(digits: &mut [bool; 10], num: i64) -> bool {
    for digit in utils::Digits::new(num) {
        let digit = digit as usize;
        if digits[digit] {
            return true;
        }
        digits[digit] = true;
    }
    false
}

pub fn solve() -> i64 {
    // We have to find a concatenated product at least as much as 918273645
    // (which is provided as an example). Hence, the most significant digit of
    // the integer to search for must be 9. Further, to end up with 9 digits,
    // said integer must have 4 non-repeated digits. This means only two
    // consecutive products need be considered.
    let num_to_find = (9123..=9876)
        .filter(|num| {
            let mut digits = [false; 10];
            if has_repetition(&mut digits, num * 1) || has_repetition(&mut digits, num * 2) {
                return false;
            }
            digits
                .iter()
                .skip(1)
                .fold(true, |pandigital, &elem| pandigital && elem)
        })
        .last()
        .unwrap();

    // We have found the integer. Now find the pandigital concatenated product
    // by sticking twice its value to the right of once its value.
    let result = num_to_find * 100002;

    assert_eq!(result, 932718654);
    result
}
