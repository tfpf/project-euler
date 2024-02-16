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
    let fhandle = std::fs::File::open("res/gcd_test.txt").unwrap();
    let reader = std::io::BufReader::new(fhandle);
    for line in reader.lines() {
        let line = line.unwrap();
        let [a, b, g]: [i64; 3] = line
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect::<Vec<i64>>()
            .try_into()
            .unwrap();
        assert_eq!(a, utils::gcd(a, a));
        assert_eq!(b, utils::gcd(b, b));
        assert_eq!(g, utils::gcd(g, g));
        assert_eq!(g, utils::gcd(a, b));
        assert_eq!(g, utils::gcd(b, a));
    }
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
