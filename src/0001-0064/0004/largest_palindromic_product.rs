/// Check whether a number is a palindrome or not.
///
/// * `num` - Number to check.
fn is_palindrome(num: i32) -> bool
{
    let num = num.to_string();
    num == num.chars().rev().collect::<String>()
}

/// Main function.
fn main()
{
    let mut result = -1;
    for i in (100..1000).rev()
    {
        for j in (100..1000).rev()
        {
            let product = i * j;
            if product > result && is_palindrome(product)
            {
                result = product;
            }
        }
    }
    println!("{}", result);

    assert_eq!(result, 906609);
}
