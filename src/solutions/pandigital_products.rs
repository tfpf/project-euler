use crate::utils;

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
    let mut pandigital_checker = utils::PandigitalChecker::new(1, 9);
    for i in outer_begin..outer_end {
        for j in inner_begin..inner_end {
            let product = i * j;
            pandigital_checker.renew();
            if pandigital_checker.update(i)
                && pandigital_checker.update(j)
                && pandigital_checker.update(product)
                && pandigital_checker.check()
            {
                products.insert(product);
            }
        }
    }
}

pub fn solve() -> i64 {
    // For the multiplicand, multiplier and product to have 9 digits in total,
    // we must multiply a 4-digit number with a 1-digit number or a 3-digit
    // number with a 2-digit number. Any other combination, and we are
    // guaranteed to have more than or less than 9 digits.
    let mut products = std::collections::HashSet::new();
    search(1000, 10000, 1, 10, &mut products);
    search(100, 1000, 10, 100, &mut products);
    let sum: i64 = products.iter().sum();

    assert_eq!(sum, 45228);
    sum
}
