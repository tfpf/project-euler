use crate::utils;

pub fn solve() -> i64 {
    let mut result = 0;
    for a in 1..100 {
        let mut num = utils::Long::from(1);
        for b in 1..100 {
            num *= a;
            result = std::cmp::max(result, num.to_string().bytes().map(|byte| (byte - b'0') as i32).sum());
        }
    }

    assert_eq!(result, 972);
    result as i64
}
