use crate::utils;

pub fn solve() -> i64 {
    let mut frequency = std::collections::BTreeMap::new();
    for cube in utils::Cubes::new() {
        let mut digits = utils::Digits::new(cube).collect::<Vec<i64>>();
        digits.sort();
        let key = digits
            .iter()
            .rev()
            .fold(0, |key, digit| key * 10 + digit);
        match frequency.get_mut(&key) {
            Some(count) => {
                *count += 1;
            }
            None => {
                frequency.insert(key, 1);
            }
        }
        println!("{} {} {}", cube, key, frequency.get(&key).unwrap());
    }

    0
}
