/// Determine the period of the continued fraction representation of the
/// square root of the given number.
///
/// * `value` - Number to analyse.
///
/// -> Continued fraction period; 0 if the square root is an integer.
fn sqrt_continued_fraction_period(value: i64) -> usize {
    let froot = (value as f64).sqrt();
    if froot.fract() == 0.0 {
        return 0;
    }

    // `numerator_integer` is the integer which gets subtracted from `froot` in
    // the numerator of an intermediate fraction. (We actually subtract it from
    // from `iroot` as an optimisation here.)
    let iroot = froot as i64;
    let (mut numerator_integer, mut denominator) = (iroot, 1);
    for period in 1.. {
        denominator = (value - numerator_integer.pow(2)) / denominator;
        let subtrahend = (iroot + numerator_integer) / denominator;
        numerator_integer = denominator * subtrahend - numerator_integer;
        if denominator == 1 {
            return period;
        }
    }
    unreachable!();
}

pub fn solve() -> i64 {
    let result = (2..=10000)
        .filter(|&value| sqrt_continued_fraction_period(value) % 2 == 1)
        .count();

    assert_eq!(result, 1322);
    result as i64
}
