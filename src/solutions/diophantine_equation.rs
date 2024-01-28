use crate::utils;

pub fn solve() -> i64 {
    println!("{:?}", utils::ContinuedFraction::new(5786459).collect::<Vec<i64>>());
    println!("{:?}", utils::ContinuedFraction::new(9).collect::<Vec<i64>>());
    0
}
