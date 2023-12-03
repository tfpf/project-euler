use crate::utils;

pub fn solve() -> i64 {
    let mut sum = utils::Long::new("0");
    for num in 1..1000 {
        let mut term = utils::Long::new("1");
        for _ in 0..num {
            term *= num;
        }
        sum += &term;
    }
    let result = sum.to_string()[sum.len() - 10..].parse::<i64>().unwrap();

    assert_eq!(result, 9110846700);
    result
}
