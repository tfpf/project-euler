use crate::utils;

pub fn solve() -> i64 {
    let words = std::fs::read_to_string("res/coded_triangle_numbers.txt").unwrap();
    let result = words
        .split(",")
        .map(|s| &s[1..s.len() - 1])
        .filter(|word| {
            let value = word.bytes().map(|b| b as i64 - 64).sum::<i64>();
            utils::Polygonal::invert(3, value) != None
        })
        .count();

    assert_eq!(result, 162);
    result as i64
}
