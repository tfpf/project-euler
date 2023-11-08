use crate::utils;

/// Update the array with the occurrences of each digit of a number. If the
/// digit 0 is encountered or if any digit occurs more than once, quit.
///
/// * `seen` - Array indicating whether an index has already been encountered.
/// * `num` - Number whose digits to check.
///
/// -> `true` if all digits of the number were encountered for the first time
///     and it contained no zeros, else `false`.
fn update(seen: &mut [bool; 10], num: i64) -> bool {
    for digit in utils::Digits::new(num) {
        let d = digit as usize;
        if d == 0 || seen[d] {
            return false;
        }
        seen[d] = true;
    }
    true
}

/// Run a nested loop in the given range, looking for pandigital products.
///
/// * `outer_begin`
/// * `outer_end`
/// * `inner_end`
/// * `inner_end`
/// * `products` - Set the put the products in. (To avoid repetition.)
fn search(
    outer_begin: i64,
    outer_end: i64,
    inner_begin: i64,
    inner_end: i64,
    products: &mut std::collections::HashSet<i64>,
) {
    for i in outer_begin..outer_end {
        for j in inner_begin..inner_end {
            let product = i * j;
            let mut seen = [false; 10];
            if update(&mut seen, i)
                && update(&mut seen, j)
                && update(&mut seen, product)
                // Check whether each digit (other than 0) was encountered.
                && seen
                    .iter()
                    .skip(1)
                    .fold(true, |accumulator, &element| accumulator && element)
            {
                products.insert(product);
            }
        }
    }
}

pub fn solve() {
    // For the multiplicand, multiplier and product to have 9 digits in total,
    // we must multiply a 4-digit number with a 1-digit number or a 3-digit
    // number with a 2-digit number.
    let mut products = std::collections::HashSet::new();
    search(1000, 100000, 1, 10, &mut products);
    search(100, 1000, 10, 100, &mut products);
    let sum: i64 = products.iter().sum();

    println!("{}", sum);
    assert_eq!(sum, 45228);
}
