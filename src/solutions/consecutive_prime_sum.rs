use crate::utils;

pub fn solve() -> i64 {
    let primes = utils::Primes::new(1000000).iter().skip_while(|&prime| prime < 1000).collect::<Vec<i64>>();

    for i in (2..primes.len()).rev() {
        let mut window = (i - 2, i - 1);
        let mut sum = primes[window.0] + primes[window.1];
        while window.0 >= 1 {
            if sum == primes[i] {
                window.0 -= 1;
                sum += primes[window.0] - primes[window.1];
                window.1 -= 1;
            }
            else if sum > primes[i] {
                sum -= primes[window.1];
                window.1 -= 1;
            }
            else {
                window.0 -= 1;
                sum += primes[window.0];
            }
        }
    }












    0
}
