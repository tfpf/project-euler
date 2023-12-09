pub fn solve() -> i64 {
    // Determine all binomial coefficients by constructing Pascal's triangle.
    // As an optimisation, store only one row, overwriting it to generate the
    // next row.
    let mut triangle = [1; 101];
    let result = (1..=100)
        .map(|row| {
            (1..row).rev().fold(0, |above_1m, col| {
                triangle[col] += triangle[col - 1];
                if triangle[col] > 1000000 {
                    // Avoid integer overflow.
                    triangle[col] = 1000000;
                    above_1m + 1
                } else {
                    above_1m
                }
            })
        })
        .sum();

    assert_eq!(result, 4075);
    result
}
