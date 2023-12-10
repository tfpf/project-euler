use crate::utils;

pub fn solve() -> i64 {
    let mut product = utils::Long::from(1);
    for num in 2..=100 {
        product *= num;
    }
    let result = product.sum();

    assert_eq!(result, 648);
    result as i64
}
