/// Positive arbitrary-precision integer.
struct BigNum
{
    digits: Vec<u32>,
}

/// Helper function to add two numbers with a carry.
///
/// * `a`
/// * `b`
/// * `carry` - Either 0 or 1. Will be added to `a` and `b`, and set to 1 if
///   the sum exceeds 1_000_000_000, else 0.
///
/// -> Sum of `a`, `b` and `carry`, modulo 1_000_000_000.
fn adder(a: u32, b: u32, carry: &mut u32) -> u32
{
    let sum = a + b + *carry;
    let sum = if sum >= 1_000_000_000
    {
        *carry = 1;
        sum - 1_000_000_000
    }
    else
    {
        *carry = 0;
        sum
    };
    sum
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

/// Output.
impl std::fmt::Display for BigNum
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        for digit in self.digits.iter().rev()
        {
            let _ = write!(f, "{:0>9}", digit);
        }
        write!(f, "")
    }
}

/// Addition.
impl std::ops::Add for BigNum
{
    type Output = BigNum;
    fn add(self, other: BigNum) -> BigNum
    {
        // Determine the number with more digits. This will make building their
        // sum easy.
        let slen: i32 = self.digits.len() as i32;
        let olen: i32 = other.digits.len() as i32;
        let (long, short) = if slen >= olen { (&self, &other) } else { (&other, &self) };

        // Build the sum until one of them runs out of digits.
        let mut digits: Vec<u32> = vec![];
        let mut carry: u32 = 0;
        for (&ld, &sd) in long.digits.iter().zip(short.digits.iter())
        {
            let sum = adder(ld, sd, &mut carry);
            digits.push(sum);
        }

        // Build the rest of the sum.
        for &ld in long.digits.iter().skip(short.digits.len())
        {
            let sum = adder(ld, 0, &mut carry);
            digits.push(sum);
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
