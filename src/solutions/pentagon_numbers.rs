/// Find the smallest difference between two pentagonal numbers which is a
/// pentagonal number, while their sum is also a pentagonal number.
///
/// -> Smallest pentagonal difference.
fn minimum_pentagonal_difference() -> i32 {
    let pentagons = (1..3000)
        .map(|num| num * (3 * num - 1) / 2)
        .collect::<Vec<i32>>();
    for i in 1..pentagons.len() {
        for j in 0..i {
            let sum = pentagons[i] + pentagons[j];
            let difference = pentagons[i] - pentagons[j];
            let Ok(_) = pentagons.binary_search(&sum) else {
                continue;
            };
            let Ok(_) = pentagons.binary_search(&difference) else {
                continue;
            };
            return difference;
        }
    }
    unreachable!();
}

pub fn solve() -> i64 {
    let result = minimum_pentagonal_difference();

    assert_eq!(result, 5482660);
    result as i64
}
