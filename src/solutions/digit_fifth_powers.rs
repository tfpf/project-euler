pub fn solve() {
    let nums = (1i32..999999)
        .filter(|&num| {
            let mut n = num;
            let mut sum = 0;
            while num > 0 {
                sum += (n % 10).pow(5);
                n /= 10;
            }
            sum == num
        })
        .collect::<Vec<i32>>();
    println!("{:?}", nums);
}
