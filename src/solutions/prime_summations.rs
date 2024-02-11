use crate::utils;

pub fn solve() -> i64 {
    // Approach is similar to that used for P31. The difference is that the
    // maximum denomination is guessed, and that there is no target sum.
    const LIMIT: usize = 100;
    let primes = utils::SieveOfAtkin::new(LIMIT).iter().collect::<Vec<i64>>();
    let (rows, cols) = (primes.len(), LIMIT + 1);
    let mut ways = vec![
        std::iter::once(1)
            .chain(std::iter::repeat(0).take(cols - 1))
            .collect::<Vec<i64>>();
        rows
    ];

    // Bottom-up dynamic programming.
    for sum in 1..cols {
        ways[0][sum] = if sum % primes[0] as usize == 0 { 1 } else { 0 };
    }
    for idx in 1..rows {
        for sum in 1..cols {
            ways[idx][sum] = ways[idx - 1][sum]
                + if sum >= primes[idx] as usize {
                    ways[idx][sum - primes[idx] as usize]
                } else {
                    0
                };
        }
    }
    let result = ways[rows - 1]
        .iter()
        .enumerate()
        .find(|(_, &count)| count >= 5000)
        .unwrap()
        .0;

    result as i64
}
