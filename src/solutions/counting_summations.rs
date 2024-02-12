pub fn solve() -> i64 {
    // Approach is similar to that used for P31. The difference is that the
    // denominations are the numbers from 1 to 99 (so we don't need a separate
    // array to store them) and that the target sum is 100. The matrix is now
    // such that the element at row index `row` and column index `col` is the
    // number of ways to obtain a sum equal to `col` using some/all of the
    // numbers up to `row`. Obviously, we can skip `row == 0`.
    let (rows, cols) = (100, 101);

    // As noted above, we start from `row == 1`.
    let mut curr = vec![1; cols];
    let mut prev = vec![1; cols];

    // Bottom-up dynamic programming.
    for idx in 2..rows {
        (prev, curr) = (curr, prev);
        for sum in 1..cols {
            curr[sum] = prev[sum] + if sum >= idx { curr[sum - idx] } else { 0 };
        }
    }
    let result = curr[cols - 1];

    assert_eq!(result, 190569291);
    result as i64
}
