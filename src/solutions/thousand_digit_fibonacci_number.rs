use crate::utils;

pub fn solve() {
    let mut a = utils::Long::new("0");
    let mut b = utils::Long::new("1");
    let mut index = 0;
    let result = loop {
        index += 1;
        (b, a) = (&a + &b, b);
        if a.len() == 1000 {
            break index;
        }
    };

    print!("{}", result);
    assert_eq!(result, 4782);
}
