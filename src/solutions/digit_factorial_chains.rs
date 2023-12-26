use crate::utils;

pub fn solve() -> i64 {
    const FACTORIAL: [i64; 10] = [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];
    let count = (1..1000000)
        .filter(|&num| {
            let mut num = num;
            let mut nums = std::collections::HashSet::new();
            for idx in 1..=60 {
                nums.insert(num);
                num = utils::Digits::new(num)
                    .map(|digit| FACTORIAL[digit as usize])
                    .sum();
                if nums.contains(&num) {
                    return idx == 60;
                }
            }
            false
        })
        .count();

    assert_eq!(count, 402);
    count as i64
}
