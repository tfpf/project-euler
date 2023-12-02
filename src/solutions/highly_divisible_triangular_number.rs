use crate::utils;

pub fn solve() -> i64 {
    let result = utils::Polygonal::new(3)
        .filter(|&num| {
            utils::PrimeDivisors::new(num)
                .fold(1, |accumulator, (_, power)| accumulator * (power + 1))
                >= 500
        })
        .next()
        .unwrap();

    assert_eq!(result, 76576500);
    result
}
