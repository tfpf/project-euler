pub fn solve() -> i64 {
    let mut fraction = (3, 7);
    let result = (2..=1000000)
        .filter_map(|_| {
            // The difference between any two consecutive fractions is 1 over
            // the product of their denominators. Hence, produce a fraction
            // equal in value to the target, and find a smaller fraction with
            // the same denominator. Then try to divide throughout by 7.
            fraction = (fraction.0 + 3, fraction.1 + 7);
            let (numerator, denominator) = (fraction.0 - 1, fraction.1);
            if numerator % 7 == 0 && denominator % 7 == 0 {
                Some(numerator / 7)
            } else {
                None
            }
        })
        .last()
        .unwrap();

    assert_eq!(result, 428570);
    result
}
