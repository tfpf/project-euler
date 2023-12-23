use crate::utils;

/// Determine the convergent at the specified index of the continued fraction
/// representation of Euler's number. Index
///
/// * `idx` - Current index. Callers must always set this to 2.
/// * `idx_max` - Index of the convergent. Must be at least 2.
fn gen(idx: i32, idx_max: i32) -> utils::Fraction {
    let addend = if idx % 3 == 0 { idx / 3 * 2 } else { 1 };
    let mut fraction = if idx == idx_max {
        utils::Fraction::from(0, 1)
    } else {
        gen(idx + 1, idx_max)
    };
    fraction += addend;
    fraction.invert();
    fraction
}

pub fn solve() -> i64 {
    let f = &gen(2, 100) + 2;
    println!("{}", f);

    0
}
