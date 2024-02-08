use crate::utils;

pub fn solve() -> i64 {
    let mut fraction = (3, 7);
    let result = (2..=1000000)
        .filter_map(|_| {
            fraction = (fraction.0 + 3, fraction.1 + 7);
            let (numerator, denominator) = (fraction.0 - 1, fraction.1);
            let g = utils::gcd(numerator, denominator);
            if denominator / g <= 1000000 {
                Some(numerator / g)
            } else {
                None
            }
        })
        .last()
        .unwrap();

    assert_eq!(result, 428570);
    result
}
