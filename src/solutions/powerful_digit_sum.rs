use crate::utils;

pub fn solve() -> i64 {
    let result: i32 = (1..100)
        .flat_map(|a| {
            let mut num = utils::Long::from(1);
            (1..100).map(move |_| {
                num *= a;
                num.to_string()
                    .bytes()
                    .map(|byte| (byte - b'0') as i32)
                    .sum()
            })
        })
        .max()
        .unwrap();

    assert_eq!(result, 972);
    result as i64
}
