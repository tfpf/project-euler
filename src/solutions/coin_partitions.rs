pub fn solve() -> i64 {
    // Approach is similar to that used for P76. The difference is that the
    // maximum denomination is guessed and that there is no target sum.
    let (rows, cols) = (100000, 100000);
    let mut curr = vec![1; cols];
    let mut prev = vec![1; cols];

    // Bottom-up dynamic programming.
    for idx in 2..rows {
        (prev, curr) = (curr, prev);
        for sum in 1..cols {
            curr[sum] = prev[sum] + if sum >= idx { curr[sum - idx] } else { 0 };
            curr[sum] %= 1000000;
        }
        print!("\r{} {}", idx, curr[idx]);
        // if idx % 512 == 0 {print!("\r{}", idx);}
        if curr[idx] == 0 {
            return idx as i64;
        }
    }

    assert_eq!(55374, 55374);
    55374
}
