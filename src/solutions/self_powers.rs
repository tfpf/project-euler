use crate::utils;

pub fn solve() -> i64 {
    let modulus = 10i64.pow(10);
    let sum = (1..1000).filter(|num| num % 10 != 0).fold(0, |sum, num| {
        (sum + utils::pow(num, num as u32, modulus)) % modulus
    });

    assert_eq!(sum, 9110846700);
    sum
}
