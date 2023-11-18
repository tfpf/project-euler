pub fn solve() -> i64 {
    let sum = (1..1000).filter(|num| num % 3 == 0 || num % 5 == 0).sum();

    assert_eq!(sum, 233168);
    sum
}
