/******************************************************************************
 * Functions.
 *****************************************************************************/

/// Check whether the given number is prime.
///
/// * `num` - Number to check for primality.
///
/// -> Whether `num` is prime.
pub fn is_prime(num: i64) -> bool {
    // Fast checks.
    match num {
        ..=1 => return false,
        2 | 3 | 5 => return true,
        _ => (),
    }
    if num % 2 == 0 {
        return false;
    }
    let idx = (num - 7) / 2;
    if idx < 64 {
        return 0x502DA2534C96996Di64 >> idx & 1 == 1;
    }

    // Slow checks.
    if num % 3 == 0 || num % 5 == 0 {
        return false;
    }
    match num {
        ..=100000 => is_prime_td(num),
        // The Miller-Rabin tests as performed below are deterministic for all
        // possible inputs. I chose the thresholds (after consulting some
        // tables) such that each is two digits longer than the previous.
        ..=38010306 => is_prime_mr(num, &vec![2, 9332593]),
        ..=1050535500 => is_prime_mr(num, &vec![336781006125, 9639812373923155]),
        ..=273919523040 => is_prime_mr(num, &vec![15, 7363882082, 992620450144556]),
        ..=31858317218646 => is_prime_mr(num, &vec![2, 642735, 553174392, 3046413974]),
        ..=3770579582154546 => is_prime_mr(num, &vec![2, 2570940, 880937, 610386380, 4130785767]),
        _ => is_prime_mr(num, &vec![2, 325, 9375, 28178, 450775, 9780504, 1795265022]),
    }
}

/// Check whether the given number is prime using trial division.
///
/// * `num` - Must not be divisible by 2, 3 or 5. Must exceed 100.
///
/// -> Whether `num` is prime.
fn is_prime_td(num: i64) -> bool {
    // No need to search for composite factors. We'll find prime factors (if
    // any) faster.
    PotentialPrimes::new(isqrt(num))
        .skip(3)
        .all(|potential_prime| num % potential_prime != 0)
}

/// Check whether the given number is prime using the Miller-Rabin test.
///
/// * `num` - Must not be divisible by 2, 3 or 5. Must exceed 100.
/// * `bases` - Bases to perform the test with.
///
/// -> Whether `num` is prime.
fn is_prime_mr(num: i64, bases: &Vec<i64>) -> bool {
    let num_minus_1 = num - 1;
    let twopower = num_minus_1.trailing_zeros();
    let multiplier = num_minus_1 >> twopower;
    'bases: for &base in bases {
        let mut residue = pow(base, multiplier as u64, num);
        // If this is 0, it means a wrong base was chosen, so the test is
        // inconclusive. Hence, I group it together with the two cases in which
        // it is suspected to be prime. This ensures that I never mislabel a
        // number as composite.
        if residue == 0 || residue == 1 || residue == num_minus_1 {
            continue;
        }
        for _ in 1..twopower {
            residue = pow(residue, 2, num);
            if residue == 1 {
                return false;
            }
            if residue == num_minus_1 {
                continue 'bases;
            }
        }
        return false;
    }
    true
}

/// Check whether a number is a palindrome or not.
///
/// * `num` - Number to check for palindromicity.
/// * `radix` - Base to use to represent the number: 2, 10 or 16.
///
/// -> Whether `num` is a palindrome in the specified base.
pub fn is_palindrome(num: i64, radix: i32) -> bool {
    let num = match radix {
        2 => format!("{:b}", num),
        10 => format!("{}", num),
        16 => format!("{:x}", num),
        _ => return false,
    };
    num == num.chars().rev().collect::<String>()
}

/// Calculate the greatest common divisor of two numbers.
///
/// * `a` - Must be non-negative.
/// * `b` - Must be non-negative.
///
/// -> GCD of the inputs.
pub fn gcd(a: i64, b: i64) -> i64 {
    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }

    // Binary GCD algorithm.
    let twopower = (a | b).trailing_zeros();
    let (mut a, mut b) = (a >> a.trailing_zeros(), b >> b.trailing_zeros());
    while a != b {
        (a, b) = if a > b { (a, b) } else { (b, a) };
        a -= b;
        a >>= a.trailing_zeros();
    }
    a << twopower
}

/// Generate the next permutation.
///
/// * `slice` - Object containing the unique items to permute.
///
/// -> Whether the next permutation was generated.
pub fn next_permutation<T: Copy + std::cmp::Ord>(slice: &mut [T]) -> bool {
    // Locate an inversion from the right.
    let Some(sorted_until) = (1..slice.len()).rev().find(|&idx| slice[idx - 1] < slice[idx]) else {
        return false;
    };

    // Find out where it can be placed while maintaining sorted order from the
    // right. (That means reverse order when looking from the left.)
    // Decrementing the search result is necessary to reorient ourselves from
    // looking from the right to looking from the left.
    let search_key = slice[sorted_until - 1];
    let target = sorted_until
        + match slice[sorted_until..].binary_search_by(|element| search_key.cmp(element)) {
            Ok(idx) => idx,
            Err(idx) => idx,
        }
        - 1;
    slice.swap(sorted_until - 1, target);
    slice[sorted_until..].reverse();
    true
}

