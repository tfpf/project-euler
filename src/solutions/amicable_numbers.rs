use crate::utils;

/// Implement the `d` function.
///
/// * `num`
///
/// -> Sum of proper divisors of `num`.
fn sum_of_proper_divisors(num: usize) -> usize {
    let divisors = utils::Divisors::new(num as i64);
    (divisors.sum::<i64>() - num as i64) as usize
}

pub fn solve() {
    let mut amicable = [false; 10000];
    for i in 0..10000 {
        if amicable[i] {
            continue;
        }
        let d_i = sum_of_proper_divisors(i);
        if d_i != i && sum_of_proper_divisors(d_i) == i {
            amicable[i] = true;
            amicable[d_i] = true;
        }
    }
    let result: usize = amicable
        .iter()
        .enumerate()
        .filter(|(_, is_amicable)| **is_amicable)
        .map(|(idx, _)| idx)
        .sum();

    println!("{}", result);
    assert_eq!(result, 31626);
}
