use crate::utils;

pub fn solve() -> i64 {
    let mut sum = 0;
    let primes_prefix_sum = utils::Primes::new(1000000)
        .iter()
        .map(|prime| {
            sum += prime;
            sum
        })
        .collect::<Vec<i64>>();

    let mut prime_and_window = (0, 0);
    for i in (1..primes_prefix_sum.len()).rev() {
        for j in (0..i).rev() {
            let consecutive_primes_sum = primes_prefix_sum[i] - primes_prefix_sum[j];
            if consecutive_primes_sum >= 1000000 {
                break;
            }
            let window = i - j;
            if utils::is_prime(consecutive_primes_sum) && window > prime_and_window.1 {
                prime_and_window = (consecutive_primes_sum, window);
            }
        }
    }
    let result = prime_and_window.0;

    assert_eq!(result, 997651);
    result
}
