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

    #[test]
    fn test_sieve_of_eratosthenes() {
        let num_of_primes = utils::SieveOfEratosthenes::new(2usize.pow(32))
            .iter()
            .count();
        assert_eq!(num_of_primes, 203280221);
    }
}
