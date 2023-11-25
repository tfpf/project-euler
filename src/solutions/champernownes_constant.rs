/// Determine the digit at the given index in Champernowne's Constant.
///
/// * `idx` - 1-based index.
///
/// -> Digit at said index.
fn digit_at(idx: u32) -> u32 {
    // Indices 1 to 9 are made of 1-digit numbers; indices 10 to 189, 2-digit
    // numbers; indices 190 to 2889, 3-digit numbers; etc.
    let (width, idx) = match idx {
        1..=9 => (1, idx - 1),
        10..=189 => (2, idx - 10),
        190..=2889 => (3, idx - 190),
        2890..=38889 => (4, idx - 2890),
        38890..=488889 => (5, idx - 38890),
        488890..=5888889 => (6, idx - 488890),
        _ => unreachable!(),
    };

    // Find the `idx`th digit of the string formed by concatening all
    // `width`-digit numbers. Note that `idx` is now a 0-based index.
    let num = 10u32.pow(width - 1) + idx / width;
    let idx_in_num_from_right = width - 1 - idx % width;
    num / 10u32.pow(idx_in_num_from_right) % 10
}

pub fn solve() -> i64 {
    let result: u32 = (1..=6).map(|exp| digit_at(10u32.pow(exp))).product();

    assert_eq!(result, 210);
    result as i64
}
