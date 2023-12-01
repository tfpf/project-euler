use crate::utils;

pub fn solve() -> i64 {
    let mut arr = vec![false; 10000];
    let primes = utils::Primes::new(10000);
    for prime in primes.iter() {
        for square in utils::Polygonal::new(4) {
            let num = prime + 2 * square;
            if (num as usize) < arr.len() {
                arr[num as usize] = true;
            } else {
                break;
            }
        }
    }
    let result = (3..arr.len())
        .step_by(2)
        .filter(|&num| !utils::is_prime(num as i64) && !arr[num])
        .next()
        .unwrap();

    assert_eq!(result, 5777);
    result as i64
}
