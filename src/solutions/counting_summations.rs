struct CacheMap {
    map: std::collections::HashMap<(usize, i32), i32>,
    numbers: Vec<i32>,
}
impl CacheMap {
    fn new(target: i32) -> CacheMap {
        CacheMap {
            map: std::collections::HashMap::new(),
            numbers: (1..target).collect(),
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
        if remaining < 0 || idx >= self.numbers.len() {
            return 0;
        }
        if remaining == 0 {
            return 1;
        }
        match self.map.get(&key) {
            Some(&value) => value,
            None => {
                let with = self.get((idx, remaining - self.numbers[idx]));
                let without = self.get((idx + 1, remaining));
                let value = with + without;
                self.map.insert(key, value);
                value
            }
        }
    }
}

pub fn solve() -> i64 {
    let mut cache_map = CacheMap::new(100);
    let result = cache_map.get((0, 100));

    assert_eq!(result, 190569291);
    result as i64
}
