use crate::utils;

pub fn solve() -> i64 {
    // Space optimisation: store the indicators of 64 numbers in a single
    // element.
    let mut p234sum_expressible = vec![0u64; 781250];

    // The largest number whose square is within the limit is 7071.
    let primes = utils::SieveOfAtkin::new(7071).iter().collect::<Vec<i64>>();
    let primes_pow_2 = primes
        .iter()
        .map(|prime| prime.pow(2))
        .collect::<Vec<i64>>();
    let primes_pow_3 = primes
        .iter()
        .map(|prime| prime.pow(3))
        .take_while(|&prime_pow_3| prime_pow_3 < 50000000)
        .collect::<Vec<i64>>();
    let primes_pow_4 = primes_pow_2
        .iter()
        .map(|prime_pow_2| prime_pow_2.pow(2))
        .take_while(|&prime_pow_4| prime_pow_4 < 50000000)
        .collect::<Vec<i64>>();

    for a in primes_pow_2 {
        for b in primes_pow_3.iter().take_while(|&&b| a + b < 50000000) {
            for c in primes_pow_4.iter().take_while(|&&c| a + b + c < 50000000) {
                let num = (a + b + c) as usize;
                p234sum_expressible[num / 64] |= 1 << (num % 64);
            }
        }
    }
    let result = p234sum_expressible
        .iter()
        .map(|bitfield| bitfield.count_ones())
        .sum::<u32>();

    assert_eq!(result, 1097343);
    result as i64
}
