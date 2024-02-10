use crate::utils;
pub fn solve() -> i64 {
    utils::SieveOfAtkin::new(100000000).is_prime(99999989) as i64
}
