use crate::utils;

pub fn solve() {
    // Start with a generous estimate of the 10000th prime number using the
    // prime-counting function.
    const LIMIT: usize = 120000;
    let result = utils::primes(LIMIT).nth(10000).unwrap();

    print!("{}", result);
    assert_eq!(result, 104743);
}
