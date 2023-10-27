/// Double the number represented as a vector of digits, each digit being a
/// number in the range [0, 999_999_999].
///
/// * `num` - Number to double.
fn double(num: &mut Vec<i32>)
{
    let mut carry = 0;
    for digit in num.iter_mut()
    {
        *digit = *digit + *digit + carry;
        carry = if *digit >= 1_000_000_000
        {
            *digit -= 1_000_000_000;
            1
        }
        else
        {
            0
        };
    }
    if carry > 0
    {
        num.push(1);
    }
}

/// Add up all digits of a number.
///
/// * `n` - Number whose digits are to be summed.
///
/// -> Sum of the digits of `n`.
fn add_digits(n: i32) -> i32
{
    if n < 10
    {
        return n;
    }
    n % 10 + add_digits(n / 10)
}

/// Add up all digits of the number represented as a vector of digits, each
/// digit being a number in the range [0, 999_999_999].
///
/// * `num` - Number whose digits are to be summed.
///
/// -> Sum of the digits of `num`.
fn add_digits_all(num: &Vec<i32>) -> i32
{
    num.iter().map(|&n| add_digits(n)).sum()
}

/// Main function.
fn main()
{
    let mut num: Vec<i32> = vec![1];
    for _ in 1..=1000
    {
        double(&mut num);
    }
    let result = add_digits_all(&num);
    println!("{}", result);

    assert_eq!(result, 1366);
}
