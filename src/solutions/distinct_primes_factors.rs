use crate::utils;

pub fn solve() -> i64 {
    let result = (644..)
        .filter(|&num| {
            for n in num..num + 4 {
                if std::collections::HashSet::<i64>::from_iter(
                    utils::PrimeDivisors::new(n).map(|(prime, _)| prime),
                )
                .len()
                    < 4
                {
                    return false;
                }
            }
            true
        })
        .next()
        .unwrap();

    assert_eq!(result, 134043)
    result
}
