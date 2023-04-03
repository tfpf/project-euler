/// Search for the only Pythagorean triplet with sum 1000.
///
/// -> Product of the triplet.
fn find_triplet() -> i32
{
    for a in 1..334
    {
        for c in 334..998
        {
            let b = 1000 - a - c;
            if a >= b || b >= c
            {
                continue;
            }
            if b * b == c * c - a * a
            {
                return a * b * c;
            }
        }
    }

    // Shouldn't happen, since we are told that a triplet satisfying the given
    // condition exists.
    return -1;
}

/// Main function.
fn main()
{
    let result = find_triplet();
    println!("{}", result);

    assert_eq!(result, 31875000);
}
