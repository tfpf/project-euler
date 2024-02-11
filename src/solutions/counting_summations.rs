pub fn solve() -> i64 {
    // Approach is similar to that used for P31. The difference is that the
    // maximum denomination (99) is not the same as the target sum (100).
    let denominations: Vec<usize> = (1..100).collect();
    let (rows, cols) = (denominations.len(), denominations.iter().max().unwrap() + 2);
    let mut ways = vec![
        std::iter::once(1)
            .chain(std::iter::repeat(0).take(cols - 1))
            .collect::<Vec<i32>>();
        rows
    ];

    // Bottom-up dynamic programming.
    for sum in 1..cols {
        ways[0][sum] = if sum % denominations[0] == 0 { 1 } else { 0 };
    }
    for idx in 1..rows {
        for sum in 1..cols {
            ways[idx][sum] = ways[idx - 1][sum]
                + if sum >= denominations[idx] {
                    ways[idx][sum - denominations[idx]]
                } else {
                    0
                };
        }
    }
    let result = ways[rows - 1][cols - 1];

    assert_eq!(result, 190569291);
    result as i64
}
