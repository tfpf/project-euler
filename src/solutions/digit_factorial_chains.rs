use crate::utils;

pub fn solve() -> i64 {
    let mut chain_len = [0; 1000000];
    const FACTORIAL: [usize; 10] = [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];
    let count = (1..1000000)
        .filter(|&num| {
            // Already calculated earlier?
            if chain_len[num] != 0 {
                return chain_len[num] == 60;
            }

            let mut curr = num;
            let (mut nums_set, mut nums_vec) = (std::collections::HashSet::new(), vec![curr]);
            for idx in 1..=60 {
                nums_set.insert(curr);
                nums_vec.push(curr);
                curr = utils::Digits::new(curr as i64)
                    .map(|digit| FACTORIAL[digit as usize])
                    .sum();

                // Reached a number from a previous chain?
                if curr < chain_len.len() && chain_len[curr] != 0 {
                    for (&n, length) in nums_vec.iter().rev().zip((0..chain_len[curr]).rev()) {
                        if n < chain_len.len() {
                            chain_len[n] = length;
                        }
                    }
                    break;
                }

                // Reached a number from the current chain?
                if nums_set.contains(&curr) {
                    for (&n, length) in nums_vec.iter().zip((0..=idx).rev()) {
                        if n < chain_len.len() {
                            chain_len[n] = length;
                        }
                        if n == curr {
                            break;
                        }
                    }
                    for &n in nums_vec.iter().rev().take_while(|&&n| n != curr) {
                        if n < chain_len.len() {
                            chain_len[n] = chain_len[curr];
                        }
                    }
                    break;
                }
            }
            chain_len[num] == 60
        })
        .count();

    assert_eq!(count, 402);
    count as i64
}
