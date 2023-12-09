use crate::utils;

pub fn solve() -> i64 {
    let result = (0..10000)
        .filter(|&num| {
            let mut num = utils::Long::from(num);
            num += &num.reverse();
            for _ in 1..50 {
                let num_rev = num.reverse();
                if num == num_rev {
                    return false;
                }
                num += &num_rev;
            }
            true
        })
        .count();

    assert_eq!(result, 249);
    result as i64
}
