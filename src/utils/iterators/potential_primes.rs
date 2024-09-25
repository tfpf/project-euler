/// Potential prime numbers. Generates numbers coprime to 30, starting from 7.
/// Used for wheel factorisation with 2, 3 and 5.
pub struct PotentialPrimes {
    limit: i64,
    num: i64,
    offset: Box<dyn Iterator<Item = i64>>,
}

impl PotentialPrimes {
    pub fn new(limit: i64) -> PotentialPrimes {
        PotentialPrimes {
            limit,
            num: 1,
            offset: Box::new([6, 4, 2, 4, 2, 4, 6, 2].into_iter().cycle()),
        }
    }
}

impl Iterator for PotentialPrimes {
    type Item = i64;
    fn next(&mut self) -> Option<i64> {
        self.num += self.offset.next().unwrap();
        if self.num > self.limit {
            None
        } else {
            Some(self.num)
        }
    }
}
