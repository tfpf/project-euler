use crate::utils;

/// Find the continued fraction convergent of Euler's number at the specified
/// index.
///
/// * `idx_max` Index of the convergent.
///
/// -> Value of the convergent; 0 if the index is non-positive.
fn generate_continued_fraction(idx_max: u32) -> utils::Fraction {
    match idx_max {
        ..=0 => utils::Fraction::from(0, 1),
        1 => utils::Fraction::from(2, 1),
        _ => &generate_continued_fraction_(2, idx_max) + 2,
    }
}

/// Helper to calculate the continued fraction convergent of Euler's number.
///
/// * `idx` Current index.
/// * `idx_max` Index of the convergent.
///
/// -> Value of the convergent.
fn generate_continued_fraction_(idx: u32, idx_max: u32) -> utils::Fraction {
    let addend = if idx % 3 == 0 { idx / 3 * 2 } else { 1 };
    let mut fraction = if idx == idx_max {
        utils::Fraction::from(0, 1)
    } else {
        generate_continued_fraction_(idx + 1, idx_max)
    };
    fraction += addend;
    fraction.invert();
    fraction
}

pub fn solve() -> i64 {
    let sum = generate_continued_fraction(100).sum().0;

    assert_eq!(sum, 272);
    sum
}
