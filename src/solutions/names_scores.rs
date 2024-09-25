pub fn solve() -> i64 {
    let names = std::fs::read_to_string("res/solutions/names_scores.txt").unwrap();
    let mut names: Vec<&str> = names.split(',').map(|s| &s[1..s.len() - 1]).collect();
    names.sort();
    let result: usize = names
        .iter()
        .enumerate()
        .map(|(index, name)| name.bytes().map(|b| b as usize - 64).sum::<usize>() * (index + 1))
        .sum();

    assert_eq!(result, 871198282);
    result as i64
}
