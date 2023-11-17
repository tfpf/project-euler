use crate::utils;

pub fn solve() {
    let result = (1..1_000_000)
        .map(|num| {
            let collatz = utils::Collatz::new(num);
            (collatz.count(), num)
        })
        .max()
        .unwrap()
        .1;

    print!("{}", result);
    assert_eq!(result, 837799);
}
