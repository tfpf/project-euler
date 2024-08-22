/// Potential prime numbers. Generates some small prime numbers and numbers
/// coprime to 30. Used for wheel factorisation with 2, 3 and 5.
pub struct PotentialPrimes {
    limit: i64,
    num: i64,
    offset: std::iter::Cycle<std::array::IntoIter<i64, 8>>,
}

impl PotentialPrimes {
    pub fn new(limit: i64) -> PotentialPrimes {
        PotentialPrimes {
            limit,
            num: 1,
            offset: [4, 2, 4, 2, 4, 6, 2, 6].into_iter().cycle(),
        }
    }
}

impl Iterator for PotentialPrimes {
    type Item = i64;
    fn next(&mut self) -> Option<i64> {
        self.num += match self.num {
            1 => 1,
            2 => 1,
            3 | 5 => 2,
            _ => self.offset.next().unwrap(),
        };
        if self.num > self.limit {
            None
        } else {
            Some(self.num)
        }
    }
}
