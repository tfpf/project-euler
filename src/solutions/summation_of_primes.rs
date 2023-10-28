use crate::utils;

pub fn solve() {
    const LIMIT: usize = 2000000;
    let prime = utils::sieve_of_eratosthenes(LIMIT);
    let result: usize = prime
        .iter()
        .enumerate()
        .filter(|(_, element)| **element)
        .map(|(index, _)| index)
        .sum();

    println!("{}", result);
    assert_eq!(result, 142913828922);
}
