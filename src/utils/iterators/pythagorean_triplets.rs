use crate::utils;

/// Pythagorean triplets iterator. Generates all Pythagorean triplets with the
/// given sum. The triplets are generated using a well-known parametrisation
/// which can represent any primitive triplet. Positive numbers only!
pub struct PythagoreanTriplets {
    semiperimeter: i64,
    m_ub: i64,
    m: i64,
}

impl PythagoreanTriplets {
    pub fn new(perimeter: i64) -> PythagoreanTriplets {
        let semiperimeter = perimeter / 2;
        // Non-strict upper bound for the parameter `m`.
        let m_ub = utils::isqrt(semiperimeter);
        PythagoreanTriplets {
            semiperimeter,
            m_ub,
            m: 1,
        }
    }
}

impl Iterator for PythagoreanTriplets {
    type Item = (i64, i64, i64);
    fn next(&mut self) -> Option<(i64, i64, i64)> {
        loop {
            self.m += 1;
            if self.m > self.m_ub {
                return None;
            }
            if self.semiperimeter % self.m != 0 {
                continue;
            }
            let m = self.m;
            let remaining_term = self.semiperimeter / m;
            let remaining_odd = remaining_term >> remaining_term.trailing_zeros();
            let m_plus_n_lb = m + 1 + m % 2;
            for m_plus_n in (m_plus_n_lb..)
                .step_by(2)
                .take_while(|&m_plus_n| m_plus_n < 2 * m && m_plus_n <= remaining_odd)
            {
                if remaining_odd % m_plus_n == 0 && utils::gcd(m_plus_n, m) == 1 {
                    let d = remaining_term / m_plus_n;
                    let n = m_plus_n - m;
                    let a = (m.pow(2) - n.pow(2)) * d;
                    let b = 2 * m * n * d;
                    let c = (m.pow(2) + n.pow(2)) * d;
                    return Some((a, b, c));
                }
            }
        }
    }
}
