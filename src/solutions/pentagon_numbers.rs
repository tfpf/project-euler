use crate::utils;

/// Find the smallest difference between two pentagonal numbers which is a
/// pentagonal number, while their sum is also a pentagonal number.
///
/// -> Smallest pentagonal difference.
fn minimum_pentagonal_difference() -> i64 {
    let pentagons = utils::Polygonal::new(5).take(3000).collect::<Vec<i64>>();
    for i in 1..pentagons.len() {
        for j in 0..i {
            let sum = pentagons[i] + pentagons[j];
            let difference = pentagons[i] - pentagons[j];
            if utils::Polygonal::invert(5, sum).is_some()
                && utils::Polygonal::invert(5, difference).is_some()
            {
                return difference;
            }
        }
    }
    unreachable!();
}

pub fn solve() -> i64 {
    let result = minimum_pentagonal_difference();

    assert_eq!(result, 5482660);
    result
}
