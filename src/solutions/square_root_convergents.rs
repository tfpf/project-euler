use crate::utils;

pub fn solve() -> i64 {
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
