use crate::utils;

pub fn solve() {
    let triangular = utils::Triangular::new();
    let result = triangular
        .filter(|num| utils::Divisors::new(*num).count() >= 500)
        .next()
        .unwrap();

    println!("{}", result);
    assert_eq!(result, 76576500);
}
