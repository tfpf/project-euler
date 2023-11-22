use crate::utils;

/// Build double-base palindromes (having at least two bits).
///
/// * `num` - Number to use to generate palindromes.
///
/// -> Sum of all double-base palindromes with binary `num` as their left half.
fn double_base_palindrome_sum(num: i64) -> i64 {
    // There are three ways to make a palindrome using this number. Having
    // reversed its bits, one can: stick the reversed bits to its right, append
    // 0 and stick the reversed bits to its right, or append 1 and stick the
    // reversed bits to the right.
    let num_shift_0 = num << 1;
    let num_shift_1 = num_shift_0 | 1;
    let palindromes =
        utils::Bits::new(num).fold([num, num_shift_0, num_shift_1], |accumulator, bit| {
            [
                accumulator[0] << 1 | bit,
                accumulator[1] << 1 | bit,
                accumulator[2] << 1 | bit,
            ]
        });
    let palindromes = palindromes
        .into_iter()
        .filter(|&palindrome| palindrome < 1000000)
        .collect::<Vec<i64>>();
    if palindromes.is_empty() {
        return 0;
    }

    // Are they palindromes in base 10 as well?
    let sum: i64 = palindromes
        .iter()
        .filter(|&&palindrome| utils::is_palindrome(palindrome, 10))
        .sum();

    // Continue searching with the augmented numbers generated above.
    sum + double_base_palindrome_sum(num_shift_0) + double_base_palindrome_sum(num_shift_1)
}

pub fn solve() -> i64 {
    // The function does not generate 1, which is also a double-base
    // palindrome. Hence, add it separately.
    let sum = 1 + double_base_palindrome_sum(1);

    assert_eq!(sum, 872187);
    sum
}
