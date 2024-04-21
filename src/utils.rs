#![deny(clippy::all)]
#![allow(clippy::len_without_is_empty)]
#![allow(clippy::manual_try_fold)]
#![allow(clippy::match_overlapping_arm)]
#![allow(clippy::new_without_default)]

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
    if num == 2 || num == 3 || num == 5 {
        return true;
    }
    if num < 2 || num % 2 == 0 || num % 3 == 0 || num % 5 == 0 {
        return false;
    }

    // Slow checks.
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
mod objects;
pub use objects::long::Long;
pub use objects::pandigital_checker::PandigitalChecker;
pub use objects::sieve_of_atkin::SieveOfAtkin;
pub use objects::poker_hand::PokerHand;
pub use objects::fraction::Fraction;


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

    fn lines(fname: &str) -> impl Iterator<Item = String> {
        let fhandle = std::fs::File::open(fname).unwrap();
        let reader = std::io::BufReader::new(fhandle);
        reader.lines().map(|line| line.unwrap())
    }

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
        for line in lines("res/tests/is_prime_large_test.txt") {
            let mut num_primality = line.split_ascii_whitespace();
            let num = num_primality.next().unwrap().parse().unwrap();
            let primality = num_primality.next().unwrap().parse().unwrap();
            assert_eq!(utils::is_prime(num), primality);
        }
    }

    #[test]
    fn gcd_test() {
        for line in lines("res/tests/gcd_test.txt") {
            let [a, b, g]: [i64; 3] = line
                .split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect::<Vec<i64>>()
                .try_into()
                .unwrap();
            assert_eq!(utils::gcd(a, a), a);
            assert_eq!(utils::gcd(b, b), b);
            assert_eq!(utils::gcd(g, g), g);
            assert_eq!(utils::gcd(a, b), g);
            assert_eq!(utils::gcd(b, a), g);
        }
    }

    #[test]
    fn isqrt_test() {
        assert_eq!(utils::isqrt(2i64.pow(53) - 1), 94906265);
        assert_eq!(utils::isqrt(2i64.pow(54) - 1), 134217727);
    }

    #[test]
    fn long_arithmetic_test() {
        for line in lines("res/tests/long_arithmetic_test.txt") {
            let mut expression = line.split_ascii_whitespace();

            // The first operation is always exponentiation. Hard-code it.
            let base = utils::Long::new(expression.next().unwrap());
            expression.next();
            let exp = expression.next().unwrap().parse().unwrap();
            let mut result = base.pow(exp);

            // The following operations can be done in order, because all
            // multiplications come before any additions.
            loop {
                let oper = expression.next().unwrap();
                let num = utils::Long::new(expression.next().unwrap());
                match oper {
                    "*" => result = &result * &num,
                    "+" => result += &num,
                    "=" => {
                        assert_eq!(result, num);
                        break;
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    #[test]
    fn long_factorial_test() {
        for line in lines("res/tests/long_factorial_test.txt") {
            let mut num_factorial = line.split_ascii_whitespace();
            let num = num_factorial.next().unwrap().parse().unwrap();
            let factorial = utils::Long::new(num_factorial.next().unwrap());
            assert_eq!(utils::Long::factorial(num), factorial);
        }
    }

    #[test]
    fn long_multiplication_test() {
        for line in lines("res/tests/long_multiplication_test.txt") {
            let mut mmp = line.split_ascii_whitespace();
            let multiplicand = utils::Long::new(mmp.next().unwrap());
            let multiplier = mmp.next().unwrap().parse::<u32>().unwrap();
            let product = utils::Long::new(mmp.next().unwrap());
            assert_eq!(&multiplicand * multiplier, product);
        }
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
        for line in lines("res/tests/continued_fraction_test.txt") {
            let mut num_terms = line.split_ascii_whitespace();
            let num = num_terms.next().unwrap().parse().unwrap();
            let terms = num_terms.next().unwrap().split(',').map(|s| s.parse().unwrap());
            assert!(utils::ContinuedFraction::new(num).eq(terms));
        }
    }
}
