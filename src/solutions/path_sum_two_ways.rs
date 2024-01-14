use std::io::BufRead;

pub fn solve() -> i64 {
    let fhandle = std::fs::File::open("res/path_sum_two_ways.txt").unwrap();
    let reader = std::io::BufReader::new(fhandle);
    let mut matrix: Vec<Vec<i32>> = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect();

    // Bottom-to-top dynamic programming.
    for row in (0..80).rev() {
        for col in (0..80).rev() {
            let down = if row + 1 < 80 {
                matrix[row + 1][col]
            } else {
                i32::MAX
            };
            let right = if col + 1 < 80 {
                matrix[row][col + 1]
            } else {
                i32::MAX
            };
            let shorter = std::cmp::min(down, right);
            if shorter != i32::MAX {
                matrix[row][col] += shorter;
            }
        }
    }
    let result = matrix[0][0];

    assert_eq!(result, 427337);
    result as i64
}
