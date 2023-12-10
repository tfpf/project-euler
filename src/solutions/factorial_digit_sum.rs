use crate::utils;

pub fn solve() -> i64 {
    let result = utils::Long::factorial(100).sum();

    assert_eq!(result, 648);
    result as i64
}
