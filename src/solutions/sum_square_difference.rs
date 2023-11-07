pub fn solve() {
    // I could apply the formulae to calculate the sums, but then I wouldn't
    // be learning Rust.
    let square_of_sum = (1..=100).sum::<i32>().pow(2);
    let sum_of_squares = (1i32..=100).map(|element| element.pow(2)).sum::<i32>();
    let difference = square_of_sum - sum_of_squares;

    println!("{}", difference);
    assert_eq!(difference, 25164150);
}
