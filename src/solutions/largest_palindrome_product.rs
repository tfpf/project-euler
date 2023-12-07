use crate::utils;

pub fn solve() -> i64 {
    let mut result = -1;
    for i in (100..1000).rev() {
        for j in (100..=i).rev() {
            let product = i * j;
            if product > result && utils::is_palindrome(product, 10) {
                result = product;
            }
        }
    }

    assert_eq!(result, 906609);
    result
}
