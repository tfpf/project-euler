use std::io::BufRead;

pub fn solve() -> i64 {
    let fhandle = std::fs::File::open("res/maximum_path_sum_ii.txt").unwrap();
    let reader = std::io::BufReader::new(fhandle);
    let mut triangle: Vec<Vec<i32>> = reader
        .lines()
        .map(|line| line.unwrap().split(' ').map(|s| s.parse().unwrap()).collect())
        .collect();

    // Bottom-up dynamic programming.
    for row in (0..triangle.len()).rev().skip(1) {
        for col in 0..triangle[row].len() {
            triangle[row][col] += std::cmp::max(triangle[row + 1][col], triangle[row + 1][col + 1]);
        }
    }
    let result = triangle[0][0];

    assert_eq!(result, 7273);
    result as i64
}
