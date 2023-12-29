use crate::utils;

pub fn solve() -> i64 {
    // The number must have at least three digits (else, there are fewer than
    // six permutations possible). Its most significant digit must be 1 (else,
    // multiplying it by 6 would increase the number of digits). (In fact, the
    // upper bound is the number starting with 1 and containing 6s, for the
    // same reason.) Actually, someone who has played around with a calculator
    // in their youth might know the answer by heart!
    let result = (100..166)
        .chain(1000..1666)
        .chain(10000..16666)
        .chain(100000..166666)
        .find(|&num| {
            let expected = utils::digits_frequencies(num);
            expected == utils::digits_frequencies(2 * num)
                && expected == utils::digits_frequencies(3 * num)
                && expected == utils::digits_frequencies(4 * num)
                && expected == utils::digits_frequencies(5 * num)
                && expected == utils::digits_frequencies(6 * num)
        })
        .unwrap();

    assert_eq!(result, 142857);
    result
}
