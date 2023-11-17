pub fn solve() {
    // Ignore the central 1. The top right diagonal contains the squares of odd
    // numbers (9, 25, 49, 81, and so on), i.e. a quadratic function of the
    // sequence of odd numbers. The bottom left diagonal contains the squares
    // of even numbers offset by 1 (5, 17, 37, 65, and so on), but with some
    // algebra, those numbers can also be expressed as a quadratic function of
    // the sequence of odd numbers. The same can be done for the remaining two
    // diagonals; the effective quadratic function is as seen below.
    let sum = (3i32..)
        .step_by(2)
        .take(500)
        .map(|num| 4 * num.pow(2) - 6 * num + 6)
        .sum::<i32>()
        + 1;

    print!("{}", sum);
    assert_eq!(sum, 669171001);
}
