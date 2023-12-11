use crate::utils;

pub fn solve() -> i64 {
    let mut primes = 0;
    for side in (3i64..).step_by(2) {
        for num in [side.pow(2), side.pow(2) - side + 1, side.pow(2) - 2 * side + 2, side.pow(2) - 3 * side + 3] {
            if utils::is_prime(num) {
                primes += 1;
            }
        }
        if primes * 10 < 2 * side - 1 {
            println!("{}", side);
            break;
        }
    }
    0
}

// n: 1, 3, 5, 7
// 1, 9, 25, 49 -> n^2
// 1, 7, 21, 43 -> (n - 1)^2 + n = n^2 - n + 1
// 1, 5, 17, 37 -> (n - 1)^2 + 1 = n^2 - 2n + 2
// 1, 3, 13, 31 -> (n - 1)^2 + 2 - n = n^2 - 3n + 3
