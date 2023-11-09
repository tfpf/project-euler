use crate::utils;

pub fn solve() {
    let mut result = (1, 1);
    for denominator in 11..=99 {
        for numerator in 10..denominator {
            let num = utils::Digits::new(numerator).collect::<Vec<i64>>();
            let den = utils::Digits::new(denominator).collect::<Vec<i64>>();
            // The unit's digit must not be 0.
            if num[0] != 0
                // At least one digit must be common.
                && (num[0] == den[0] || num[0] == den[1] || num[1] == den[0] || num[1] == den[1])
                // The incorrectly-obtained value must be same as the correct
                // value.
                && num[0] * num[1] * denominator == den[0] * den[1] * numerator
            {
                result = (result.0 * numerator, result.1 * denominator);
            }
        }
    }
    let gcd = utils::gcd(result.0, result.1);
    let denominator = result.1 / gcd;

    println!("{}", denominator);
    assert_eq!(denominator, 100);
}
