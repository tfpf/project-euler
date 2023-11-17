use crate::utils;

pub fn solve() {
    let sum: i64 = (1..1000000)
        .filter(|&num| utils::is_palindrome(num, 2) && utils::is_palindrome(num, 10))
        .sum();

    println!("{}", sum);
    assert_eq!(sum, 872187);
}
