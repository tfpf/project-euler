/// Implement the `d` function.
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
    let mut seen = [false; 10000];
    for i in 0..10000
    {
        if seen[i]
        {
            continue;
        }
        let d_i = sum_of_proper_divisors(i);
        if d_i != i && sum_of_proper_divisors(d_i) == i
        {
            seen[i] = true;
            seen[d_i] = true;
        }
    }
    let result = seen.iter().enumerate().fold(0, |sum, (i, &amicable)| if amicable { sum + i } else { sum });
    println!("{}", result);

    assert_eq!(result, 31626);
}
