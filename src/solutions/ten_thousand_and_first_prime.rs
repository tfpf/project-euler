pub fn solve() {
    // Estimate the largest number using an approximation of the prime-counting
    // function. Then use the sieve of Eratosthenes.
    const LIMIT: usize = 120000;
    let mut prime = [true; LIMIT + 1];
    prime[0] = false;
    prime[1] = false;
    for num in (2usize..).take_while(|c| c * c <= LIMIT) {
        // If this number is prime, mark its multiples starting from its square
        // as composite (smaller multiples have already been marked as
        // composite).
        if prime[num] {
            for multiple in (num * num..LIMIT + 1).step_by(num) {
                prime[multiple] = false;
            }
        }
    }
    let result = prime
        .iter()
        .enumerate()
        .filter(|(_, c)| **c)
        .nth(10000)
        .unwrap()
        .0;

    println!("{}", result);
    assert_eq!(result, 104743);
}
