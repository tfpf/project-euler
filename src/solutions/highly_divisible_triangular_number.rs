use crate::utils;

pub fn solve() -> i64 {
    // The number of divisors of the product of two coprime numbers is the
    // product of their number of divisors. Every triangular number is the
    // product of two coprime numbers. Just iterate over the indices in order
    // to have those two coprime numbers at hand.
    let idx = (1..)
        .filter(|&idx| {
            (if idx & 1 == 0 {
                utils::Divisors::new(idx / 2).count() * utils::Divisors::new(idx + 1).count()
            } else {
                utils::Divisors::new(idx).count() * utils::Divisors::new((idx + 1) / 2).count()
            }) >= 500
        })
        .next()
        .unwrap();
    let result = idx * (idx + 1) / 2;

    assert_eq!(result, 76576500);
    result
}
