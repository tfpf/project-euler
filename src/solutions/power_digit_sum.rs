use crate::utils;

pub fn solve() {
    let product = (0..1000).fold(utils::Long::new("1"), |product, _| &product * 2);
    let result: u32 = product
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .sum();

    println!("{}", result);
    assert_eq!(result, 1366);
}
