use crate::utils;

pub fn solve() -> i64 {
    let result = utils::Polygonal::new(6)
        .skip(143)
        .filter(|&num| {
            utils::Polygonal::invert(3, num) != None && utils::Polygonal::invert(5, num) != None
        })
        .next().unwrap();

    result
    assert_eq!(result, 1533776805);
}
