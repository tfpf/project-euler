use crate::utils;

pub fn solve() -> i64 {
    let mut primes = 0;
    let result = (1..)
        .zip(utils::Polygonal::new(4))
        .step_by(2)
        .skip(1)
        .skip_while(|&(side, area)| {
            primes += [
                area,
                area - side + 1,
                area - 2 * side + 2,
                area - 3 * side + 3,
            ]
            .iter()
            .filter(|&&num| utils::is_prime(num))
            .count() as i64;
            primes * 10 >= 2 * side - 1
        })
        .next()
        .unwrap()
        .0;

    assert_eq!(result, 26241);
    result
}
