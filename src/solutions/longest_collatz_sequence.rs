use crate::utils;

pub fn solve() -> i64 {
    let result = (1..1_000_000)
        .map(|num| {
            let collatz = utils::Collatz::new(num);
            (collatz.count(), num)
        })
        .max()
        .unwrap()
        .1;

    assert_eq!(result, 837799);
    result
}