/// Generate the previous permutation.
///
/// * `slice` - Object containing the unique items to permute.
///
/// -> Whether the previous permutation was generated.
pub fn prev_permutation<T: Copy + std::cmp::Ord>(slice: &mut [T]) -> bool {
    // Locate an anti-inversion from the right.
    let Some(sorted_until) = (1..slice.len()).rev().find(|&idx| slice[idx - 1] > slice[idx]) else {
        return false;
    };

    // Find out where it can be placed while maintaining reverse order from the
    // right. (That means sorted order when looking from the left.)
    // Decrementing the search result is necessary to avoid overshooting.
    let search_key = slice[sorted_until - 1];
    let target = sorted_until
        + match slice[sorted_until..].binary_search(&search_key) {
            Ok(idx) => idx,
            Err(idx) => idx,
        }
        - 1;
    slice.swap(sorted_until - 1, target);
    slice[sorted_until..].reverse();
    true
}

/// Perform modular exponentiation.
///
/// * `base` - Number to be exponentiated.
/// * `exp` - Exponent.
/// * `modulus` - Modulus.
///
/// -> Modular exponentiation of the given number.
pub fn pow(base: i64, exp: u64, modulus: i64) -> i64 {
    let (mut base, mut exp, modulus, mut multiplier) = (base as i128, exp, modulus as i128, 1);
    loop {
        if exp % 2 == 1 {
            multiplier = multiplier * base % modulus;
        }
        if exp <= 1 {
            return multiplier as i64;
        }
        exp /= 2;
        base = base.pow(2) % modulus;
    }
}

/// Determine the frequencies of all digits in a number.
///
/// * `num` - Number to analyse.
///
/// -> Map from digits to their frequencies.
pub fn digits_frequencies(num: i64) -> [u8; 10] {
    let mut frequency = [0; 10];
    for digit in Digits::new(num) {
        frequency[digit as usize] += 1;
    }
    frequency
}

/// Calculate the square root of an integer, rounded down. To be used until
/// integer square roots are stabilised.
///
/// * `num` - Number to root.
///
/// -> Square root. If negative, the same number is returned.
pub fn isqrt(mut num: i64) -> i64 {
    if num < 2 {
        return num;
    }
    let (mut result, mut one) = (0, 1 << (num.ilog2() & !1));
    while one != 0 {
        if num >= result + one {
            num -= result + one;
            result = (result >> 1) + one;
        } else {
            result >>= 1;
        }
        one >>= 2;
    }
    result
}

/******************************************************************************
 * Objects.
 *****************************************************************************/

