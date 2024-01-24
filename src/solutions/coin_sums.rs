struct CacheMap {
    map: std::collections::HashMap<(usize, i32), i32>,
    denominations: [i32; 8],
}
impl CacheMap {
    fn new(denominations: [i32; 8]) -> CacheMap {
        CacheMap {
            map: std::collections::HashMap::new(),
            denominations,
        }
    }
    /// Count the number of ways to obtain the remaining amount of money using
    /// denominations at the given index and above.
    ///
    /// * `key` - Tuple of the index and remaining amount.
    ///
    /// -> Number of ways.
    fn get(&mut self, key: (usize, i32)) -> i32 {
        let (index, remaining) = key;
        if index >= 8 || remaining < 0 {
            return 0;
        }
        if remaining == 0 {
            return 1;
        }
        match self.map.get(&key) {
            Some(&value) => value,
            None => {
                let without = self.get((index + 1, remaining));
                let with = self.get((index, remaining - self.denominations[index]));
                let value = without + with;
                self.map.insert(key, value);
                value
            }
        }
    }
}

pub fn solve() -> i64 {
    let denominations = [1, 2, 5, 10, 20, 50, 100, 200];
    let mut cache_map = CacheMap::new(denominations);
    let result = cache_map.get((0, 200));

    assert_eq!(result, 73682);
    result as i64
}
