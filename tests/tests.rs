#[cfg(test)]
mod tests {
    use project_euler::utils;

    #[test]
    fn is_prime_test() {
        assert_eq!((0..10i64.pow(3)).filter(|&num| utils::is_prime(num)).count(), 168);
    }
}
