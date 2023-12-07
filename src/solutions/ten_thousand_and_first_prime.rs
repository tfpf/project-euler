use crate::utils;

pub fn solve() -> i64 {
    // Start with a generous estimate of the 10000th prime number using the
    // prime-counting function.
    const LIMIT: usize = 120000;
    let result = utils::SieveOfEratosthenes::new(LIMIT)
        .iter()
        .nth(10000)
        .unwrap();

    assert_eq!(result, 104743);
    result
}
