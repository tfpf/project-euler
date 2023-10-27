pub fn solve() {
    let sum = (1..1000).fold(0, |sum, element| {
        if element % 3 == 0 || element % 5 == 0 {
            sum + element
        } else {
            sum
        }
    });

    println!("{}", sum);
    assert_eq!(sum, 233168);
}
