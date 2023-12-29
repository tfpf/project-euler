use crate::utils;

pub fn solve() -> i64 {
    // The number of divisors of the product of two coprime numbers is the
    // product of their number of divisors. Every triangular number is the
    // product of two coprime numbers. Just iterate over the indices in order
    // to have those two coprime numbers at hand.
    let mut multiplicand1_divisors = 1;
    let idx = (1..)
        .find(|&idx| {
            let multiplicand2 = if idx & 1 == 1 { (idx + 1) / 2 } else { idx + 1 };
            let multiplicand2_divisors = utils::Divisors::new(multiplicand2).count();
            let divisors = multiplicand1_divisors * multiplicand2_divisors;
            multiplicand1_divisors = multiplicand2_divisors;
            divisors >= 500
        })
        .unwrap();
    let result = idx * (idx + 1) / 2;

    assert_eq!(result, 76576500);
    result
}
