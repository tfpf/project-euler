use crate::utils;

pub fn solve() -> i64 {
    let result = utils::Polygonal::new(6)
        .skip(143)
        .filter(|&num| {
            // Every triangular number is a hexagonal number as well, so it is
            // enough to check whether the number is pentagonal.
            utils::Polygonal::invert(5, num) != None
        })
        .next()
        .unwrap();

    assert_eq!(result, 1533776805);
    result
}
