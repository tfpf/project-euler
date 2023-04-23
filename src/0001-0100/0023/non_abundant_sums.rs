/// Add up all proper divisors of the given number.
///
/// * `num`
///
/// -> Sum of proper divisors of `num`.
fn sum_of_proper_divisors(num: usize) -> usize
{
    // Handle some simple cases.
    match num
    {
        0 | 1 => return 0,
        2 | 3 | 5 | 7 => return 1,
        4 => return 3,
        6 => return 6,
        8 => return 7,
        9 => return 4,
        _ => (),
    };

    // Handle the remaining cases. Note that 1 is always a proper divisor;
    // start counting from 2.
    let mut sum = 1;
    let upper = (num as f64).sqrt() as usize;
    for factor in 2..=upper
    {
        if num % factor == 0
        {
            sum += factor;
            let other_factor = num / factor;
            if other_factor != factor
            {
                sum += other_factor;
            }
        }
    }
    sum
}

/// Main function.
fn main()
{
    // This array indicates whether a particular number can be written as the
    // sum of two abundant numbers.
    let mut arr = [false; 28124];

    // Find all (relevant) abundant numbers.
    let mut abundant = vec![];
    for i in 0..28124
    {
        let di = sum_of_proper_divisors(i);
        if di > i
        {
            abundant.push(i);
        }
    }

    // Find all (relevant) sums of two abundant numbers.
    for (i, &num1) in abundant.iter().enumerate()
    {
        for &num2 in abundant.iter().rev().take(28123 - i)
        {
            let sum = num1 + num2;
            if sum < 28124
            {
                arr[sum] = true;
            }
        }
    }

    // Add all numbers which are not the sums of two abundant numbers.
    let result: usize = arr.iter().enumerate().map(|(i, &elem)| if elem { 0 } else { i }).sum();
    println!("{}", result);

    assert_eq!(result, 4179871);
}
