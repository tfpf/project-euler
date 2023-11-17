use crate::utils;

pub fn solve() {
    // Since we need only even terms, start from 2 and take every third term.
    let fibonacci = utils::Fibonacci::new(2, 3);
    let sum: i64 = fibonacci.step_by(3).take_while(|num| *num < 4000000).sum();

    print!("{}", sum);
    assert_eq!(sum, 4613732);
}
