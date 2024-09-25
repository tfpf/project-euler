use std::io::BufRead;

pub fn solve() -> i64 {
    let fhandle = std::fs::File::open("res/solutions/path_sum_two_ways.txt").unwrap();
    let reader = std::io::BufReader::new(fhandle);
    let mut matrix: Vec<Vec<i32>> = reader
        .lines()
        .map(|line| line.unwrap().split(',').map(|s| s.parse().unwrap()).collect())
        .collect();

    // Bottom-up dynamic programming.
    for row in (0..80).rev().skip(1) {
        matrix[row][79] += matrix[row + 1][79];
    }
    for col in (0..80).rev().skip(1) {
        matrix[79][col] += matrix[79][col + 1];
    }
    for row in (0..80).rev().skip(1) {
        for col in (0..80).rev().skip(1) {
            matrix[row][col] += std::cmp::min(matrix[row + 1][col], matrix[row][col + 1]);
        }
    }
    let result = matrix[0][0];

    assert_eq!(result, 427337);
    result as i64
}
