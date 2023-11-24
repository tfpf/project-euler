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

    let sum = (0..28124)
        .filter(|&num| {
            // Can this number be expressed as the sum of two abundant numbers?
            for abundant1 in abundant_numbers
                .iter()
                .take_while(|&&abundant1| abundant1 <= num)
            {
                let abundant2 = num - abundant1;
                match abundant_numbers.binary_search(&abundant2) {
                    Ok(_) => return false,
                    _ => (),
                };
            }
            true
        })
        .sum();

    assert_eq!(sum, 4179871);
    sum
}
