use crate::utils;

pub fn solve() -> i64 {
    // Approach is similar to that used for P31. The difference is that the
    // denominations are prime numbers (how many of them to use is guessed),
    // and that there is no target sum.
    const LIMIT: usize = 100;
    let primes = utils::SieveOfAtkin::new(LIMIT).iter().collect::<Vec<i64>>();
    let (rows, cols) = (primes.len(), LIMIT + 1);

    // This will no longer be all ones, because the number of ways to obtain a
    // particular sum using only the first prime number depends on the parity
    // of the sum!
    let mut curr = (0..cols)
        .map(|sum| if sum % primes[0] as usize == 0 { 1 } else { 0 })
        .collect();
    let mut prev = vec![1; cols];

    // Bottom-up dynamic programming.
    for idx in 1..rows {
        (prev, curr) = (curr, prev);
        for sum in 1..cols {
            curr[sum] = prev[sum]
                + if sum >= primes[idx] as usize {
                    curr[sum - primes[idx] as usize]
                } else {
                    0
                };
        }
    }
    let result = curr.iter().enumerate().find(|(_, &count)| count >= 5000).unwrap().0;

    assert_eq!(result, 71);
    result as i64
}
