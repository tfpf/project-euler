use crate::utils;

pub fn solve() -> i64 {
    println!("{:?}", utils::ContinuedFraction::new(7768).collect::<Vec<i64>>());
    0
}
