/// Check whether the given number is prime by searching for any prime factors.
/// Use the fact that a prime factor, by virtue of being prime, is 2 or 3, or
/// differs from 6 by exactly 1.
///
/// * `num` - Number to check for primality.
///
/// -> Whether `num` is prime.
fn is_prime(num: i64) -> bool
{
    if num % 2 == 0 || num % 3 == 0
    {
        return false;
    }
    for candidate in (5i64..).step_by(6).take_while(|f| f * f <= num)
    {
        if num % candidate == 0 || num % (candidate + 2) == 0
        {
            return false;
        }
    }
    true
}

/// Out of two numbers, find the one which is the larger prime factor of a
/// given number.
///
/// * `largest_pf` - Current largest prime factor. Will be set to `candidate`
///   if it is a larger prime factor of `num`.
/// * `candidate` - Candidate largest prime factor.
/// * `num` - Number which is supposed to be divisible by the factors.
fn check_factor(largest_pf: &mut i64, candidate: i64, num: i64)
{
    if num % candidate != 0
    {
        return;
    }
    let other_candidate = num / candidate;
    if *largest_pf < candidate && is_prime(candidate)
    {
        *largest_pf = candidate;
    }
    if *largest_pf < other_candidate && is_prime(other_candidate)
    {
        *largest_pf = other_candidate;
    }
}

/// Main function.
fn main()
{
    let num: i64 = 600851475143;
    let mut largest_pf: i64 = -1;
    for candidate in (5i64..).step_by(6).take_while(|f| f * f <= num)
    {
        check_factor(&mut largest_pf, candidate, num);
        check_factor(&mut largest_pf, candidate + 2, num);
    }
    println!("{}", largest_pf);

    assert_eq!(largest_pf, 6857);
}
