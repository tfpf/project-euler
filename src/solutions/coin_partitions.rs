use crate::utils;

/// Find the smallest number for which the number of ways to sum to it using
/// positive integers is divisible by 1000000. Essentially, implement the
/// partition function (of number theory) for every number (storing values
/// modulo 1000000) until we find the answer.
pub fn coin_partitions() -> usize {
    let mut p = vec![0; 100000];
    p[0] = 1;
    p[1] = 1;
    for idx in 2..p.len() {
        // We have to iterate over generalised pentagon numbers to evaluate the
        // partition function at any value. Generate them using pentagon
        // numbers.
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
        // Technically, I should reduce the number modulo 1000000 in the
        // above loop, but it will not overflow beyond 32 bits, so I do it
        // here.
        p[idx] %= 1000000;
        if p[idx] == 0 {
            return idx;
        }
    }
    unreachable!();
}

pub fn solve() -> i64 {
    let result = coin_partitions();

    assert_eq!(result, 55374);
    result as i64
}
