pub fn solve() {
    // No need to check numbers containing more than 6 digits, because they
    // will always be less than the sum of the fifth power of their digits.
    let sum: i32 = (10i32..1000000)
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
