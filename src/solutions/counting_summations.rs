use crate::utils;

pub fn solve() -> i64 {
    // Approach is similar to that used for P78. The difference is that the
    // exact values of the partition function are computed.
    let mut p = vec![0; 101];
    p[0] = 1;
    p[1] = 1;
    for idx in 2..p.len() {
        for (offset, pentagonal) in (1..).zip(utils::Polygonal::new(5)) {
            if idx < pentagonal as usize {
                break;
            }
            let recurrence_term = p[idx - pentagonal as usize]
                + if idx < pentagonal as usize + offset {
                    0
                } else {
                    p[idx - pentagonal as usize - offset]
                };
            if offset % 2 == 1 {
                p[idx] += recurrence_term;
            } else {
                p[idx] -= recurrence_term;
            }
        }
    }

    // Exclude the singleton partition, because we are asked for the number of
    // ways to sum to 100 using at least two numbers.
    let result = p[100] - 1;

    assert_eq!(result, 190569291);
    result as i64
}
