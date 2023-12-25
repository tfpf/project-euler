use crate::utils;

pub fn solve() -> i64 {
    // The totient function has some interesting properties which can be
    // exploited if the prime factorisation of a number is known.
    // φ(2.pow(a) * 3.pow(b) * ...) = φ(2.pow(a)) * φ(3.pow(b)) * ...
    // φ(2.pow(a)) = 2.pow(a) - 2.pow(a - 1)
    // φ(3.pow(b)) = 3.pow(b) - 3.pow(b - 1)
    // Hence, the ratio of a number 2.pow(a) * 3.pow(b) * ... to its totient is
    // 1/(1 - 1/2) * 1/(1 - 1/3) * ... which we can maximise by maximising the
    // the number of fractions multiplied. To do this, we choose our number as
    // the product of prime numbers.
    let mut product = 1;
    for prime in utils::SieveOfEratosthenes::new(20).iter() {
        let product_ = product * prime;
        if product_ > 1000000 {
            break;
        }
        product = product_
    }

    assert_eq!(product, 510510);
    product
}
