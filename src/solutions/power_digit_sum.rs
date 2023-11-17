use crate::utils;

pub fn solve() {
    let mut product = utils::Long::new("1");
    for _ in 0..1000 {
        product *= 2;
    }
    let result: u32 = product
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .sum();

    print!("{}", result);
    assert_eq!(result, 1366);
}
