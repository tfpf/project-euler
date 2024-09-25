use crate::utils;

pub fn solve() -> i64 {
    let result = (2..=10000)
        // The repeating part of the continued fraction is that which comes
        // after the first element of the iterator. Hence, the period is odd if
        // the iterator has an even number of elements.
        .filter(|&num| utils::ContinuedFraction::new(num).count() % 2 == 0)
        .count();

    assert_eq!(result, 1322);
    result as i64
}
