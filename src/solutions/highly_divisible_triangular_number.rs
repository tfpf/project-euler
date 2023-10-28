use crate::utils;

/// Find the first highly divisible triangular number.
///
/// -> Highly divisible triangular number.
fn get_hdtn() -> i32 {
    let mut num = 0;
    for i in 1.. {
        num += i;
        if utils::count_divisors(num) >= 500 {
            return num;
        }
    }
    0
}

pub fn solve() {
    let result = get_hdtn();

    println!("{}", result);
    assert_eq!(result, 76576500);
}
