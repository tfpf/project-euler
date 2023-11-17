use crate::utils;

pub fn solve() {
    // Cache what we need.
    let factorial = [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];

    // An easy upper bound is 9999999, which exceeds the sum of the factorials
    // of its digits. However, a better upper bound is the ordinate of the
    // point at which the graphs of
    //     y = x (value of a number)
    // and
    //     y = 362880 × log(x, 10) (maximum sum of its digit factorials)
    // intersect.
    let sum: i64 = (10..2309171)
        .filter(|&num| {
            utils::Digits::new(num)
                .map(|digit| factorial[digit as usize])
                .sum::<i64>()
                == num
        })
        .sum();

    print!("{}", sum);
    assert_eq!(sum, 40730);
}
