use crate::utils;

pub fn solve() -> i64 {
    // The number of rectangles in a rectangular grid measuring `a` units by
    // `b` units is obtained by multiplying the sum of all positive integers
    // till `a` and the sum of all positive integers till `b`. Said product is
    // approximately `a.pow(2) * b.pow(2) / 4`, which be close to 2000000.
    // Hence, we can make an educated guess about the range to test `a` and `b`
    // for.
    let mut result = (0, 0, i64::MAX);
    for (a, sum_till_a) in (1..).zip(utils::Polygonal::new(3)).take(500) {
        for (b, sum_till_b) in (1..).zip(utils::Polygonal::new(3)).take(500) {
            let rectangles = sum_till_a * sum_till_b;
            let off = 2000000 - rectangles;
            let off_abs = off.abs();
            if off_abs < result.2 {
                result = (a, b, off_abs);
            } else {
                if off < 0 {
                    break;
                }
            }
        }
    }
    let result = result.0 * result.1;

    assert_eq!(result, 2772);
    result
}