/// Arbitrary-precision integer type which stores digits of a positive number
/// in base 1_000_000_000.
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Long {
    digits: Vec<i32>,
}
impl Long {
    /// Construct an arbitrary-precision integer from an iterator over decimal
    /// digits, least significant first.
    ///
    /// * `bytes` - Iterator which yields numbers from 0 to 9.
    ///
    /// -> Arbitrary-precision integer.
    fn create(bytes: impl Iterator<Item = u8>) -> Long {
        let mut long = Long { digits: vec![] };
        let (_, digit) = bytes.fold((0, 0), |(idx, digit), byte| {
            let digit = digit + 10i32.pow(idx) * (byte - b'0') as i32;
            if idx == 8 {
                long.digits.push(digit);
                (0, 0)
            } else {
                (idx + 1, digit)
            }
        });
        if digit > 0 || long.digits.is_empty() {
            long.digits.push(digit);
        }
        long
    }
    pub fn new(s: &str) -> Long {
        Long::create(s.bytes().rev())
    }
    pub fn from(digit: i32) -> Long {
        Long { digits: vec![digit] }
    }
    /// Calculate the factorial of a non-negative number.
    ///
    /// * `num` - Number whose factorial is to be calculated.
    pub fn factorial(num: i32) -> Long {
        match num {
            ..=-1 => panic!("factorials are not defined for negative integers"),
            0 | 1 => return Long::from(1),
            2 => return Long::from(2),
            3 => return Long::from(6),
            4 => return Long::from(24),
            5 => return Long::from(120),
            6 => return Long::from(720),
            7 => return Long::from(5040),
            8 => return Long::from(40320),
            9 => return Long::from(362880),
            10 => return Long::from(3628800),
            11 => return Long::from(39916800),
            12 => return Long::from(479001600),
            _ => (),
        }

        // Multiply the extremes, converting towards the centre. For example,
        // 9! is computed as (9 × 1) × (8 × 2) × (7 × 3) × (6 × 4) × 5, whereas
        // 10! is computed as (10 × 1) × (9 × 2) × (8 × 3) × (7 × 4) × (6 × 5).
        let partials = num / 2;
        let mut result = if num % 2 == 1 {
            Long::from(partials + 1)
        } else {
            Long::from(1)
        };
        let (mut multiplicand, mut delta) = (num, num - 2);
        for _ in 0..partials {
            result *= multiplicand;
            multiplicand += delta;
            delta -= 2;
        }
        result
    }
    pub fn reverse(&self) -> Long {
        Long::create(self.to_string().bytes())
    }
    /// Obtain the number of decimal digits of this number (i.e. its length).
    ///
    /// -> Length.
    pub fn len(&self) -> usize {
        match self.digits.len() {
            0 => 0,
            1 if self.digits[0] == 0 => 0,
            len => (len - 1) * 9 + self.digits.last().unwrap().to_string().len(),
        }
    }
    /// Calculate the sum of all decimal digits of this number.
    ///
    /// -> Sum.
    pub fn sum(&self) -> i64 {
        self.digits
            .iter()
            .map(|&digit| Digits::new(digit as i64).sum::<i64>())
            .sum()
    }
    /// Raise this number to the given power.
    ///
    /// * `exp` - Power.
    ///
    /// -> Value of this number raised to the given power.
    pub fn pow(&self, mut exp: u32) -> Long {
        // Multiplication is expensive, so these checks will improve
        // performance.
        let mut multiplier = Long::from(1);
        if exp == 0 {
            return multiplier;
        }
        let mut base = self.clone();
        if exp == 1 {
            return base;
        }
        loop {
            if exp % 2 == 1 {
                multiplier = &multiplier * &base;
            }
            if exp == 1 {
                return multiplier;
            }
            exp /= 2;
            base = &base * &base;
        }
    }
    fn adc(a: i32, b: i32, carry: i32) -> (i32, i32) {
        let sum = a + b + carry;
        if sum >= 1_000_000_000 {
            (sum - 1_000_000_000, 1)
        } else {
            (sum, 0)
        }
    }
    fn mlc(a: i32, b: i32, carry: i32) -> (i32, i32) {
        let product = a as i64 * b as i64 + carry as i64;
        ((product % 1_000_000_000) as i32, (product / 1_000_000_000) as i32)
    }
}
impl std::ops::AddAssign<&Long> for Long {
    fn add_assign(&mut self, other: &Long) {
        let mut carry = 0;
        for (sd, od) in self.digits.iter_mut().zip(other.digits.iter()) {
            (*sd, carry) = Long::adc(*sd, *od, carry);
        }
        for od in other.digits.iter().skip(self.digits.len()) {
            let sum_carry = Long::adc(0, *od, carry);
            self.digits.push(sum_carry.0);
            carry = sum_carry.1;
        }
        if carry > 0 {
            self.digits.push(carry);
        }
    }
}
impl std::ops::Add<&Long> for &Long {
    type Output = Long;
    fn add(self, other: &Long) -> Long {
        let mut result = self.clone();
        result += other;
        result
    }
}
impl std::ops::MulAssign<i32> for Long {
    fn mul_assign(&mut self, other: i32) {
        let mut carry = 0;
        for sd in self.digits.iter_mut() {
            (*sd, carry) = Long::mlc(*sd, other, carry);
        }
        if carry > 0 {
            self.digits.push(carry);
        }
    }
}
impl std::ops::Mul<i32> for &Long {
    type Output = Long;
    fn mul(self, other: i32) -> Long {
        let mut result = self.clone();
        result *= other;
        result
    }
}
impl std::ops::Mul<&Long> for &Long {
    type Output = Long;
    fn mul(self, other: &Long) -> Long {
        let mut result = Long { digits: vec![] };
        for (pad, od) in other.digits.iter().enumerate() {
            let mut partial_product = Long { digits: vec![0; pad] };
            let mut carry = 0;
            for sd in self.digits.iter() {
                let sum_carry = Long::mlc(*sd, *od, carry);
                partial_product.digits.push(sum_carry.0);
                carry = sum_carry.1;
            }
            if carry > 0 {
                partial_product.digits.push(carry);
            }
            result += &partial_product;
        }
        result
    }
}
impl std::fmt::Display for Long {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.digits
            .iter()
            .rev()
            .enumerate()
            .fold(Ok(()), |result, (index, digit)| {
                result.and_then(|_| {
                    if index == 0 {
                        write!(f, "{}", digit)
                    } else {
                        write!(f, "{:0>9}", digit)
                    }
                })
            })
    }
}

