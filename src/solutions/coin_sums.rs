pub fn solve() -> i64 {
    let denominations = [1, 2, 5, 10, 20, 50, 100, 200];

    // Imagine a matrix in which the element at row index `row` and column
    // index `col` is the number of ways to obtain a sum equal to `col` using
    // some/all of the coins up to `denominations[row]`. We need a sum of 200.
    // What should be the size of this matrix?
    let (rows, cols) = (denominations.len(), 201);

    // We don't have to store the whole matrix. At any point in time, we need
    // only two consecutive rows. The current row can be constructed from the
    // previous row. Since the number of ways to obtain a sum of 0 is always 1,
    // the element at index 0 is 1 for any row. For the first row, the
    // remaining elements are also 1 (which is the number of ways to obtain any
    // sum using the first coin).
    let mut curr = vec![1; cols];

    // Initialise the previous row with dummy values. The first element must be
    // 1, as mentioned above.
    let mut prev = vec![1; cols];

    // Bottom-up dynamic programming.
    for idx in 1..rows {
        (prev, curr) = (curr, prev);
        for sum in 1..cols {
            curr[sum] = prev[sum]
                + if sum >= denominations[idx] {
                    curr[sum - denominations[idx]]
                } else {
                    0
                };
        }
        // At this point, `curr` is the `idx`th row of the matrix, and `prev`
        // is the previous.
    }
    let result = curr[cols - 1];

    assert_eq!(result, 73682);
    result as i64
}
