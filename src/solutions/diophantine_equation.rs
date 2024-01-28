use crate::utils;

pub fn solve() -> i64 {
    let result = (2..=1000)
        .filter_map(|d| {
            // Exclude perfect squares.
            let a0 = utils::isqrt(d);
            if a0.pow(2) == d {
                return None;
            }

            // Solve the Pell equation using the continued fraction
            // representation of the constant term. This is a known technique.
            let a = utils::ContinuedFraction::new(d).collect::<Vec<i64>>();
            let r = a.len() - 2;
            let solution_idx = if r % 2 == 1 { r } else { 2 * r + 1 };
            // Must use 128-bit integers to avoid overflow. Could also have
            // used my arbitrary-precision integer type, but 128 bits turn out
            // to be enough.
            let (mut pcurr, mut pprev, mut pprevprev) = (0i128, 0i128, 0i128);
            let (mut qcurr, mut qprev, mut qprevprev) = (0i128, 0i128, 0i128);
            for idx in 0..=solution_idx {
                let idx = if idx >= a.len() { idx - a.len() + 1 } else { idx };
                let (pnext, qnext) = if idx == 0 {
                    (a[idx] as i128, 1)
                } else {
                    (a[idx] as i128 * pprev + pprevprev, a[idx] as i128 * qprev + qprevprev)
                };
                (pcurr, pprev, pprevprev) = (pnext, pcurr, pprev);
                (qcurr, qprev, qprevprev) = (qnext, qcurr, qprev);
            }
            Some((pcurr, d))
        })
        .max()
        .unwrap()
        .1;

    result
}
