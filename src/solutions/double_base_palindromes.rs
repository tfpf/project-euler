use crate::utils;

pub fn solve() -> i64 {
    let sum: i64 = (1..1000000)
        .step_by(2)
        .filter(|&num| utils::is_palindrome(num, 2) && utils::is_palindrome(num, 10))
        .sum();

    assert_eq!(sum, 872187);
    sum
}
