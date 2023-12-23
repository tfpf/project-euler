pub fn solve() -> i64 {
    // For any number which is not an integral power of 10, the number of
    // decimal digits it contains is equal to the ceiling of its 10-logarithm.
    // For a number expressible as a base to the power of some exponent to have
    // as many digits as the value of the exponent, the 10-logarithm of the
    // base must be less than 1. This is only possible if the base is less than
    // 10.
    let sum = (2..=9)
        .map(|base| {
            // When the exponent is 1, the number of decimal digits is
            // trivially equal to the exponent, so start from 2.
            (2..)
                .take_while(|&exp| {
                    // Once the number of digits falls short of the exponent,
                    // it will remain short for all following exponents,
                    // because the base is less than 10.
                    let digits = (exp as f64 * (base as f64).log10()).ceil() as i32;
                    digits == exp
                })
                .count()
        })
        .sum::<usize>()
        // We did not consider the base 1, because it is an integral power of
        // 10 (the formula mentioned above does not work on such). We also did
        // not consider the exponent 1. Count all of them.
        + 9;

    assert_eq!(sum, 49);
    sum as i64
}
