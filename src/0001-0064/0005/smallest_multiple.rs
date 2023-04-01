/// Main function.
fn main()
{
    let primes = [2, 3, 5, 7, 11, 13, 17, 19];
    let mut powers = [1; 8];
    for num in 2..21
    {
        for (prime, power) in primes.iter().zip(powers.iter_mut())
        {
            // If the current number is divisible by the current prime, find
            // the largest power of the latter the former is divisible by.
            if num % *prime == 0
            {
                loop
                {
                    let new_power = *power * *prime;
                    if num % new_power != 0
                    {
                        break;
                    }
                    *power = new_power;
                }
            }
        }
    }
    let product = powers.iter().fold(1, |p, elem| p * elem);
    println!("{}", product);

    assert_eq!(product, 232792560);
}