/// Check whether a bunch of numbers are pandigital with respect to the given
/// range.
pub struct PandigitalChecker {
    seen: [bool; 10],
    min_digit: usize,
    max_digit: usize,
}
impl PandigitalChecker {
    pub fn new(min_digit: usize, max_digit: usize) -> PandigitalChecker {
        PandigitalChecker {
            seen: [false; 10],
            min_digit,
            max_digit,
        }
    }
    pub fn renew(&mut self) {
        self.seen = [false; 10];
    }
    /// Update the internal array with the digits of a given number.
    ///
    /// * `num` - Number to update with.
    ///
    /// -> Whether all digits of the number were in range and not seen earlier.
    pub fn update(&mut self, num: i64) -> bool {
        for digit in Digits::new(num) {
            let digit = digit as usize;
            if digit < self.min_digit || digit > self.max_digit || self.seen[digit] {
                return false;
            }
            self.seen[digit] = true;
        }
        true
    }
    /// Check whether all digits in the range have been seen. This indicates
    /// pandigitality only if used in tandem with the above method.
    ///
    /// -> Pandigitality.
    pub fn check(&self) -> bool {
        self.seen
            .iter()
            .skip(self.min_digit)
            .take(self.max_digit - self.min_digit + 1)
            .all(|&digit_seen| digit_seen)
    }
}

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
            limit_rounded_isqrt: isqrt(limit_rounded as i64) as usize,
            sieve: vec![0; limit / 60 + 1],
        };
        sieve_of_atkin.init();
        sieve_of_atkin
    }
    fn init(&mut self) {
        self.algorithm_3_1(1);
        self.algorithm_3_1(13);
        self.algorithm_3_1(17);
        self.algorithm_3_1(29);
        self.algorithm_3_1(37);
        self.algorithm_3_1(41);
        self.algorithm_3_1(49);
        self.algorithm_3_1(53);
        self.algorithm_3_2(7);
        self.algorithm_3_2(19);
        self.algorithm_3_2(31);
        self.algorithm_3_2(43);
        self.algorithm_3_3(11);
        self.algorithm_3_3(23);
        self.algorithm_3_3(47);
        self.algorithm_3_3(59);

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
    fn algorithm_3_1(&mut self, delta: i32) {
        for f in 1..=15 {
            for g in (1..=30).step_by(2) {
                let quadratic = 4 * f * f + g * g;
                if delta == quadratic % 60 {
                    self.algorithm_4_1(delta, f, g, quadratic / 60);
                }
            }
        }
    }
    fn algorithm_3_2(&mut self, delta: i32) {
        for f in (1..=10).step_by(2) {
            for g in [2, 4, 8, 10, 14, 16, 20, 22, 26, 28] {
                let quadratic = 3 * f * f + g * g;
                if delta == quadratic % 60 {
                    self.algorithm_4_2(delta, f, g, quadratic / 60);
                }
            }
        }
    }
    fn algorithm_3_3(&mut self, delta: i32) {
        for (f, gstart) in (1..=10).zip([2, 1].into_iter().cycle()) {
            for g in (gstart..=30).step_by(2) {
                let quadratic = 3i32 * f * f - g * g;
                // Remainder can be negative, so perform modulo operation.
                if delta == quadratic.rem_euclid(60) {
                    self.algorithm_4_3(delta, f, g, quadratic.div_euclid(60));
                }
            }
        }
    }
    fn algorithm_4_1(&mut self, delta: i32, f: i32, g: i32, h: i32) {
        let (mut x, mut y0, mut k0) = (f as i64, g as i64, h as i64);
        while k0 < self.sieve.len() as i64 {
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
            while k < self.sieve.len() as i64 {
                self.sieve[k as usize] ^= 1u16 << SieveOfAtkin::SHIFTS[delta as usize];
                (k, y) = (k + y + 15, y + 30);
            }
        }
    }
    fn algorithm_4_2(&mut self, delta: i32, f: i32, g: i32, h: i32) {
        let (mut x, mut y0, mut k0) = (f as i64, g as i64, h as i64);
        while k0 < self.sieve.len() as i64 {
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
            while k < self.sieve.len() as i64 {
                self.sieve[k as usize] ^= 1u16 << SieveOfAtkin::SHIFTS[delta as usize];
                (k, y) = (k + y + 15, y + 30);
            }
        }
    }
    fn algorithm_4_3(&mut self, delta: i32, f: i32, g: i32, h: i32) {
        let (mut x, mut y0, mut k0) = (f as i64, g as i64, h as i64);
        loop {
            while k0 >= self.sieve.len() as i64 {
                if x <= y0 {
                    return;
                }
                (k0, y0) = (k0 - y0 - 15, y0 + 30);
            }
            let (mut k, mut y) = (k0, y0);
            while k >= 0 && y < x {
                self.sieve[k as usize] ^= 1u16 << SieveOfAtkin::SHIFTS[delta as usize];
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

/// A hand of poker.
#[derive(Eq, PartialEq)]
pub struct PokerHand {
    // Pairs in which the first number is the value of a card and the second is
    // its suit. The pairs shall be sorted in descending order of the value.
    hand: Vec<(u8, u8)>,
    // Used to rank this hand on an arbitrary scale. Higher is better.
    score: i32,
}
impl PokerHand {
    const HIGH_CARD: i32 = 0;
    const ONE_PAIR: i32 = 1 << 16;
    const TWO_PAIRS: i32 = 2 << 16;
    const THREE_OF_A_KIND: i32 = 3 << 16;
    const STRAIGHT: i32 = 4 << 16;
    const FLUSH: i32 = 5 << 16;
    const FULL_HOUSE: i32 = 6 << 16;
    const FOUR_OF_A_KIND: i32 = 7 << 16;
    const STRAIGHT_FLUSH: i32 = 8 << 16;
    const ROYAL_FLUSH: i32 = 9 << 16;
}
impl PokerHand {
    pub fn new(hand: &[&str]) -> PokerHand {
        let hand = hand
            .iter()
            .map(|card| {
                let card = card.as_bytes();
                let value = match card[0] {
                    b'T' => 10,
                    b'J' => 11,
                    b'Q' => 12,
                    b'K' => 13,
                    b'A' => 14,
                    value => value - b'0',
                };
                let suit = card[1];
                (value, suit)
            })
            .collect();
        let mut poker_hand = PokerHand { hand, score: 0 };
        poker_hand.calculate_score();
        poker_hand
    }
    fn calculate_score(&mut self) {
        // Sorting the cards by descending order of value makes analysing them
        // easier.
        self.hand.sort_by(|a, b| b.cmp(a));
        let same_suit = self.hand.iter().skip(1).all(|card| card.1 == self.hand[0].1);
        let consecutive_values = self
            .hand
            .iter()
            .enumerate()
            .skip(1)
            .all(|(idx, card)| card.0 == self.hand[idx - 1].0 - 1);

        if same_suit && consecutive_values {
            if self.hand[0].0 == 14 {
                self.score = PokerHand::ROYAL_FLUSH;
            } else {
                self.score = PokerHand::STRAIGHT_FLUSH;
            }
            return;
        }

        // How many times does each value appear?
        let mut values_frequencies = [0; 15];
        for card in &self.hand {
            let value = card.0 as usize;
            values_frequencies[value] += 1;
        }

        // Suppose two hands have the same rank, four of a kind: 3 clubs,
        // 3 diamonds, 3 hearts and 3 spades, and 6 clubs, 6 diamonds, 6 hearts
        // and 6 spades. The second hand is the winner, because the value
        // forming four of a kind is higher. To take care of such
        // possibilities, assign a different weightage to each rank, depending
        // on the values forming that rank. This weightage shall be the extra
        // score added to the rank. (If we have two pairs, assign greater
        // weightage to the pair with the greater value.) If the scores are
        // tied, the greatest values in each hand are compared. Hence, if the
        // rank is formed by all cards (e.g. straight or flush), no extra score
        // has to be added. The weightage must be chosen carefully so that it
        // does not cause the score to overflow into the score of the next
        // higher rank.
        let mut score_weightage = (1, 1 << 8, 1 << 12);
        let (mut twos, mut threes, mut fours) = (0, 0, 0);
        let extra_score: i32 = values_frequencies
            .iter()
            .enumerate()
            .map(|(value, frequency)| match frequency {
                2 => {
                    twos += 1;
                    let extra_score = score_weightage.0 * value as i32;
                    // Increase the weightage for the next pair (if any).
                    score_weightage.0 <<= 4;
                    extra_score
                }
                3 => {
                    threes += 1;
                    score_weightage.1 * value as i32
                }
                4 => {
                    fours += 1;
                    score_weightage.2 * value as i32
                }
                _ => 0,
            })
            .sum();

        if fours == 1 {
            self.score = PokerHand::FOUR_OF_A_KIND + extra_score;
            return;
        }
        if twos == 1 && threes == 1 {
            self.score = PokerHand::FULL_HOUSE + extra_score;
            return;
        }
        if same_suit {
            self.score = PokerHand::FLUSH;
            return;
        }
        if consecutive_values {
            self.score = PokerHand::STRAIGHT;
            return;
        }
        if threes == 1 {
            self.score = PokerHand::THREE_OF_A_KIND + extra_score;
            return;
        }
        if twos == 2 {
            self.score = PokerHand::TWO_PAIRS + extra_score;
            return;
        }
        if twos == 1 {
            self.score = PokerHand::ONE_PAIR + extra_score;
            return;
        }
        self.score = PokerHand::HIGH_CARD;
    }
}
impl Ord for PokerHand {
    fn cmp(&self, other: &PokerHand) -> std::cmp::Ordering {
        let compare = self.score.cmp(&other.score);
        if compare != std::cmp::Ordering::Equal {
            return compare;
        }
        for (scard, ocard) in self.hand.iter().zip(other.hand.iter()) {
            let compare = scard.0.cmp(&ocard.0);
            if compare != std::cmp::Ordering::Equal {
                return compare;
            }
        }
        std::cmp::Ordering::Equal
    }
}
impl PartialOrd for PokerHand {
    fn partial_cmp(&self, other: &PokerHand) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Rational numbers.
#[derive(Clone)]
pub struct Fraction {
    numerator: Long,
    denominator: Long,
}
impl Fraction {
    pub fn from(numerator: i32, denominator: i32) -> Fraction {
        Fraction {
            numerator: Long::from(numerator),
            denominator: Long::from(denominator),
        }
    }
    pub fn invert(&mut self) {
        std::mem::swap(&mut self.numerator, &mut self.denominator);
    }
    pub fn len(&self) -> (usize, usize) {
        (self.numerator.len(), self.denominator.len())
    }
    pub fn sum(&self) -> (i64, i64) {
        (self.numerator.sum(), self.denominator.sum())
    }
}
impl std::ops::AddAssign<i32> for Fraction {
    fn add_assign(&mut self, other: i32) {
        self.numerator += &(&self.denominator * other);
    }
}
impl std::ops::Add<i32> for &Fraction {
    type Output = Fraction;
    fn add(self, other: i32) -> Fraction {
        let mut result = self.clone();
        result += other;
        result
    }
}
impl std::fmt::Display for Fraction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}

/******************************************************************************
 * Iterators.
 *****************************************************************************/

/// Fibonacci sequence iterator.
pub struct Fibonacci {
    a: i64,
    b: i64,
}
impl Fibonacci {
    pub fn new(a: i64, b: i64) -> Fibonacci {
        Fibonacci { a, b }
    }
}
impl Iterator for Fibonacci {
    type Item = i64;
    fn next(&mut self) -> Option<i64> {
        let a = self.a;
        (self.a, self.b) = (self.b, self.a + self.b);
        Some(a)
    }
}

/// Triangular, pentagonal, hexagonal, etc. number iterator. Specify the
/// number of sides of the polygon as the argument to the constructor.
pub struct Polygonal {
    increment: i64,
    offset: i64,
    num: i64,
}
impl Polygonal {
    pub fn new(sides: i64) -> Polygonal {
        Polygonal {
            increment: sides - 2,
            offset: 1,
            num: 0,
        }
    }
    /// Find the index at which the given number would appear in a sequence of
    /// polygonal numbers (if it is a polygonal number).
    ///
    /// * `sides` - Number of sides of the polygon the sequence is based on.
    /// * `num` - Number whose index is to be found.
    ///
    /// -> Index.
    pub fn invert(sides: i64, num: i64) -> Result<i64, i64> {
        // A polygonal number is a quadratic function of the index it appears
        // at. Solve for the positive root of the corresponding quadratic
        // equation.
        let a = sides - 2;
        let b = 4 - sides;
        let c = -2 * num;
        let discriminant = b.pow(2) - 4 * a * c;
        let idx = (-b as f64 + (discriminant as f64).sqrt()) / (2.0 * a as f64);
        let idx_rounded = idx.round() as i64;
        if idx.fract() == 0.0 {
            Ok(idx_rounded)
        } else {
            Err(idx_rounded)
        }
    }
}
impl Iterator for Polygonal {
    type Item = i64;
    fn next(&mut self) -> Option<i64> {
        self.num += self.offset;
        self.offset += self.increment;
        Some(self.num)
    }
}

/// Cubes iterator. Generates cubes of integers without multiplication or
/// exponentiation.
pub struct Cubes {
    increment: i64,
    offset: i64,
    num: i64,
}
impl Cubes {
    pub fn new() -> Cubes {
        Cubes {
            increment: 6,
            offset: 1,
            num: 0,
        }
    }
}
impl Iterator for Cubes {
    type Item = i64;
    fn next(&mut self) -> Option<i64> {
        self.num += self.offset;
        self.offset += self.increment;
        self.increment += 6;
        Some(self.num)
    }
}

/// Collatz sequence iterator.
pub struct Collatz {
    num: i64,
    done: bool,
}
impl Collatz {
    pub fn new(num: i64) -> Collatz {
        Collatz { num, done: false }
    }
}
impl Iterator for Collatz {
    type Item = i64;
    fn next(&mut self) -> Option<i64> {
        if self.done {
            return None;
        }
        let num = self.num;
        self.num = if self.num % 2 == 0 {
            self.num / 2
        } else {
            3 * self.num + 1
        };
        self.done = num == 1;
        Some(num)
    }
}

/// Divisors iterator. Generates all divisors of a number in an unspecified
/// order. Positive numbers only!
pub struct Divisors {
    dividend: i64,
    limit: i64,
    current: i64,
    other: i64,
}
impl Divisors {
    pub fn new(dividend: i64) -> Divisors {
        Divisors {
            dividend,
            limit: isqrt(dividend),
            current: 0,
            other: 0,
        }
    }
}
impl Iterator for Divisors {
    type Item = i64;
    fn next(&mut self) -> Option<i64> {
        if self.other > 0 {
            let other = self.other;
            self.other = 0;
            return Some(other);
        }
        loop {
            self.current += 1;
            if self.current > self.limit {
                return None;
            }
            if self.dividend % self.current == 0 {
                let other = self.dividend / self.current;
                if other != self.current {
                    self.other = other;
                }
                return Some(self.current);
            }
        }
    }
}

/// Prime divisors iterator. Generates all prime divisors of a number in
/// ascending order. Positive numbers only!
pub struct PrimeDivisors {
    // If I actually find all prime numbers to iterate over (instead of just
    // using potential prime numbers), performance drops significantly.
    potential_primes: PotentialPrimes,
    num: i64,
}
impl PrimeDivisors {
    pub fn new(num: i64) -> PrimeDivisors {
        PrimeDivisors {
            potential_primes: PotentialPrimes::new(num),
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

/// Digits iterator. Generates the decimal digits of a number from least
/// significant to most significant. Positive numbers only!
pub struct Digits {
    num: i64,
}
impl Digits {
    pub fn new(num: i64) -> Digits {
        Digits { num }
    }
}
impl Iterator for Digits {
    type Item = i64;
    fn next(&mut self) -> Option<i64> {
        if self.num == 0 {
            None
        } else {
            let digit = self.num % 10;
            self.num /= 10;
            Some(digit)
        }
    }
}

/// Bits iterator. Generates the binary digits of a number from least
/// significant to most significant. Positive numbers only!
pub struct Bits {
    num: i64,
}
impl Bits {
    pub fn new(num: i64) -> Bits {
        Bits { num }
    }
}
impl Iterator for Bits {
    type Item = i64;
    fn next(&mut self) -> Option<i64> {
        if self.num == 0 {
            None
        } else {
            let bit = self.num & 1;
            self.num >>= 1;
            Some(bit)
        }
    }
}

/// Pythagorean triplets iterator. Generates all Pythagorean triplets with the
/// given sum. The triplets are generated using a well-known parametrisation
/// which can represent any primitive triplet. Positive numbers only!
pub struct PythagoreanTriplets {
    semiperimeter: i64,
    m_ub: i64,
    m: i64,
}
impl PythagoreanTriplets {
    pub fn new(perimeter: i64) -> PythagoreanTriplets {
        let semiperimeter = perimeter / 2;
        // Non-strict upper bound for the parameter `m`.
        let m_ub = isqrt(semiperimeter);
        PythagoreanTriplets {
            semiperimeter,
            m_ub,
            m: 1,
        }
    }
}
impl Iterator for PythagoreanTriplets {
    type Item = (i64, i64, i64);
    fn next(&mut self) -> Option<(i64, i64, i64)> {
        loop {
            self.m += 1;
            if self.m > self.m_ub {
                return None;
            }
            if self.semiperimeter % self.m != 0 {
                continue;
            }
            let m = self.m;
            let remaining_term = self.semiperimeter / m;
            let remaining_odd = remaining_term >> remaining_term.trailing_zeros();
            let m_plus_n_lb = m + 1 + m % 2;
            for m_plus_n in (m_plus_n_lb..)
                .step_by(2)
                .take_while(|&m_plus_n| m_plus_n < 2 * m && m_plus_n <= remaining_odd)
            {
                if remaining_odd % m_plus_n == 0 && gcd(m_plus_n, m) == 1 {
                    let d = remaining_term / m_plus_n;
                    let n = m_plus_n - m;
                    let a = (m.pow(2) - n.pow(2)) * d;
                    let b = 2 * m * n * d;
                    let c = (m.pow(2) + n.pow(2)) * d;
                    return Some((a, b, c));
                }
            }
        }
    }
}

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

/// Generate the continued fraction representation of the square root of a
/// number. The elements generated after the first shall constitute the
/// repeating terms in the continued fraction.
pub struct ContinuedFraction {
    num: i64,
    a0: i64,
    numerator_addend: i64,
    denominator: i64,
}
impl ContinuedFraction {
    pub fn new(num: i64) -> ContinuedFraction {
        ContinuedFraction {
            num,
            a0: isqrt(num),
            numerator_addend: 0,
            denominator: 1,
        }
    }
}
impl Iterator for ContinuedFraction {
    type Item = i64;
    fn next(&mut self) -> Option<i64> {
        // This will happen if the given number was a perfect square. We will
        // also use this as the condition for detecting repeating terms.
        if self.denominator == 0 {
            return None;
        }

        // If a part of the continued fraction is
        //     (num.sqrt() + numerator_addend) / denominator
        // then the next term of the continued fraction, `numerator_addend` and
        // `denominator` can be found using a recurrence relation.
        let a = (self.a0 + self.numerator_addend) / self.denominator;
        self.numerator_addend = a * self.denominator - self.numerator_addend;
        self.denominator = (self.num - self.numerator_addend.pow(2)) / self.denominator;

        // When this happens, the terms will start repeating.
        if a == self.a0 * 2 {
            self.denominator = 0;
        }
        Some(a)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils;
    use std::io::BufRead;

    #[cfg(target_pointer_width = "64")]
    #[test]
    fn is_prime_smaller_test() {
        let num_of_primes = (0..2i64.pow(32)).filter(|&num| utils::is_prime(num)).count();
        assert_eq!(num_of_primes, 203280221);
    }

    #[cfg(target_pointer_width = "64")]
    #[test]
    fn is_prime_small_test() {
        let num_of_primes = (2i64.pow(32)..2i64.pow(33)).filter(|&num| utils::is_prime(num)).count();
        assert_eq!(num_of_primes, 190335585);
    }

    #[test]
    fn is_prime_large_test() {
        let fhandle = std::fs::File::open("res/is_prime_large_test.txt").unwrap();
        let reader = std::io::BufReader::new(fhandle);
        for line in reader.lines() {
            let line = line.unwrap();
            let mut num_primality = line.split_ascii_whitespace();
            let num = num_primality.next().unwrap().parse().unwrap();
            let primality = num_primality.next().unwrap().parse().unwrap();
            assert_eq!(utils::is_prime(num), primality);
        }
    }

    #[test]
    fn gcd_test() {
        let coprime_pairs = (0..10i64.pow(8))
            .zip((0..10i64.pow(8)).rev())
            .filter(|&(a, b)| utils::gcd(a, b) == 1)
            .count();
        assert_eq!(coprime_pairs, 58752000);
    }

    #[test]
    fn isqrt_test() {
        assert_eq!(utils::isqrt(2i64.pow(53) - 1), 94906265);
        assert_eq!(utils::isqrt(2i64.pow(54) - 1), 134217727);
    }

    #[test]
    fn long_test() {
        let mut num = &utils::Long::new("43").pow(37) * &utils::Long::from(745683);
        num += &utils::Long::factorial(51);
        assert_eq!(
            num.to_string(),
            "3597031455246992664728898500113748859466269359952342048214143659169"
        );
    }

    #[test]
    fn sieve_of_atkin_smaller_test() {
        let num_of_primes = utils::SieveOfAtkin::new(2usize.pow(14)).iter().count();
        assert_eq!(num_of_primes, 1900);
    }

    #[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
    #[test]
    fn sieve_of_atkin_small_test() {
        let num_of_primes = utils::SieveOfAtkin::new(10usize.pow(9)).iter().count();
        assert_eq!(num_of_primes, 50847534);
    }

    #[cfg(target_pointer_width = "64")]
    #[test]
    fn sieve_of_atkin_large_test() {
        let num_of_primes = utils::SieveOfAtkin::new(2usize.pow(36)).iter().count();
        assert_eq!(num_of_primes, 2874398515);
    }

    #[test]
    fn continued_fraction_test() {
        let fhandle = std::fs::File::open("res/continued_fraction_test.txt").unwrap();
        let reader = std::io::BufReader::new(fhandle);
        for line in reader.lines() {
            let line = line.unwrap();
            let mut num_terms = line.split_ascii_whitespace();
            let num = num_terms.next().unwrap().parse().unwrap();
            let terms = num_terms.next().unwrap().split(',').map(|s| s.parse().unwrap());
            assert!(utils::ContinuedFraction::new(num).eq(terms));
        }
    }
}
