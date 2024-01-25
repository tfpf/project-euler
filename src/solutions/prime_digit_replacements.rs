use crate::utils;

pub fn solve() -> i64 {
    const LIMIT: usize = 1000000;
    let sieve = utils::SieveOfAtkin::new(LIMIT);
    for prime in sieve.iter() {
        let idx_digits = utils::Digits::new(prime)
            .enumerate()
            .filter(|&(_, digit)| digit <= 2)
            .collect::<Vec<(usize, i64)>>();
        for &(i, idigit) in &idx_digits {
            for &(j, jdigit) in idx_digits.iter().take_while(|(j, _)| *j < i) {
                for &(k, kdigit) in idx_digits.iter().take_while(|(k, _)| *k < j) {
                    if idigit == jdigit && jdigit == kdigit {
                        let jump =
                            10usize.pow(i as u32) + 10usize.pow(j as u32) + 10usize.pow(k as u32);
                        let candidates = 10 - idigit as usize;
                        if (prime as usize..)
                            .step_by(jump)
                            .take(candidates)
                            .filter(|&num| sieve.is_prime(num))
                            .count()
                            == 8
                        {
                            return prime;
                        }
                    }
                }
            }
        }
    }

    0
}
