use crate::utils;

fn replacements_are_prime(prime: i64, sieve: &utils::SieveOfEratosthenes) -> bool {
    for difference in [11, 101, 110, 1001, 1010, 1100, 10001, 10010, 10100, 11000, 100001, 100010, 100100, 101000, 110000, 1000001, 1000010, 1000100, 1001000, 1010000, 1100000] {
        if difference > prime {
            break;
        }
        if (0..=9).filter(|digit| {
            let num = prime + digit * difference;
            utils::is_prime(num)
        }).count() >= 8 {
            return true;
        }
    }




    false
}

pub fn solve() -> i64 {
    let sieve = utils::SieveOfEratosthenes::new(100000000);
    for prime in sieve.iter() {
        if replacements_are_prime(prime, &sieve) {
            println!("{}", prime);
        }
    }
    // After finding a prime number, check whether the numbers obtained by
    // adding 11, 22, 33 and so on (or 101, 202, 303 and so on) are prime.
    0
}
