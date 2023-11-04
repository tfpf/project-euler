use crate::utils;

pub fn solve() {
    let num: i64 = 600851475143;
    let largest_pf = (3i64..)
        .step_by(2)
        .take_while(|f| f * f <= num)
        .filter(|f| num % f == 0 && utils::is_prime(*f))
        .map(|f| {
            let other = num / f;
            if utils::is_prime(other) {
                other
            } else {
                f
            }
        })
        .max()
        .unwrap();

    println!("{}", largest_pf);
    assert_eq!(largest_pf, 6857);
}
