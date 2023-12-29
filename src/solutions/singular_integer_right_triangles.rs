use crate::utils;

pub fn solve() -> i64 {
    let result = (12..=1500000)
        .step_by(2)
        .filter(|&perimeter| utils::PythagoreanTriplets::new(perimeter).count() == 1)
        .count();

    assert_eq!(result, 161667);
    result as i64
}
