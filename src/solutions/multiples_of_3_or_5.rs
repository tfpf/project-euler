use crate::utils;

pub fn solve() -> i64 {
    // (1) 10 => 4 µs
    // (2) 100 => 4 µs
    // (3) 1000 => 15 µs
    // (4) 10000 => 90 µs
    // (5) 100000 => 890 µs
    // (6) 1000000 => 10 ms
    // (7) 10000000 => 130 ms
    // (8) 100000000 => 1600 ms
    // (9) 1000000000 => 18 s
    // utils::Primes::new(10usize.pow(9)).iter().last().unwrap()
    println!("{:?}", utils::SieveOfEratosthenes::new(10usize.pow(9)).iter().count());
    // utils::SieveOfEratosthenes::new(10usize.pow(2)).iter().count() as i64




    0
}
