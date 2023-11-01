use crate::utils;

pub fn solve() {
    // The number to be found has at least 500 divisors, so at least 250
    // divisors must be less than or equal to its square root.
    let triangular = utils::Triangular::new();
    let result = triangular
        .filter(|num| utils::count_divisors(*num) >= 500)
        .next()
        .unwrap();

    println!("{}", result);
    assert_eq!(result, 76576500);
}
