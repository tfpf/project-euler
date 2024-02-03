use crate::utils;

pub fn solve() -> i64 {
    for num in 2..10i64.pow(7) {
        let t = utils::totient(num);
        if utils::digits_frequencies(num) == utils::digits_frequencies(t) {
            println!("{} {}", num as f64 / t as f64, num);
        }
    }

    0
}
