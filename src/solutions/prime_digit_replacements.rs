use crate::utils;

pub fn solve() -> i64 {
    let primes = utils::Primes::new(100000);
    let sieve = primes.sieve();
    let primes = primes.iter().collect::<Vec<i64>>();
    for i in 1..primes.len() {
        for j in 0..i {
            let mut difference = primes[i] - primes[j];
            let mut tenpower = 1;
            while difference % 10 == 0 {
                difference /= 10;
                tenpower *= 10;
            }
            let digit = difference % 10;
            if difference % digit == 0 {
                difference /= digit;
            }
            match difference {
                11 | 101 | 1001 | 10001 | 100001 => (),
                _ => continue,
            }
            let difference = difference * tenpower;
            if (0..=9).map(|digit| (primes[i] + difference * digit) as usize).filter(|&num| num < sieve.len()).map(|num| sieve[num]).filter(|&p| p).count() >= 4 {
                println!("{} {} {}", primes[i], primes[i] + difference, primes[i] + difference * 2);
            }
        }
    }










    0
}
