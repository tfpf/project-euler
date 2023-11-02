/// Implement the `d` function.
///
/// * `num`
///
/// -> Sum of proper divisors of `num`.
fn sum_of_proper_divisors(num: usize) -> usize {
    if num == 0 || num == 1 {
        return 0;
    }

    // By definition, a number is not its own proper divisor. Hence, exclude 1
    // (which automatically excludes the number) from the list, then add it
    // back later.
    (2usize..)
        .take_while(|candidate| candidate.pow(2) <= num)
        .map(|candidate| {
            if num % candidate == 0 {
                if num / candidate != candidate {
                    candidate + num / candidate
                } else {
                    candidate
                }
            } else {
                0
            }
        })
        .sum::<usize>()
        + 1
}

pub fn solve() {
    let mut amicable = [false; 10000];
    for i in 0..10000 {
        if amicable[i] {
            continue;
        }
        let d_i = sum_of_proper_divisors(i);
        if d_i != i && sum_of_proper_divisors(d_i) == i {
            amicable[i] = true;
            amicable[d_i] = true;
        }
    }
    let result: usize = amicable
        .iter()
        .enumerate()
        .filter(|(_, is_amicable)| **is_amicable)
        .map(|(idx, _)| idx)
        .sum();

    println!("{}", result);
    assert_eq!(result, 31626);
}
