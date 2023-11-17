/// Find the digit which causes us to stay within 1_000_000 permutations.
///
/// * `digits` - Pool of available digits.
/// * `count` - Number of permutations we have already seen. Will get updated
///   to the number of permutations we have seen after the required digit is
///   found.
/// * `step` - Number of permutations we will see if we pick a digit from
///   `digits`.
///
/// -> The correct digit of the 1_000_000th permutation.
fn overshoot(digits: &Vec<i32>, count: &mut i32, step: i32) -> i32 {
    for (&prev, &_) in digits.iter().zip(digits.iter().skip(1)) {
        // If I set the digit `_`, how many permutations will I have seen
        // before any number containing `_` at this position?
        let count_ = *count + step;
        if count_ < 1_000_000 {
            *count = count_;
            continue;
        }

        // If I overshot the target permutation, then the target permutation
        // must have `prev` at this position.
        return prev;
    }
    -1
}

pub fn solve() {
    // The last number in this vector is a sentinel value. It won't actually be
    // used; it's there just to make the code simpler.
    let mut digits = (0..=10).collect::<Vec<i32>>();

    let mut result: Vec<i32> = vec![];
    let mut count = 0;
    let mut step = 3628800;
    for position in 0..=9 {
        step /= 10 - position;
        let digit = overshoot(&digits, &mut count, step);
        result.push(digit);
        digits.retain(|&elem| elem != digit);
    }
    let result = result.iter().fold(0, |s, &elem| s * 10 + elem as i64);

    print!("{}", result);
    assert_eq!(result, 2783915460);
}
