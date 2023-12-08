use crate::utils;

pub fn solve() -> i64 {
    let result = (1..)
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
