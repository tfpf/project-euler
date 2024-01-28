use crate::utils;

pub fn solve() -> i64 {
    let result = (2..=10000)
        // The repeating part of the continued fraction is that which comes
        // after the first element of the returned vector. Hence, the period
        // is odd if the vector is of even length.
        .filter(|&num| utils::sqrt_continued_fraction(num).len() % 2 == 0)
        .count();

    assert_eq!(result, 1322);
    result as i64
}
