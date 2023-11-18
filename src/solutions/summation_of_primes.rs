use crate::utils;

pub fn solve() {
    const LIMIT: usize = 2000000;
    let result: i64 = utils::primes(LIMIT).sum();

    println!("{}", result);
    assert_eq!(result, 142913828922);
}
