struct CacheMap {
    map: std::collections::HashMap<(usize, i32), i32>,
    numbers: Vec<i32>,
}
impl CacheMap {
    fn new(numbers: Vec<i32>) -> CacheMap {
        CacheMap {
            map: std::collections::HashMap::new(),
            numbers,
        }
    }
    /// Find the number of ways to obtain given sum using numbers at the given
    /// index or higher.
    ///
    /// * `key` - Tuple of index and sum.
    ///
    /// -> Number of ways.
    fn get(&mut self, key: (usize, i32)) -> i32 {
        let (idx, remaining) = key;
        if idx >= self.numbers.len() || remaining < 0 {
            return 0;
        }
        if remaining == 0 {
            return 1;
        }
        match self.map.get(&key) {
            Some(&value) => value,
            None => {
                let without = self.get((idx + 1, remaining));
                let with = self.get((idx, remaining - self.numbers[idx]));
                let value = without + with;
                self.map.insert(key, value);
                value
            }
        }
    }
}

pub fn solve() -> i64 {
    // Approach is identical to that used for P31.
    let numbers = (1..100).collect();
    let mut cache_map = CacheMap::new(numbers);
    let result = cache_map.get((0, 100));

    assert_eq!(result, 190569291);
    result as i64
}
