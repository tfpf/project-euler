use crate::utils;

pub fn solve() -> i64 {
    const LIMIT: usize = 10000;

    // This indicates which numbers can be constructed in the described manner.
    // Since only odd numbers are of interest, half of the space used is
    // wasted. Doing extra processing to prevent wastage increases the running
    // time by more than 100 µs.
    let mut goldbach_constructible = vec![false; LIMIT];

    let sieve = utils::SieveOfAtkin::new(LIMIT);
    for prime in sieve.iter() {
        // Squares are quadrilateral numbers. Generating them like this avoids
        // having to do multiplication.
        for square in utils::Polygonal::new(4) {
            let num = (prime + 2 * square) as usize;
            if num < goldbach_constructible.len() {
                goldbach_constructible[num] = true;
            } else {
                break;
            }
        }
    }
    let result = (3..goldbach_constructible.len())
        .step_by(2)
        .find(|&idx| !goldbach_constructible[idx] && !sieve.is_prime(idx))
        .unwrap();

    assert_eq!(result, 5777);
    result as i64
}
