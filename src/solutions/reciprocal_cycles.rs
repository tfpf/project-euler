use crate::utils;

/// Find the length of the repeating part of the decimal representation of the
/// reciprocal of a prime number.
///
/// * `prime` Prime number.
fn recurrence_length(prime: i64) -> i64 {
    // The digits (i.e. the sequence of quotients) will start repeating when
    // the remainder becomes 1 for the second time.
    let mut rem = 1;
    for length in 1.. {
        rem = rem * 10 % prime;
        if rem == 1 {
            return length;
        }
    }
    unreachable!();
}

pub fn solve() -> i64 {
    // It is enough to check only prime numbers. (The recurrence length for a
    // composite number is equal to the greatest recurrence length of its prime
    // factors.) Exclude 2 and 5, because they can divide any number without
    // decimal recurrence. Exclude 3 because it is definitely not the answer.
    let result = utils::SieveOfAtkin::new(1000)
        .iter()
        .skip_while(|prime| *prime <= 5)
        .map(|prime| (recurrence_length(prime), prime))
        .max()
        .unwrap()
        .1;

    assert_eq!(result, 983);
    result
}
