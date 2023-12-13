/******************************************************************************
 * Macros.
 *****************************************************************************/

/// Exchange two variables.
macro_rules! swap {
    ($a:expr, $b:expr) => {
        ($a, $b) = ($b, $a)
    };
}

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
        (..=1) => return false,
        2 | 3 | 5 => return true,
        _ => (),
    }
    if num % 2 == 0 {
        return false;
    }
    let idx = (num - 7) >> 1;
    if idx < 64 {
        return 0x502DA2534C96996Di64 >> idx & 1 == 1;
    }

    // Slow checks.
    if num % 3 == 0 || num % 5 == 0 {
        return false;
    }
    match num {
        (..=100000) => is_prime_tbd(num),
        num => is_prime_mr(num),
    }
}

/// Check whether the given number is prime using trial by division. Use wheel
/// factorisation with 2 and 3.
///
/// * `num` - Number to check for primality.
///
/// -> Whether `num` is prime.
fn is_prime_tbd(num: i64) -> bool {
    let (mut factor, mut offset) = (7i64, 4);
    while factor.pow(2) <= num {
        if num % factor == 0 {
            return false;
        }
        factor += offset;
        offset = 6 - offset;
    }
    true
}

/// Check whether the given number is prime using the Miller-Rabin test. This
/// is probabilistic in general; however, this implementation is exact for all
/// 32-bit integers (signed or unsigned).
///
/// * `num` - Number to check for primality.
///
/// -> Whether `num` is prime.
fn is_prime_mr(num: i64) -> bool {
    let num_minus_1 = num - 1;
    let twopower = num_minus_1.trailing_zeros();
    let multiplier = num_minus_1 >> twopower;
    'next_prime: for prime in [2, 7, 61] {
        let mut residue = pow(prime, multiplier as u64, num);
        if residue == 1 || residue == num_minus_1 {
            continue;
        }
        for _ in 1..twopower {
            let residue_ = pow(residue, 2, num);
            if residue_ == 1 {
                return false;
            }
            if residue_ == num_minus_1 {
                continue 'next_prime;
            }
            residue = residue_;
        }
        if residue != 1 && residue != num_minus_1 {
            return false;
        }
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
/// * `a`
/// * `b`
///
/// -> GCD of the inputs.
pub fn gcd(a: i64, b: i64) -> i64 {
    let (a, b) = if a > b { (a, b) } else { (b, a) };
    if b == 0 {
        return a;
    }
    let remainder = a % b;
    if remainder == 0 {
        b
    } else {
        gcd(b, remainder)
    }
}

/// Check for leap years.
///
/// * `year`
///
/// -> Whether the input is a leap year.
pub fn is_leap(year: i64) -> bool {
    year % 4 == 0 && year % 100 != 0 || year % 400 == 0
}

/// Determine the number of days in the given month of the given year.
///
/// * `year`
/// * `month` - Number from 1 to 12.
///
/// -> Number of days, or 0 if the month is invalid.
pub fn days_in(year: i64, month: i64) -> i64 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap(year) {
                29
            } else {
                28
            }
        }
        _ => 0,
    }
}

/// Find the length of the repeating part of the decimal representation of the
/// reciprocal of a prime number.
///
/// * `prime` - Prime number.
///
/// -> Recurrence cycle length.
pub fn recurrence_length(prime: i64) -> i64 {
    // The digits (i.e. the sequence of quotients) will start repeating when
    // the remainder becomes 1 for the second time.
    let mut rem = 1;
    for length in 1.. {
        rem = rem * 10 % prime;
        if rem == 1 {
            return length;
        }
    }
    unreachable!();
}

