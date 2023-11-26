use crate::utils;

pub fn solve() -> i64 {
    // Collect enough triangle numbers to dwarf the greatest word value.
    let triangle_numbers =
        std::collections::HashSet::<i64>::from_iter(utils::Triangular::new().take(20));

    let words = std::fs::read_to_string("res/coded_triangle_numbers.txt").unwrap();
    let words: Vec<&str> = words.split(",").map(|s| &s[1..s.len() - 1]).collect();
    let result = words
        .iter()
        .filter(|word| {
            let value = word.bytes().map(|b| b as i64 - 64).sum::<i64>();
            triangle_numbers.contains(&value)
        })
        .count();

    assert_eq!(result, 162);
    result as i64
}
