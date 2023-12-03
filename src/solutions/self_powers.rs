use crate::utils;

pub fn solve() -> i64 {
    let mut sum = utils::Long::from(0);
    for num in (1..1000).filter(|num| num % 10 != 0) {
        sum += &utils::Long::from(num).pow(num as u32);
    }
    let result = sum.to_string()[sum.len() - 10..].parse::<i64>().unwrap();

    assert_eq!(result, 9110846700);
    result
}
