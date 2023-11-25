use crate::utils;

pub fn solve() -> i64 {
    let triplet = utils::PythagoreanTriplets::new(1000).next().unwrap();
    let result = triplet.0 * triplet.1 * triplet.2;

    assert_eq!(result, 31875000);
    result
}
