pub fn solve() {
    let sum: u32 = (10..10000000)
        .filter(|&num| {
            num.to_string()
                .chars()
                .map(|c| c.to_digit(10).unwrap().pow(5))
                .sum::<u32>()
                == num
        })
        .sum();

    println!("{}", sum);
    assert_eq!(sum, 443839);
}
