/// Determine the digit at the given index in Champernowne's Constant.
///
/// * `idx` 1-based index.
///
/// -> Digit at said index.
fn digit_at(idx: u32) -> u32 {
    // Obtain a 0-based index.
    let mut idx = idx - 1;

    // Indices 0 to 8 are made of 1-digit numbers.
    let mut idx_ub = 9;
    let mut width = 1;
    while idx >= idx_ub {
        idx -= idx_ub;
        width += 1;

        // How many total digits are present in all `width`-digit numbers? That
        // tells us the maximum index permitted.
        let numbers_with_width_digits = 9 * 10u32.pow(width - 1);
        let digits_count = numbers_with_width_digits * width;
        idx_ub = digits_count;
    }

    // Find the `idx`th digit of the string formed by concatening all
    // `width`-digit numbers.
    let num = 10u32.pow(width - 1) + idx / width;
    let idx_in_num_from_right = width - 1 - idx % width;
    num / 10u32.pow(idx_in_num_from_right) % 10
}

pub fn solve() -> i64 {
    let result: u32 = (0..=6).map(|exp| digit_at(10u32.pow(exp))).product();

    assert_eq!(result, 210);
    result as i64
}
