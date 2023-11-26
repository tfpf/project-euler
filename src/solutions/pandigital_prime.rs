use crate::utils;

pub fn solve() -> i64 {
    // A pandigital prime number can have 4 or 7 digits. Any pandigital number
    // with a different number of digits is divisible by 3. We get lucky while
    // searching 7-digit numbers.
    let mut pandigital_checker = utils::PandigitalChecker::new(7);
    let result = (1234567..=7654321)
        .rev()
        .step_by(2)
        .filter(|&num| {
            pandigital_checker.renew();
            pandigital_checker.update(num) && pandigital_checker.check() && utils::is_prime(num)
        })
        .next()
        .unwrap();

    assert_eq!(result, 7652413);
    result
}
