use crate::utils;

/// Compute the number of ways to obtain each number up to the given number by
/// adding positive integers. Essentially, implement the partition function (of
/// number theory) for each number up to the given number. The values shall be
/// modulo 1000000.
///
/// * `num` - Last number.
///
/// -> Array of the numbers of ways to to sum to the array index.
pub fn partitions(num: usize) -> Vec<i32> {
    let mut p = vec![0; num + 1];
    p[0] = 1;
    p[1] = 1;
    for idx in 2..=num {
        // We have to iterate over generalised pentagon numbers to evaluate the
        // partition function at any value. Generate them using pentagon
        // numbers.
        for (i, subtrahend) in utils::Polygonal::new(5)
            .zip(1..)
            .flat_map(|(polygonal, offset)| [polygonal, polygonal + offset].into_iter())
            .take_while(|&subtrahend| idx >= subtrahend as usize)
            .enumerate()
        {
            let rec = p[idx - subtrahend as usize];
            if i % 4 < 2 {
                p[idx] += rec;
            } else {
                p[idx] -= rec;
            }
            p[idx] %= 1000000;
        }
    }
    p
}

pub fn solve() -> i64 {
    let result = partitions(100000)
        .iter()
        .enumerate()
        .find(|(_, &count)| count == 0)
        .unwrap()
        .0;

    assert_eq!(result, 55374);
    result as i64
}
