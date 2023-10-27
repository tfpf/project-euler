pub fn solve() {
    let mut a: i32 = 1;
    let mut b: i32 = 1;
    let mut sum: i32 = 0;
    while b < 4000000 {
        if b % 2 == 0 {
            sum += b;
        }
        let prev = a;
        a = b;
        b += prev;
    }

    println!("{}", sum);
    assert_eq!(sum, 4613732);
}
