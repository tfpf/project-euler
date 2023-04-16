/// Positive arbitrary-precision integer.
struct BigNum
{
    digits: Vec<u32>,
}

/// Constructor.
impl BigNum
{
    fn new(num: u32) -> BigNum
    {
        BigNum
        {
            digits: vec![num],
        }
    }
}

/// Multiplication.
impl std::ops::Mul<u32> for BigNum
{
    type Output = BigNum;
    fn mul(self, other: u32) -> BigNum
    {
        let mut digits: Vec<u32> = vec![];
        let mut carry: u32 = 0;
        for digit in self.digits
        {
            let product: u64 = digit as u64 * other as u64 + carry as u64;
            let product: u32 = if product >= 1_000_000_000
            {
                carry = (product / 1_000_000_000) as u32;
                product % 1_000_000_000
            }
            else
            {
                carry = 0;
                product
            } as u32;
            digits.push(product);
        }
        if carry > 0
        {
            digits.push(carry);
        }
        BigNum
        {
            digits: digits,
        }
    }
}

/// Calculate the factorial of a number.
///
/// * `num`
///
/// -> Factorial of `num`.
fn factorial(num: u32) -> BigNum
{
    let mut product = BigNum::new(1);
    match num
    {
        0 | 1 => return product,
        _ => (),
    };

    for i in 2..=num
    {
        product = product * i;
    }
    product
}

/// Add up the digits of a number.
///
/// * `num`
///
/// -> Sum of the digits of `num`.
fn digit_sum(num: u32) -> u32
{
    if num < 10
    {
        return num;
    }
    num % 10 + digit_sum(num / 10)
}

/// Main function.
fn main()
{
    let f = factorial(100);
    let result: u32 = f.digits.iter().map(|&d| digit_sum(d)).sum();
    println!("{}", result);

    assert_eq!(result, 648);
}
