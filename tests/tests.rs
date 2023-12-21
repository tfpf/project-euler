#[cfg(test)]
mod tests {
    use project_euler::utils;

    #[test]
    fn is_prime_test() {
        let num_of_primes = (0..2i64.pow(32))
            .filter(|&num| utils::is_prime(num))
            .count();
        assert_eq!(num_of_primes, 203280221);
    }
}
