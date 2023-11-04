use crate::utils;

pub fn solve() {
    // It is enough to check only prime numbers. (The recurrence length for a
    // composite number is equal to the greatest recurrence length of its prime
    // factors.) Exclude 2 and 5, because they can divide any number without
    // decimal recurrence. Exclude 3 because it is definitely not the answer.
    let result = utils::primes(1000)
        .skip_while(|prime| *prime <= 5)
        .map(|prime| (utils::recurrence_length(prime as i64), prime))
        .max()
        .unwrap()
        .1;

    println!("{}", result);
    assert_eq!(result, 983);
}
