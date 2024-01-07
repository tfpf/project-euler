use crate::utils;

pub fn solve() -> i64 {
    let mut triangles = vec![0u8; 1500001];
    const PERIMETER_MAX: usize = 1500000;
    const SEMIPERIMETER_MAX: usize = PERIMETER_MAX / 2;

    // Find all primitive triplets with sum less than or equal to the maximum.
    for m in 2..=utils::isqrt(SEMIPERIMETER_MAX as i64) as usize {
        for n in (1 + m % 2..m)
            .step_by(2)
            .filter(|&n| utils::gcd(m as i64, n as i64) == 1)
        {
            let perimeter = 2 * m * (m + n);
            // Increment the counter for it and its multiples.
            for p in (perimeter..)
                .step_by(perimeter)
                .take_while(|&p| p <= PERIMETER_MAX)
            {
                triangles[p] += 1;
            }
        }
    }
    let result = triangles.iter().filter(|&&elem| elem == 1).count();

    assert_eq!(result, 161667);
    result as i64
}
