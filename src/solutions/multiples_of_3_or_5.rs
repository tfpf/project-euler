pub fn solve() {
    let sum: i32 = (1..1000).filter(|num| num % 3 == 0 || num % 5 == 0).sum();

    println!("{}", sum);
    assert_eq!(sum, 233168);
}
