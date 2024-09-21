use crate::utils;

/// Prime numbers.
pub struct Primes {
    initial_primes: std::array::IntoIter<i64, 3>,
    potential_primes: utils::PotentialPrimes,
    lookup: std::collections::HashMap<i64, i64>,
    residues: u32,
}

impl Primes {
    /// Generates the first three prime numbers (2, 3 and 5) unconditionally.
    /// Then generates the prime numbers up to the given number.
    ///
    /// * `limit`
    pub fn new(limit: i64) -> Self {
        Self {
            initial_primes: [2, 3, 5].into_iter(),
            potential_primes: utils::PotentialPrimes::new(limit),
            lookup: std::collections::HashMap::from([(9, 3), (25, 5)]),
            residues: 0x208A2882,
        }
    }

    fn next_initial_prime(&mut self) -> Option<i64> {
        self.initial_primes.next()
    }

    fn next_other_prime(&mut self) -> Option<i64> {
        loop {
            let potential_prime = self.potential_primes.next()?;
            match self.lookup.remove(&potential_prime) {
                None => {
                    self.lookup.insert(potential_prime.pow(2), potential_prime);
                    return Some(potential_prime);
                }
                Some(offset) => {
                    let mut num = potential_prime + 2 * offset;
                    while self.lookup.contains_key(&num) || self.residues >> (num % 30) & 1 == 0 {
                        num += 2 * offset;
                    }
                    self.lookup.insert(num, offset);
                }
            }
        }
    }
}

impl Iterator for Primes {
    type Item = i64;
    fn next(&mut self) -> Option<Self::Item> {
        self.next_initial_prime().or_else(|| self.next_other_prime())
    }
}
