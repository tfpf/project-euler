use crate::utils;

pub fn solve() -> i64 {
    // Assume that first expansion is at index 0. The first 33 expansions which
    // have a numerator with more digits than the denominator are those at
    // indices 7, 12, 20, 25, 33, 38, 46, 54, 59, 67, 72, 80, 85, 88, 93, 101,
    // 106, 114, 119, 127, 135, 140, 148, 153, 161, 166, 174, 182, 187, 195,
    // 200, 208 and 216. The differences between consecutive indices are 5, 8,
    // 5, 8, 5, 8, 8, 5, 8, 5, 8, 5, 3, 5, 8, 5, 8, 5, 8, 8, 5, 8, 5, 8, 5, 8,
    // 8, 5, 8, 5, 8 and 8. This pattern of differences continues to hold till
    // 1000. It can be used to find the answer. However, I wouldn't have found
    // this pattern without the current implementation, so I'm leaving it be.
    let mut fraction = utils::Fraction::from(1, 2);
    let result = (0..1000)
        .filter(|_| {
            let value = &fraction + 1;
            fraction += 2;
            fraction.invert();
            let (numerator_len, denominator_len) = value.len();
            numerator_len > denominator_len
        })
        .count();

    assert_eq!(result, 153);
    result as i64
}