/// Generate the next permutation.
///
/// * `slice` - Object containing the unique items to permute.
///
/// -> Whether the next permutation was generated.
pub fn next_permutation<T: Copy + std::cmp::Ord>(slice: &mut [T]) -> bool {
    // Locate an inversion from the right.
    let Some(sorted_until) = (1..slice.len())
        .rev()
        .filter(|&idx| slice[idx - 1] < slice[idx])
        .next()
    else {
        return false;
    };

    // Find out where it can be placed while maintaining sorted order from the
    // right. (That means reverse order when looking from the left.)
    // Decrementing the search result is necessary to reorient ourselves from
    // looking from the right to looking from the left.
    let search_key = slice[sorted_until - 1];
    let target = sorted_until
        + match slice[sorted_until..].binary_search_by(|element| element.cmp(&search_key).reverse())
        {
            Ok(idx) => idx,
            Err(idx) => idx,
        }
        - 1;
    swap!(slice[sorted_until - 1], slice[target]);
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
    let Some(sorted_until) = (1..slice.len())
        .rev()
        .filter(|&idx| slice[idx - 1] > slice[idx])
        .next()
    else {
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
    swap!(slice[sorted_until - 1], slice[target]);
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
        if exp & 1 == 1 {
            multiplier = multiplier * base % modulus;
        }
        if exp <= 1 {
            return multiplier as i64;
        }
        exp >>= 1;
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
        Long {
            digits: vec![digit],
        }
    }
    /// Calculate the factorial of a non-negative number.
    ///
    /// * `num` - Number whose factorial is to be calculated.
    pub fn factorial(num: i32) -> Long {
        match num {
            (..=-1) => panic!("factorials are not defined for negative integers"),
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
        let partials = num >> 1;
        let mut result = if num & 1 == 1 {
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
            if exp & 1 == 1 {
                multiplier = &multiplier * &base;
            }
            if exp == 1 {
                return multiplier;
            }
            exp >>= 1;
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
        (
            (product % 1_000_000_000) as i32,
            (product / 1_000_000_000) as i32,
        )
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
            let mut partial_product = Long {
                digits: vec![0; pad],
            };
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
            min_digit: min_digit,
            max_digit: max_digit,
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
            .fold(true, |pandigital, &digit_seen| pandigital && digit_seen)
    }
}

/// Construct the sieve of Eratosthenes. This is stored as an array of bytes
/// in which the bits of the first byte (from least significant to most
/// significant) indicate the primality of 1, 7, 11, 13, 17, 19, 23 and 29; the
/// second byte, of 31, 37, 41, 43, 47, 49, 53 and 59; and so on. (These are
/// the numbers coprime to 30. Any number not coprime to 30 is guaranteed to be
/// composite, with 2, 3 and 5 being the only exceptions.) In effect, wheel
/// factorisation with 2, 3 and 5 is used.
pub struct SieveOfEratosthenes {
    limit: usize,
    bitfields: Vec<u8>,
}
impl SieveOfEratosthenes {
    // Differences between consecutive elements of
    // [1, 7, 11, 13, 17, 19, 23, 29]. Starting from 1, repeatedly adding these
    // numbers cyclically will yield all numbers congruent to them.
    const OFFSETS: [usize; 8] = [6, 4, 2, 4, 2, 4, 6, 2];
    // Indices are residues modulo 30. Values are indices into
    // [1, 7, 11, 13, 17, 19, 23, 29] at which the residue appears. If the
    // value is 8, it means that that residue does not appear in said array.
    const RESIDUE_MAP: [usize; 30] = [
        8, 0, 8, 8, 8, 8, 8, 1, 8, 8, 8, 2, 8, 3, 8, 8, 8, 4, 8, 5, 8, 8, 8, 6, 8, 8, 8, 8, 8, 7,
    ];
}
impl SieveOfEratosthenes {
    pub fn new(limit: usize) -> SieveOfEratosthenes {
        let bitfields_len = (limit + 1) / 30 + if (limit + 1) % 30 == 0 { 0 } else { 1 };
        let mut sieve_of_eratosthenes = SieveOfEratosthenes {
            limit: limit,
            bitfields: vec![255; bitfields_len],
        };
        sieve_of_eratosthenes.init();
        sieve_of_eratosthenes
    }
    fn init(&mut self) {
        // In the first byte, only 1 is not prime.
        self.bitfields[0] = 254;

        // In the remaining bytes, sieve out the composite numbers.
        let mut num = 1;
        for bitfields_idx in 0..self.bitfields.len() {
            if num * num > self.limit {
                break;
            }
            for offsets_idx in 0..8 {
                if self.bitfields[bitfields_idx] >> offsets_idx & 1 == 1 {
                    for multiple in (num * num..=self.limit).step_by(num) {
                        let (bitfields_idx, offsets_idx) = (
                            multiple / 30,
                            SieveOfEratosthenes::RESIDUE_MAP[multiple % 30],
                        );
                        if offsets_idx < 8 {
                            self.bitfields[bitfields_idx] &= !(1 << offsets_idx);
                        }
                    }
                }
                num += SieveOfEratosthenes::OFFSETS[offsets_idx];
            }
        }
    }
    /// Determine whether the given number is prime. The number must be less
    /// than or equal to the number with which this object was constructed.
    ///
    /// * `num` - Number to check.
    pub fn is_prime(&self, num: usize) -> bool {
        if num > self.limit {
            panic!("primality is not determined for numbers beyond the limit");
        }
        if num < 2 {
            return false;
        }
        if num == 2 || num == 3 || num == 5 {
            return true;
        }
        let (bitfields_idx, offsets_idx) = (num / 30, SieveOfEratosthenes::RESIDUE_MAP[num % 30]);
        if offsets_idx == 8 {
            return false;
        }
        return self.bitfields[bitfields_idx] >> offsets_idx & 1 == 1;
    }
    /// Iterate over all prime numbers until the number this object was
    /// constructed with.
    ///
    /// -> Prime iterator.
    pub fn iter(&self) -> impl Iterator<Item = i64> + '_ {
        let mut num = 1;
        [2, 3, 5]
            .into_iter()
            .chain(
                self.bitfields
                    .iter()
                    .flat_map(|bitfield| {
                        (0..8).map(move |offsets_idx| (offsets_idx, bitfield >> offsets_idx & 1))
                    })
                    .map(move |(offsets_idx, bit)| {
                        let pair = (bit, num);
                        num += SieveOfEratosthenes::OFFSETS[offsets_idx];
                        pair
                    })
                    .filter(|&(bit, _)| bit == 1)
                    .map(|(_, num)| num),
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
        let mut poker_hand = PokerHand {
            hand: hand,
            score: 0,
        };
        poker_hand.calculate_score();
        poker_hand
    }
    fn calculate_score(&mut self) {
        // Sorting the cards by descending order of value makes analysing them
        // easier.
        self.hand.sort_by(|a, b| b.cmp(a));
        let same_suit = self
            .hand
            .iter()
            .skip(1)
            .all(|card| card.1 == self.hand[0].1);
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
        Fibonacci { a: a, b: b }
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
    pub fn invert(sides: i64, num: i64) -> Option<i64> {
        // A polygonal number is a quadratic function of the index it appears
        // at. Solve for the positive root of the corresponding quadratic
        // equation.
        let a = sides - 2;
        let b = 4 - sides;
        let c = -2 * num;
        let discriminant = b.pow(2) - 4 * a * c;
        let idx = (-b as f64 + (discriminant as f64).sqrt()) / (2.0 * a as f64);
        if idx.fract() == 0.0 {
            Some(idx as i64)
        } else {
            None
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

/// Collatz sequence iterator.
pub struct Collatz {
    num: i64,
    done: bool,
}
impl Collatz {
    pub fn new(num: i64) -> Collatz {
        Collatz {
            num: num,
            done: false,
        }
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
            dividend: dividend,
            limit: (dividend as f64).sqrt() as i64,
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
    num: i64,
    divisor: i64,
    offset: i64,
}
impl PrimeDivisors {
    pub fn new(num: i64) -> PrimeDivisors {
        PrimeDivisors {
            num: num,
            divisor: 1,
            offset: 2,
        }
    }
}
impl Iterator for PrimeDivisors {
    type Item = (i64, u32);
    fn next(&mut self) -> Option<(i64, u32)> {
        loop {
            // After checking for divisibility by 2, 3 and 5, check only
            // numbers which differ from a multiple of 6 by exactly 1, because
            // only they can be prime numbers. If I actually find all prime
            // numbers to iterate over before running this loop, performance
            // drops significantly.
            self.divisor = match self.divisor {
                1 => 2,
                2 => 3,
                3 => 5,
                d => {
                    let d = d + self.offset;
                    self.offset = 6 - self.offset;
                    d
                }
            };
            if self.num < self.divisor {
                return None;
            }
            let mut power = 0;
            while self.num % self.divisor == 0 {
                self.num /= self.divisor;
                power += 1;
            }
            if power > 0 {
                return Some((self.divisor, power));
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
        Digits { num: num }
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
        Bits { num: num }
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
        let m_ub = (semiperimeter as f64).sqrt() as i64;
        PythagoreanTriplets {
            semiperimeter: semiperimeter,
            m_ub: m_ub,
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
            let m_plus_n_lb = m + 1 + (m & 1);
            for m_plus_n in (m_plus_n_lb..)
                .step_by(2)
                .take_while(|&m_plus_n| m_plus_n < 2 * m)
            {
                if remaining_term % m_plus_n == 0 && gcd(m_plus_n, m) == 1 {
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
