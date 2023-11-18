use crate::utils;

pub fn solve() -> i64 {
    let triangular = utils::Triangular::new();
    let result = triangular
        .filter(|num| utils::Divisors::new(*num).count() >= 500)
        .next()
        .unwrap();

    assert_eq!(result, 76576500);
    result
}
