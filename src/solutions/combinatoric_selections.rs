pub fn solve() -> i64 {
    let mut result = 0;

    // Determine all binomial coefficients by constructing Pascal's triangle.
    // As an optimisation, store only one row, overwriting it to generate the
    // next row.
    let mut triangle = [0; 101];
    triangle[0] = 1;
    for row in 1..=100 {
        triangle[0] = 1;
        triangle[row] = 1;
        let above_1_mil = (1..row).rev().fold(0, |above_1_mil, col| {
            triangle[col] += triangle[col - 1];
            if triangle[col] > 1000000 {
                // Avoid integer overflow.
                triangle[col] = 1000000;
                above_1_mil + 1
            } else {
                above_1_mil
            }
        });
        result += above_1_mil;
    }

    assert_eq!(result, 4075);
    result
}
