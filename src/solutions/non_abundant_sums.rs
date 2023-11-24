use crate::utils;

pub fn solve() -> i64 {
    let prime_divisors = utils::PrimeDivisors::new(28124);
    let mut abundant_numbers = vec![];
    for num in 12..28124 {
        // If the prime factorisation of a number is known, the sum of its
        // divisors is easily calculated using a formula. This block implements
        // said formula.
        let sum_of_divisors: i64 = prime_divisors
            .iter(num)
            .map(|(prime, power)| (prime.pow(power + 1) - 1) / (prime - 1))
            .product();
        if sum_of_divisors > num + num {
            abundant_numbers.push(num);
        }
    }
    let sum = (0..28124)
        .filter(|&num| {
            // Can this number be expressed as the sum of two abundant numbers?
            for abundant1 in abundant_numbers
                .iter()
                .take_while(|&&abundant1| abundant1 <= num)
            {
                let abundant2 = num - abundant1;
                match abundant_numbers.binary_search(&abundant2) {
                    Ok(_) => return false,
                    _ => {}
                };
            }
            true
        })
        .sum();

    assert_eq!(sum, 4179871);
    sum
}
