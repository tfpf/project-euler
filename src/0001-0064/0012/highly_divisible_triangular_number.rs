/// Check how many divisors the given number has.
///
/// * `num` - Number to check.
///
/// -> `true` if `num` has 500 or more divisors, else `false`.
fn count_div(num: i32) -> bool
{
    let mut count = 0;
    for candidate in (2..).take_while(|f| f * f <= num)
    {
        if num % candidate == 0
        {
            count += 1;

            // If this is a divisor, the quotient of the division is also a
            // divisor. We've already performed a modulo operation above, so
            // this division shouldn't cost us anything.
            if num / candidate != candidate
            {
                count += 1;
            }
            if count >= 500
            {
                return true;
            }
        }
    }
    false
}

/// Find the first highly divisible triangular number.
///
/// -> Highly divisible triangular number.
fn get_hdtn() -> i32
{
    let mut num = 0;
    for i in 1..
    {
        num += i;
        if count_div(num)
        {
            return num;
        }
    }
    0
}

/// Main function.
fn main()
{
    let result = get_hdtn();
    println!("{}", result);

    assert_eq!(result, 76576500);
}
