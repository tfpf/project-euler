/// Main function.
fn main()
{
    // I could apply the formulae to calculate the sums, but then I wouldn't
    // be learning Rust.
    let square_of_sum = (1..101).sum::<i32>().pow(2);
    let sum_of_squares = (1..101).fold(0, |s, elem| s + elem * elem);
    let difference = square_of_sum - sum_of_squares;
    println!("{difference}");

    assert_eq!(difference, 25164150);
}
