pub fn solve() -> i64 {
    // Calculate Euler's totient of every number up to the limit.
    let mut totients = (0..=1000000).collect::<Vec<i64>>();
    for num in 2..totients.len() {
        if totients[num] == num as i64 {
            // This number is prime. The totient of each of its multiples will
            // contain the term
            //     1/(1 - num)
            // in its product expression. Instead of multiply-assigning that
            // term, we use an equivalent subtract-assign.
            for multiple in (num..totients.len()).step_by(num) {
                totients[multiple] -= totients[multiple] / num as i64;
            }
        }
    }

    // The number of reduced fractions with a particular denominator is the
    // totient of the denominator. Hence, the total number of fractions is the
    // sum.
    let result = totients[2..].iter().sum();

    assert_eq!(result, 303963552391);
    result
}
