pub fn solve() -> i64 {
    let mut fraction = (3, 7);
    let result = (2..=1000000)
        .filter_map(|_| {
            // Produce a fraction equal in value to the target. Find a smaller
            // fraction with the same denominator.
            fraction = (fraction.0 + 3, fraction.1 + 7);
            let (numerator, denominator) = (fraction.0 - 1, fraction.1);
            // Their GCD can only be 7. Not the multiplier, because we
            // subtracted 1 from the numerator.
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
