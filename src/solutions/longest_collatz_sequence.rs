/// Collatz procedure. Use 64-bit integers to avoid overflow.
///
/// * `num` - Number to apply the procedure to.
///
/// -> Tuple of the number of terms in the Collatz sequence and `num`.
fn collatz(num: i64) -> (i32, i64)
{
    let mut num_ = num;
    let mut terms = 1;
    while num_ != 1
    {
        if num_ % 2 == 0
        {
            num_ /= 2;
        }
        else
        {
            num_ = 3 * num_ + 1;
        }
        terms += 1;
    }
    (terms, num)
}

/// Main function.
fn main()
{
    let result = (1..1_000_000).map(collatz).max().unwrap().1;
    println!("{}", result);

    assert_eq!(result, 837799);
}
