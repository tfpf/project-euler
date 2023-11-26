use crate::utils;

pub fn solve() -> i64 {
    let mut pandigital_checker = utils::PandigitalChecker::new(9);

    // We have to find a concatenated product at least as much as 918273645
    // (which is provided as an example). Hence, the most significant digit of
    // the integer to search for must be 9. Further, to end up with 9 digits,
    // said integer must have 4 non-repeated digits. This means only two
    // consecutive products need be considered.
    let num_to_find = (9123..=9876)
        .rev()
        .filter(|num| {
            pandigital_checker.renew();
            pandigital_checker.update(num * 1)
                && pandigital_checker.update(num * 2)
                && pandigital_checker.check()
        })
        .next()
        .unwrap();

    // We have found the integer. Now find the pandigital concatenated product
    // by sticking twice its value to its right.
    let result = num_to_find * 100002;

    assert_eq!(result, 932718654);
    result
}
