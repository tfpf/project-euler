/// Main function.
fn main()
{
    // Sieve of Eratosthenes.
    const LIMIT: usize = 2000000;
    let mut prime = [true; LIMIT + 1];
    prime[0] = false;
    prime[1] = false;
    for num in (2usize..).take_while(|n| n * n <= LIMIT)
    {
        if prime[num]
        {
            for multiple in (num * num..LIMIT + 1).step_by(num)
            {
                prime[multiple] = false;
            }
        }
    }
    let result = prime.iter().enumerate().fold(0usize, |s, (i, is_prime)| s + if *is_prime { i } else { 0 });
    println!("{}", result);

    assert_eq!(result, 142913828922);
}
