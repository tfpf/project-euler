use crate::utils;

/// A. O. L. Atkin and D. J. Bernstein, "Prime Sieves Using Binary Quadratic
/// Forms", in Mathematics of Computation, vol. 73, no. 246, pp. 1023–1030.
/// Generate prime numbers using the sieve of Atkin. This sieves prime numbers
/// up to 1000000000 nearly twice as fast as my wheel-factorised sieve of
/// Eratosthenes implementation (which I have now removed). It only determines
/// the primality of numbers coprime to 60, because other numbers are
/// guaranteed to be composite. (Exceptions 2, 3 and 5 are handled separately.)
pub struct SieveOfAtkin {
    limit: usize,
    limit_rounded: usize,
    limit_rounded_isqrt: usize,
    sieve: Vec<u16>,
}
impl SieveOfAtkin {
    // Consecutive differences between coprime residues modulo 60: 1, 7, 11,
    // 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 49, 53 and 59.
    const OFFSETS: [usize; 16] = [6, 4, 2, 4, 2, 4, 6, 2, 6, 4, 2, 4, 2, 4, 6, 2];
    // Position of the bit indicating the primality of a coprime residue modulo
    // 60 in a 16-element bitfield. For non-coprime residues, the value is 16.
    const SHIFTS: [u8; 60] = [
        16, 0, 16, 16, 16, 16, 16, 1, 16, 16, 16, 2, 16, 3, 16, 16, 16, 4, 16, 5, 16, 16, 16, 6, 16, 16, 16, 16, 16,
        7, 16, 8, 16, 16, 16, 16, 16, 9, 16, 16, 16, 10, 16, 11, 16, 16, 16, 12, 16, 13, 16, 16, 16, 14, 16, 16, 16,
        16, 16, 15,
    ];
}
impl SieveOfAtkin {
    /// Construct the sieve of Atkin up to and including the given number.
    ///
    /// * `limit` - Non-strict upper bound.
    ///
    /// -> Sieve of Atkin.
    pub fn new(limit: usize) -> SieveOfAtkin {
        // Strict upper bound divisible by 60.
        let limit_rounded = (limit - limit % 60)
            .checked_add(60)
            .expect("overflow detected; argument too large");
        let mut sieve_of_atkin = SieveOfAtkin {
            limit,
            limit_rounded,
            limit_rounded_isqrt: utils::isqrt(limit_rounded as i64) as usize,
            sieve: vec![0; limit / 60 + 1],
        };
        sieve_of_atkin.init();
        sieve_of_atkin
    }
    fn init(&mut self) {
        let mut sieve3 = self.sieve.clone();
        let mut sieve2 = self.sieve.clone();
        let sieve1 = self.sieve.as_mut_slice();
        std::thread::scope(|s| {
            s.spawn(|| {
                for delta in [1, 13, 17, 29, 37, 41, 49, 53] {
                    SieveOfAtkin::algorithm_3_1(delta, sieve1);
                }
            });
            s.spawn(|| {
                for delta in [7, 19, 31, 43] {
                    SieveOfAtkin::algorithm_3_2(delta, &mut sieve2);
                }
            });
            s.spawn(|| {
                for delta in [11, 23, 47, 59] {
                    SieveOfAtkin::algorithm_3_3(delta, &mut sieve3);
                }
            });
        });

        // Combine the results. Since no two threads operated on bits at the
        // same position, the bitfields can simply be ORed.
        for sieve_idx in 0..sieve1.len() {
            sieve1[sieve_idx] |= sieve2[sieve_idx] | sieve3[sieve_idx];
        }
        drop(sieve2);
        drop(sieve3);

        // Mark composite all numbers divisible by the squares of primes.
        let mut num: usize = 1;
        let mut offset = SieveOfAtkin::OFFSETS.iter().cycle();
        'sieve: for sieve_idx in 0..self.sieve.len() {
            for shift in 0..16 {
                if self.sieve[sieve_idx] >> shift & 1 == 1 {
                    let num_sqr = num.pow(2);
                    for multiple in (num_sqr..)
                        .step_by(num_sqr)
                        .take_while(|&multiple| multiple < self.limit_rounded)
                    {
                        self.sieve[multiple / 60] &= !(1u32 << SieveOfAtkin::SHIFTS[multiple % 60]) as u16;
                    }
                }
                num += offset.next().unwrap();
                if num > self.limit_rounded_isqrt {
                    break 'sieve;
                }
            }
        }
    }
    fn algorithm_3_1(delta: i32, sieve: &mut [u16]) {
        for f in 1..=15 {
            for g in (1..=30).step_by(2) {
                let quadratic = 4 * f * f + g * g;
                if delta == quadratic % 60 {
                    SieveOfAtkin::algorithm_4_1(delta, f, g, quadratic / 60, sieve);
                }
            }
        }
    }
    fn algorithm_3_2(delta: i32, sieve: &mut [u16]) {
        for f in (1..=10).step_by(2) {
            for g in [2, 4, 8, 10, 14, 16, 20, 22, 26, 28] {
                let quadratic = 3 * f * f + g * g;
                if delta == quadratic % 60 {
                    SieveOfAtkin::algorithm_4_2(delta, f, g, quadratic / 60, sieve);
                }
            }
        }
    }
    fn algorithm_3_3(delta: i32, sieve: &mut [u16]) {
        for (f, gstart) in (1..=10).zip([2, 1].into_iter().cycle()) {
            for g in (gstart..=30).step_by(2) {
                let quadratic = 3i32 * f * f - g * g;
                // Remainder can be negative, so perform modulo operation.
                if delta == quadratic.rem_euclid(60) {
                    SieveOfAtkin::algorithm_4_3(delta, f, g, quadratic.div_euclid(60), sieve);
                }
            }
        }
    }
    fn algorithm_4_1(delta: i32, f: i32, g: i32, h: i32, sieve: &mut [u16]) {
        let (mut x, mut y0, mut k0) = (f as i64, g as i64, h as i64);
        while k0 < sieve.len() as i64 {
            (k0, x) = (k0 + 2 * x + 15, x + 15);
        }
        loop {
            (k0, x) = (k0 - 2 * x + 15, x - 15);
            if x <= 0 {
                return;
            }
            while k0 < 0 {
                (k0, y0) = (k0 + y0 + 15, y0 + 30);
            }
            let (mut k, mut y) = (k0, y0);
            while k < sieve.len() as i64 {
                sieve[k as usize] ^= 1u16 << SieveOfAtkin::SHIFTS[delta as usize];
                (k, y) = (k + y + 15, y + 30);
            }
        }
    }
    fn algorithm_4_2(delta: i32, f: i32, g: i32, h: i32, sieve: &mut [u16]) {
        let (mut x, mut y0, mut k0) = (f as i64, g as i64, h as i64);
        while k0 < sieve.len() as i64 {
            (k0, x) = (k0 + x + 5, x + 10);
        }
        loop {
            (k0, x) = (k0 - x + 5, x - 10);
            if x <= 0 {
                return;
            }
            while k0 < 0 {
                (k0, y0) = (k0 + y0 + 15, y0 + 30);
            }
            let (mut k, mut y) = (k0, y0);
            while k < sieve.len() as i64 {
                sieve[k as usize] ^= 1u16 << SieveOfAtkin::SHIFTS[delta as usize];
                (k, y) = (k + y + 15, y + 30);
            }
        }
    }
    fn algorithm_4_3(delta: i32, f: i32, g: i32, h: i32, sieve: &mut [u16]) {
        let (mut x, mut y0, mut k0) = (f as i64, g as i64, h as i64);
        loop {
            while k0 >= sieve.len() as i64 {
                if x <= y0 {
                    return;
                }
                (k0, y0) = (k0 - y0 - 15, y0 + 30);
            }
            let (mut k, mut y) = (k0, y0);
            while k >= 0 && y < x {
                sieve[k as usize] ^= 1u16 << SieveOfAtkin::SHIFTS[delta as usize];
                (k, y) = (k - y - 15, y + 30);
            }
            (k0, x) = (k0 + x + 5, x + 10);
        }
    }
    pub fn is_prime(&self, num: usize) -> bool {
        if num < 2 {
            return false;
        }
        if num == 2 || num == 3 || num == 5 {
            return true;
        }
        let (num_div_60, num_mod_60) = (num / 60, num % 60);
        if num_div_60 >= self.sieve.len() {
            panic!("queried number is out of range of the sieve")
        }
        self.sieve[num_div_60] & (1u32 << SieveOfAtkin::SHIFTS[num_mod_60]) as u16 != 0
    }
    pub fn iter(&self) -> impl Iterator<Item = i64> + '_ {
        let mut num: usize = 1;
        let mut offset = SieveOfAtkin::OFFSETS.iter().cycle();
        [2, 3, 5]
            .into_iter()
            .chain(
                self.sieve
                    .iter()
                    .flat_map(|bitfield| (0..16).map(move |shift| bitfield >> shift & 1 == 1))
                    .filter_map(move |is_prime| {
                        let filtered = if is_prime { Some(num) } else { None };
                        num += offset.next().unwrap();
                        filtered
                    }),
            )
            .take_while(|&num| num <= self.limit)
            .map(|num| num as i64)
    }
}
