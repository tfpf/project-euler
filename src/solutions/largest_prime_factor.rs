use crate::utils;

pub fn solve() -> i64 {
    let num: i64 = 600851475143;
    let largest_pf = utils::PrimeDivisors::new(num).max().unwrap().0;

    assert_eq!(largest_pf, 6857);
    largest_pf
}
