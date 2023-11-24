/// Find the length of the Collatz sequence starting from the given number
/// and ending at 1.
///
/// * `collatz_lengths` - Array to cache the lengths.
/// * `num` - Number to find the length of the Collatz sequence for.
///
/// -> Length of Collatz sequence.
fn get_collatz_length(collatz_lengths: &mut Vec<i32>, num: usize) -> i32 {
    if num == 1 {
        return 1;
    }
    if num < collatz_lengths.len() && collatz_lengths[num] != 0 {
        return collatz_lengths[num];
    }
    let num_next = if num & 1 == 0 { num >> 1 } else { 3 * num + 1 };
    let len = 1 + get_collatz_length(collatz_lengths, num_next);
    if num < collatz_lengths.len() {
        collatz_lengths[num] = len;
    }
    len
}

pub fn solve() -> i64 {
    let mut collatz_lengths = vec![0; 1000000];
    let result = (2..collatz_lengths.len())
        .map(|num| (get_collatz_length(&mut collatz_lengths, num), num))
        .max()
        .unwrap()
        .1;

    assert_eq!(result, 837799);
    result as i64
}
