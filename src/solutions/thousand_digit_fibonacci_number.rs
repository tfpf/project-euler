use crate::utils;

pub fn solve() -> i64 {
    let mut a = utils::Long::from(0);
    let mut b = utils::Long::from(1);
    let mut index = 0;
    let result = loop {
        index += 1;
        (b, a) = (&a + &b, b);
        if a.len() == 1000 {
            break index;
        }
    };

    assert_eq!(result, 4782);
    result
}
