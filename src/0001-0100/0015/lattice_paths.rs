/// Calculate the greatest common divisor of two positive integers.
///
/// * `m`
/// * `n`
///
/// -> GCD of `m` and `n`.
fn gcd(m: i64, n: i64) -> i64
{
    let (m, n) = if m > n { (m, n) } else { (n, m) };
    let rem = m % n;
    if rem == 0
    {
        return n;
    }
    gcd(n, rem)
}

/// Main function.
fn main()
{
    let mut numerator: i64 = 1;
    let mut denominator: i64 = 1;
    for (n, d) in (21..=40).zip(1..=20)
    {
        numerator *= n;
        denominator *= d;
        let g = gcd(numerator, denominator);
        numerator /= g;
        denominator /= g;
    }
    let result = numerator / denominator;
    println!("{}", result);

    assert_eq!(result, 137846528820);
}
