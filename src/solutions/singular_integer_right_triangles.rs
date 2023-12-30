use crate::utils;

pub fn solve() -> i64 {
    let mut triangles = [0; 1500001];
    const PERIMETER_MAX: usize = 1500000;
    const SEMIPERIMETER_MAX: usize = PERIMETER_MAX / 2;
    for m in 2..=(SEMIPERIMETER_MAX as f64).sqrt() as usize {
        for n in (1 + m % 2..m).step_by(2) {
            if utils::gcd(m as i64, n as i64) != 1 {
                continue;
            }
            let perimeter = 2 * m * (m + n);
            for p in (perimeter..).step_by(perimeter).take_while(|&p| p <= PERIMETER_MAX) {
                triangles[p] += 1;
            }
        }
    }
    let result = triangles.iter().filter(|&&t| t == 1).count();

    assert_eq!(result, 161667);
    result as i64
}
