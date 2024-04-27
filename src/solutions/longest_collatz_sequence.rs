/// Find the length of the Collatz sequence starting from the given number
/// and ending at 1.
///
/// * `collatz_lengths` - Array to cache the lengths. Indexable with `num`.
/// * `num` - Number to find the length of the Collatz sequence for.
///
/// -> Length of Collatz sequence.
fn get_collatz_length(collatz_lengths: &mut Vec<i32>, num: usize) -> i32 {
    if collatz_lengths[num] != 0 {
        return collatz_lengths[num];
    }
    // In the course of Collatz iterations, a 32-bit number will overflow, so
    // temporarily use a 64-bit number.
    let mut num_next = num as u64;
    for count in 1.. {
        // Keep transforming this number until it can be used as an index into
        // the cache.
        num_next = if num_next % 2 == 0 {num_next / 2} else {3*num_next+1};
        if num_next >= collatz_lengths.len() as u64 {
            continue;
        }
        let num_next = num_next as usize;
        collatz_lengths[num] = count + get_collatz_length(collatz_lengths, num_next);
        return collatz_lengths[num];
    }
    unreachable!();
}

pub fn solve() -> i64 {
    let mut collatz_lengths = vec![0; 1000000];
    collatz_lengths[1] = 1;
    let result = (2..collatz_lengths.len())
        .map(|num| (get_collatz_length(&mut collatz_lengths, num), num))
        .max()
        .unwrap()
        .1;

    assert_eq!(result, 837799);
    result as i64
}
