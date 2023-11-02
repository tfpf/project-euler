use crate::utils;

pub fn solve() {
    let product = (2..=100).fold(utils::Long::new("1"), |product, num| &product * num);
    let result: u32 = product
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .sum();

    println!("{}", result);
    assert_eq!(result, 648);
}
