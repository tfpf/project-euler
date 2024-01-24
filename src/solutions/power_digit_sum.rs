use crate::utils;

pub fn solve() -> i64 {
    let result = utils::Long::from(2).pow(1000).sum();

    assert_eq!(result, 1366);
    result
}
