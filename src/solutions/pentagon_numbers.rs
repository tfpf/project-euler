/// Check whether the given number is a pentagonal number.
///
/// * `num` - Number to check.
///
/// -> Whether it is pentagonal.
fn is_pentagonal(num: i32) -> bool {
    // Calculate its index using the inverse of the pentagonal number formula.
    // Said index must be an integer.
    let idx = (1.0 + (1.0 + 24.0 * num as f64).sqrt()) / 6.0;
    (idx - idx as i32 as f64).abs() <= 1e-5
}

/// Find the smallest difference between two pentagonal numbers which is a
/// pentagonal number, while their sum is also a pentagonal number.
///
/// -> Smallest pentagonal difference.
fn minimum_pentagonal_difference() -> i32 {
    let pentagons = (1..3000)
        .map(|num| num * (3 * num - 1) / 2)
        .collect::<Vec<i32>>();
    for i in 1..pentagons.len() {
        for j in 0..i {
            let sum = pentagons[i] + pentagons[j];
            let difference = pentagons[i] - pentagons[j];
            if is_pentagonal(sum) && is_pentagonal(difference) {
                return difference;
            }
        }
    }
    unreachable!();
}

pub fn solve() -> i64 {
    let result = minimum_pentagonal_difference();

    assert_eq!(result, 5482660);
    result as i64
}
