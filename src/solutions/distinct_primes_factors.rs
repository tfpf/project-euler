use crate::utils;

pub fn solve() -> i64 {
    let result = (644..)
        .filter(|&num| {
            for n in num..num + 4 {
                if utils::PrimeDivisors::new(n).count() < 4 {
                    return false;
                }
            }
            true
        })
        .next()
        .unwrap();

    assert_eq!(result, 134043);
    result
}
