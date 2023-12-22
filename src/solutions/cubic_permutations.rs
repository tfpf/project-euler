use crate::utils;

pub fn solve() -> i64 {
    // Keys are the digits forming cubes. The value corresponding to each key
    // is a pair of the smallest cube made of those digits and the number of
    // cubes made of those digits.
    let mut frequency = std::collections::BTreeMap::<i64, (i64, u8)>::new();

    for cube in utils::Cubes::new() {
        let mut digits = utils::Digits::new(cube).collect::<Vec<i64>>();
        digits.sort();
        let key = digits
            .iter()
            .rev()
            .fold(0, |key, digit| key * 10 + digit);

        match frequency.get_mut(&key) {
            Some(value) => {
                value.1 += 1;
            }
            None => {
                frequency.insert(key, (cube, 1));
            }
        }
        println!("{} {} {:?}", cube, key, frequency.get(&key).unwrap());
    }

    0
}
