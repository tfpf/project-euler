use crate::utils;

pub fn solve() -> i64 {
    let num: i64 = 600851475143;
    let largest_pf = (3i64..)
        .step_by(2)
        .take_while(|f| f * f <= num)
        // FIXME This doesn't seem correct. What if `f` is composite, but
        // `other` is prime?
        .filter_map(|f| {
            if num % f != 0 || !utils::is_prime(f) {
                return None;
            }
            let other = num / f;
            if utils::is_prime(other) {
                Some(other)
            } else {
                Some(f)
            }
        })
        .max()
        .unwrap();

    assert_eq!(largest_pf, 6857);
    largest_pf
}
