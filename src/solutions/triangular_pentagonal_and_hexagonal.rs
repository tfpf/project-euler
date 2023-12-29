use crate::utils;

pub fn solve() -> i64 {
    // Every hexagonal number is a triangular number as well, so it is enough
    // to find a hexagonal number which is also pentagonal.
    let result = utils::Polygonal::new(6)
        .skip(143)
        .find(|&num| utils::Polygonal::invert(5, num).is_some())
        .unwrap();

    assert_eq!(result, 1533776805);
    result
}
