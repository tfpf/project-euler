pub fn solve() -> i64 {
    let denominations = [1, 2, 5, 10, 20, 50, 100, 200];

    // Any element in this matrix shall be the number of ways to obtain a sum
    // equal to the column index using coins at indices less than or equal to
    // the row index. Note that there is only one way to obtain a sum of 0.
    // Hence, as an optimisation, set the first element of each row of the
    // matrix to 1 during initialisation.
    let (rows, cols) = (denominations.len(), denominations.iter().max().unwrap() + 1);
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

    assert_eq!(result, 73682);
    result as i64
}
