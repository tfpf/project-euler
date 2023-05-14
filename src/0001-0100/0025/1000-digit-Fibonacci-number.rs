struct BigNum
{
    digits: Vec<i32>,
}
impl std::ops::Add for &BigNum
{
    type Output = BigNum;
    fn add(self, other: &BigNum) -> BigNum
    {
        let mut digits: Vec<i32> = vec![];
        let mut carry = 0;
        for (&sd, &od) in self.digits.iter().zip(other.digits.iter())
        {
            let sum = sd + od + carry;
            let sum = if sum >= 1_000_000_000 { carry = 1; sum - 1_000_000_000 } else { carry = 0; sum };
            digits.push(sum);
        }
        let self_len = self.digits.len();
        let other_len = other.digits.len();
        let (bigger, skip) = if self_len > other_len { (self, other_len) } else { (other, self_len) };
        for d in bigger.digits.iter().skip(skip)
        {
            let sum = d + carry;
            let sum = if sum >= 1_000_000_000 { carry = 1; sum - 1_000_000_000 } else { carry = 0; sum };
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

fn count_digits(num: &BigNum) -> usize
{
    let less_significant = (num.digits.len() - 1) * 9;
    let last = num.digits.last().unwrap();
    let most_significant = last.to_string().len();
    less_significant + most_significant
}

/// Find the index of the Fibonacci number with 1000 digits.
fn thousand_digits() -> usize
{
    let mut a = BigNum { digits: vec![1] };
    let mut b = BigNum { digits: vec![1] };
    for i in 3..
    {
        let c = &a + &b;
        a = b;
        b = c;
        if count_digits(&b) == 1000
        {
            return i;
        }
    }
    0
}

/// Main function
fn main()
{
    let result = thousand_digits();
    println!("{}", result);

    assert_eq!(result, 4782);
}
