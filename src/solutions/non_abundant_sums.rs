use crate::utils;

pub fn solve() {
    let abundant_numbers = std::collections::BTreeSet::from_iter(
        (0..=28123).filter(|&num| utils::Divisors::new(num).sum::<i64>() > num + num),
    );
    let sum: i64 = (0..=28123)
        .filter(|num| {
            // Can this number be expressed as the sum of two abundant numbers?
            for abundant1 in abundant_numbers
                .iter()
                .take_while(|abundant1| *abundant1 <= num)
            {
                let abundant2 = num - abundant1;
                if abundant_numbers.contains(&abundant2) {
                    // It can be, so it must be filtered out.
                    return false;
                }
            }
            true
        })
        .sum();

    print!("{}", sum);
    assert_eq!(sum, 4179871);
}
