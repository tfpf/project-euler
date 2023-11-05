pub fn solve() {
    let sum: i32 = (10i32..10000000)
        .filter(|&num| {
            let mut n = num;
            let mut val = 0;
            while n > 0 {
                val += (n % 10).pow(5);
                n /= 10;
            }
            val == num
        })
        .sum();

    println!("{}", sum);
    assert_eq!(sum, 443839);
}
