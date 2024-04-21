use crate::utils;

/// Prime divisors iterator. Generates all prime divisors of a number in
/// ascending order. Positive numbers only!
pub struct PrimeDivisors {
    // If I actually find all prime numbers to iterate over (instead of just
    // using potential prime numbers), performance drops significantly.
    potential_primes: utils::PotentialPrimes,
    num: i64,
}
impl PrimeDivisors {
    pub fn new(num: i64) -> PrimeDivisors {
        PrimeDivisors {
            potential_primes: utils::PotentialPrimes::new(num),
            num,
        }
    }
}
impl Iterator for PrimeDivisors {
    type Item = (i64, u32);
    fn next(&mut self) -> Option<(i64, u32)> {
        loop {
            let potential_prime = match self.potential_primes.next() {
                Some(potential_prime) if potential_prime <= self.num => potential_prime,
                _ => return None,
            };
            let mut power = 0;
            while self.num % potential_prime == 0 {
                self.num /= potential_prime;
                power += 1;
            }
            // Since I divide out the number by all potential primes I find,
            // the potential primes I do find are actually prime.
            if power > 0 {
                return Some((potential_prime, power));
            }
        }
    }
}
