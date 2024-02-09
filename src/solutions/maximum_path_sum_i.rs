use std::io::BufRead;

struct CacheMap {
    map: std::collections::HashMap<(usize, usize), i32>,
    triangle: Vec<Vec<i32>>,
}
impl CacheMap {
    fn new(triangle: Vec<Vec<i32>>) -> CacheMap {
        CacheMap {
            map: std::collections::HashMap::new(),
            triangle,
        }
    }
    /// Find the maximum path sum to the bottom, starting from the given row
    /// and column indices.
    ///
    /// * `key` - Tuple of row and column indices.
    ///
    /// -> Maximum path sum.
    fn get(&mut self, key: (usize, usize)) -> i32 {
        let (row, col) = key;
        if row >= self.triangle.len() {
            return 0;
        }
        match self.map.get(&key) {
            Some(&value) => value,
            None => {
                let left = self.get((row + 1, col));
                let right = self.get((row + 1, col + 1));
                let value = self.triangle[row][col] + std::cmp::max(left, right);
                self.map.insert(key, value);
                value
            }
        }
    }
}

pub fn solve() -> i64 {
    let fhandle = std::fs::File::open("res/maximum_path_sum_i.txt").unwrap();
    let reader = std::io::BufReader::new(fhandle);
    let triangle: Vec<Vec<i32>> = reader
        .lines()
        .map(|line| line.unwrap().split(' ').map(|s| s.parse().unwrap()).collect())
        .collect();

    let mut cache_map = CacheMap::new(triangle);
    let result = cache_map.get((0, 0));

    assert_eq!(result, 1074);
    result as i64
}
