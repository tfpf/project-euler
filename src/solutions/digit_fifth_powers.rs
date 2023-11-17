use crate::utils;

pub fn solve() {
    // No need to check numbers containing more than 6 digits, because they
    // will always be less than the sum of the fifth power of their digits.
    let sum: i64 = (10..1000000)
        .filter(|&num| {
            utils::Digits::new(num)
                .map(|digit| digit.pow(5))
                .sum::<i64>()
                == num
        })
        .sum();

    print!("{}", sum);
    assert_eq!(sum, 443839);
}
