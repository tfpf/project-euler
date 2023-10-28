use crate::utils;

pub fn solve() {
    // Estimate the largest number using an approximation of the prime-counting
    // function. Then use the sieve of Eratosthenes.
    const LIMIT: usize = 120000;
    let prime = utils::sieve_of_eratosthenes(LIMIT);
    let result = prime
        .iter()
        .enumerate()
        .filter(|(_, c)| **c)
        .nth(10000)
        .unwrap()
        .0;

    println!("{}", result);
    assert_eq!(result, 104743);
}
