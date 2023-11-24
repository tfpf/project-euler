use crate::utils;

pub fn solve() -> i64 {
    let abundant_numbers = (0..=28123)
        .filter(|&num| utils::Divisors::new(num).sum::<i64>() > num + num)
        .collect::<Vec<i64>>();
    let sum: i64 = (0..=28123)
        .filter(|num| {
            // Can this number be expressed as the sum of two abundant numbers?
            for abundant1 in abundant_numbers
                .iter()
                .take_while(|&abundant1| abundant1 <= num)
            {
                let abundant2 = num - abundant1;
                // If it can be, filter it out.
                match abundant_numbers.binary_search(&abundant2) {
                    Ok(_) => return false,
                    _ => (),
                }
            }
            true
        })
        .sum();

    assert_eq!(sum, 4179871);
    sum
}
