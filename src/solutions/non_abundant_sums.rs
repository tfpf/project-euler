use crate::utils;

pub fn solve() -> i64 {
    let primes = utils::Primes::new(28124).iter().collect::<Vec<i64>>();
    let abundant_numbers = (12..28124)
        .filter(|&num| {
            let mut num = num;
            let sum_of_divisors_lb = num + num;
            let mut sum_of_divisors = 1;
            for &prime in &primes {
                if prime > num {
                    break;
                }
                let mut power = 0;
                while num % prime == 0 {
                    num /= prime;
                    power += 1;
                }
                sum_of_divisors *= (prime.pow(power + 1) - 1) / (prime - 1);
            }
            sum_of_divisors > sum_of_divisors_lb
        })
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
        .filter(|(_, &abundant)| !abundant)
        .map(|(elem, _)| elem)
        .sum();

    assert_eq!(sum, 4179871);
    sum as i64
}
