use crate::utils;

pub fn solve() -> i64 {
    // The number must have at least three digits (else, there are fewer than
    // six permutations possible). Its most significant digit must be 1 (else,
    // multiplying it by 6 would increase the number of digits). Actually,
    // someone who has played around with a calculator in their youth might
    // know the answer by heart!
    let result = (100..200).chain(1000..2000).chain(10000..20000).chain(100000..200000)
        .filter(|&num| {
            let expected = utils::digits_frequencies(num);
            expected == utils::digits_frequencies(2 * num)
                && expected == utils::digits_frequencies(3 * num)
                && expected == utils::digits_frequencies(4 * num)
                && expected == utils::digits_frequencies(5 * num)
                && expected == utils::digits_frequencies(6 * num)
        })
        .next()
        .unwrap();

    assert_eq!(result, 142857);
    result
}
