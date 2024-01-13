use crate::utils;

pub fn solve() -> i64 {
    const LIMIT: usize = 2000000;
    let result: i64 = utils::SieveOfAtkin::new(LIMIT).iter().sum();

    assert_eq!(result, 142913828922);
    result
}
