use crate::utils;

pub fn solve() -> i64 {
    let result = (0..10000)
        .filter(|&num| {
            let mut num = utils::Long::from(num);
            let mut num_rev = num.reverse();
            for _ in 0..50 {
                num += &num_rev;
                num_rev = num.reverse();
                if num == num_rev {
                    return false;
                }
            }
            true
        })
        .count();

    assert_eq!(result, 249);
    result as i64
}
