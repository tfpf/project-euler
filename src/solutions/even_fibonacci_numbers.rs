use crate::utils;

pub fn solve() {
    let fibonacci = utils::Fibonacci::new(1, 2);
    let sum: i64 = fibonacci
        .filter(|num| *num % 2 == 0)
        .take_while(|num| *num < 4000000)
        .sum();

    println!("{}", sum);
    assert_eq!(sum, 4613732);
}
