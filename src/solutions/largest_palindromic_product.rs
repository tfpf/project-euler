use crate::utils;

pub fn solve() {
    let mut result = -1;
    for i in (100..1000).rev() {
        for j in (100..1000).rev() {
            let product = i * j;
            if product > result && utils::is_palindrome(product) {
                result = product;
            }
        }
    }

    println!("{}", result);
    assert_eq!(result, 906609);
}
