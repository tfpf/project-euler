pub fn solve() -> i64 {
    // Determine all binomial coefficients by constructing Pascal's triangle.
    // As an optimisation, store only one row, overwriting it to generate the
    // next row.
    let mut triangle = [1; 101];
    let result = (1..=100)
        .map(|row| {
            (1..row)
                .rev()
                .filter(|&col| {
                    // Avoid integer overflow.
                    triangle[col] = std::cmp::min(1000000, triangle[col] + triangle[col - 1]);
                    triangle[col] == 1000000
                })
                .count()
        })
        .sum::<usize>();

    assert_eq!(result, 4075);
    result as i64
}
