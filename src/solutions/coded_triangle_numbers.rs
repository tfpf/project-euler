use crate::utils;

pub fn solve() -> i64 {
    let words = std::fs::read_to_string("res/coded_triangle_numbers.txt").unwrap();
    let result = words
        .split(',')
        .filter_map(|s| {
            let word = &s[1..s.len() - 1];
            let value = word.bytes().map(|b| (b - b'A' + 1) as i64).sum::<i64>();
            utils::Polygonal::invert(3, value)
        })
        .count();

    assert_eq!(result, 162);
    result as i64
}
