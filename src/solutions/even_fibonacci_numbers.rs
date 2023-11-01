use crate::utils;

pub fn solve() {
    // Since we need only even terms, start from 2 and take every third term.
    let fibonacci = utils::Fibonacci::new(utils::Long::new("2"), utils::Long::new("3"));
    let sum: utils::Long = fibonacci
        .step_by(3)
        .take_while(|num| num < &4000000)
        .sum();
    let sum = sum.get();

    println!("{}", sum);
    assert_eq!(sum, 4613732);
}
