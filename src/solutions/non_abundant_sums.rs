use crate::utils;

pub fn solve() -> i64 {
    let abundant_numbers = (0..28124)
        .filter(|&num| utils::Divisors::new(num).sum::<i64>() > num + num)
        .collect::<Vec<i64>>();

    let mut abundant_sum = [false; 28124];
    for i in 0..abundant_numbers.len() - 1 {
        for j in i..abundant_numbers.len() {
            let sum = (abundant_numbers[i] + abundant_numbers[j]) as usize;
            if sum >= abundant_sum.len() {
                break;
            }
            abundant_sum[sum] = true;
        }
    }
    let sum: usize = abundant_sum
        .iter()
        .enumerate()
        .filter_map(|(elem, &abundant)| if abundant { None } else { Some(elem) })
        .sum();

    assert_eq!(sum, 4179871);
    sum as i64
}
