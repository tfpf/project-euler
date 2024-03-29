use crate::utils;

pub fn solve() -> i64 {
    let result = (1..100)
        .flat_map(|a| {
            // Start from raising numbers to 2, because, by simple observation,
            // the maximum sum can never be obtained by raising numbers to 1.
            let mut num = utils::Long::from(a);
            (2..100).map(move |_| {
                num *= a;
                num.sum()
            })
        })
        .max()
        .unwrap();

    assert_eq!(result, 972);
    result
}
