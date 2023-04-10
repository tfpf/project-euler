/// Main function.
fn main()
{
    let mut sum: i32 = 0;
    for i in 1..1000
    {
        if i % 3 == 0 || i % 5 == 0
        {
            sum += i;
        }
    }
    println!("{}", sum);

    assert_eq!(sum, 233168);
}
