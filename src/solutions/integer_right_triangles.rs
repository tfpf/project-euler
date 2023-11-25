use crate::utils;

pub fn solve() -> i64 {
    // The perimeter of a right-angled triangle with integral side lengths will
    // always be even.
    let result = (12..=1000)
        .step_by(2)
        .map(|p| (utils::PythagoreanTriplets::new(p).count(), p))
        .max()
        .unwrap()
        .1;

    assert_eq!(result, 840);
    result as i64
}
