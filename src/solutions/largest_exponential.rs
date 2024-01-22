use std::io::BufRead;

pub fn solve() -> i64 {
    let fhandle = std::fs::File::open("res/largest_exponential.txt").unwrap();
    let reader = std::io::BufReader::new(fhandle);
    let result = reader
        .lines()
        .map(|line| {
            let [base, exp] = line
                .unwrap()
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect::<Vec<f64>>()
                .try_into()
                .unwrap();
            exp * base.log2()
        })
        .zip(1..)
        // Safe to compare because there are no NaNs.
        .max_by(|(a, _), (b, _)| a.total_cmp(b))
        .unwrap()
        .1;

    assert_eq!(result, 709);
    result as i64
}
