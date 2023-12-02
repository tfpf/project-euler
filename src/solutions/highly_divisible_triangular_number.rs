use crate::utils;

pub fn solve() -> i64 {
    let primes = utils::Primes::new(76576501).iter().collect::<Vec<i64>>();
    let result = utils::Polygonal::new(3)
        .filter(|&num| {
            let mut num = num;
            let mut divisors = 1;
            for &prime in primes.iter() {
                if num < prime {
                    break;
                }
                let mut power = 0;
                while num % prime == 0 {
                    num /= prime;
                    power += 1;
                }
                divisors *= 1 + power;
            }
            divisors >= 500
        })
        .next()
        .unwrap();

    assert_eq!(result, 76576500);
    result
}
