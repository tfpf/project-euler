#![allow(clippy::match_overlapping_arm)]

/// Check whether the given number is prime.
///
/// * `num`
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
/// * `num` Must not be divisible by 2, 3 or 5. Must exceed 5.
fn is_prime_td(num: i64) -> bool {
    // No need to search for composite factors. We'll find prime factors (if
    // any) faster. Don't bother generating prime numbers. Potential prime
    // numbers are faster to generate.
    PotentialPrimes::new(isqrt(num)).all(|potential_prime| num % potential_prime != 0)
}

/// Check whether the given number is prime using the Miller-Rabin test.
///
/// * `num` Must not be divisible by 2, 3 or 5. Must exceed 5.
/// * `bases` Bases to perform the test with.
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

/// Check whether the given number is a palindrome.
///
/// * `num`
/// * `radix` Base to use to represent the number: 2, 10 or 16.
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
/// * `a` Must be non-negative.
/// * `b` Must be non-negative.
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
/// * `slice` Container with unique items.
///
/// Returns `true` if the next permutation was generated. Returns `false`
/// without modifying the container otherwise.
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
/// * `slice` Container with unique items.
///
/// Returns `true` if the previous permutation was generated. Returns `false`
/// without modifying the container otherwise.
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
/// * `base` Number to be exponentiated.
/// * `exp` Exponent.
/// * `modulus` Modulus.
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

/// Determine the number of times each digit appears in the given number.
///
/// * `num`
///
/// Returns an array in which each element is the number of occurrences of its
/// index.
pub fn digits_frequencies(num: i64) -> [u8; 10] {
    let mut frequency = [0; 10];
    for digit in Digits::new(num) {
        frequency[digit as usize] += 1;
    }
    frequency
}

/// Calculate the square root of an integer, rounded down. To be used until
/// integer square roots are stabilised in Rust. See
/// https://github.com/rust-lang/rust/issues/116226.
///
/// * `num`
///
/// Returns the square root of the number if it is non-negative. Returns the
/// same number otherwise.
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

mod objects;
pub use objects::fraction::Fraction;
pub use objects::long::Long;
pub use objects::pandigital_checker::PandigitalChecker;
pub use objects::poker_hand::PokerHand;
pub use objects::sieve_of_atkin::SieveOfAtkin;

mod iterators;
pub use iterators::bits::Bits;
pub use iterators::collatz::Collatz;
pub use iterators::continued_fraction::ContinuedFraction;
pub use iterators::cubes::Cubes;
pub use iterators::digits::Digits;
pub use iterators::divisors::Divisors;
pub use iterators::fibonacci::Fibonacci;
pub use iterators::polygonal::Polygonal;
pub use iterators::potential_primes::PotentialPrimes;
pub use iterators::primes::Primes;
pub use iterators::pythagorean_triplets::PythagoreanTriplets;

#[cfg(test)]
mod tests {
    use crate::utils;
    use std::io::BufRead;

    /// Convenience to create an iterator over the lines of a file.
    ///
    /// * `fname` File name.
    fn lines(fname: &str) -> impl Iterator<Item = String> {
        let fhandle = std::fs::File::open(fname).unwrap();
        let reader = std::io::BufReader::new(fhandle);
        reader.lines().map(|line| line.unwrap())
    }

    #[cfg(target_pointer_width = "64")]
    #[test]
    fn is_prime_small_test() {
        // Break the range into equal sub-ranges, and count the number of
        // primes in each sub-range in a separate thread.
        let (lower, upper, pieces) = (0, 3i64.pow(20), 3);
        let search_space = upper - lower;
        let search_space = search_space / pieces + if search_space % pieces == 0 { 0 } else { 1 };
        let num_of_primes = (lower..upper)
            .step_by(search_space as usize)
            .map(|lower| {
                let upper = std::cmp::min(lower + search_space, upper);
                std::thread::spawn(move || (lower..upper).filter(|&num| utils::is_prime(num)).count())
            })
            .collect::<Vec<_>>()
            .into_iter()
            .map(|worker| worker.join().unwrap())
            .sum::<usize>();
        assert_eq!(num_of_primes, 166677978);
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

    #[test]
    fn primes_smaller_test() {
        let num_of_primes = utils::Primes::new(2i64.pow(14)).count();
        assert_eq!(num_of_primes, 1900);
    }

    #[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
    #[test]
    fn primes_small_test() {
        let num_of_primes = utils::Primes::new(10i64.pow(9)).count();
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
