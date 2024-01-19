use crate::utils;

pub fn solve() -> i64 {
    let primes = utils::SieveOfAtkin::new(9000).iter().collect::<Vec<i64>>();
    let mut p234sum_expressible = vec![0u64; 781250];
    for a in 0..primes.len() {
        let num = primes[a].pow(2);
        for b in 0..primes.len() {
            let num = num + primes[b].pow(3);
            if num as usize / 64 > p234sum_expressible.len() {
                break;
            }
            for c in 0..primes.len() {
                let num = num + primes[c].pow(4);
                let num = num as usize;
                if num / 64 > p234sum_expressible.len() {
                    break;
                }
                p234sum_expressible[num / 64] |= 1 << (num % 64);
            }
        }
    }
    let result = p234sum_expressible
        .iter()
        .map(|bitfield| bitfield.count_ones())
        .sum::<u32>();

    assert_eq!(result, 1097343);
    result as i64
}
