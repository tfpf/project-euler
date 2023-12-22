use crate::utils;

pub fn solve() -> i64 {
    // Keys are the digits forming cubes. The value corresponding to each key
    // is a pair of the smallest cube made of those digits and the number of
    // cubes made of those digits.
    let mut frequency = std::collections::HashMap::<i64, (i64, u8)>::new();

    let result = utils::Cubes::new()
        .filter_map(|cube| {
            let mut digits = utils::Digits::new(cube).collect::<Vec<i64>>();
            digits.sort();
            let key = digits
                .iter()
                // Reversing ensures that there are no leading zeros, thereby
                // preserving the number of digits.
                .rev()
                .fold(0, |key, digit| key * 10 + digit);

            let cube_and_count = match frequency.get_mut(&key) {
                Some(value) => {
                    value.1 += 1;
                    (value.0, value.1)
                }
                None => {
                    frequency.insert(key, (cube, 1));
                    (cube, 1)
                }
            };

            if cube_and_count.1 < 5 {
                None
            } else {
                // Technically, I should check that the count does not become
                // 6 (some larger cube might have the same digits). Luckily, it
                // turns out that it doesn't, so I don't bother checking.
                Some(cube_and_count.0)
            }
        })
        .next()
        .unwrap();

    assert_eq!(result, 127035954683);
    result
}
