use crate::utils;

pub fn solve() {
    // Make 40 movements, out of which 20 are rightwards and 20 are downwards.
    // The total number of possibilities is the binomial coefficient (40, 20).
    let result = (21..=40)
        .zip(1..=20)
        .fold((1i64, 1i64), |(numerator, denominator), (n, d)| {
            let numerator = numerator * n;
            let denominator = denominator * d;
            let g = utils::gcd(numerator, denominator);
            (numerator / g, denominator / g)
        })
        .0;

    print!("{}", result);
    assert_eq!(result, 137846528820);
}
