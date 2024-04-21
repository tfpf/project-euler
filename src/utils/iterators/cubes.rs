#![allow(clippy::new_without_default)]

/// Cubes iterator. Generates cubes of integers without multiplication or
/// exponentiation.
pub struct Cubes {
    increment: i64,
    offset: i64,
    num: i64,
}
impl Cubes {
    pub fn new() -> Cubes {
        Cubes {
            increment: 6,
            offset: 1,
            num: 0,
        }
    }
}
impl Iterator for Cubes {
    type Item = i64;
    fn next(&mut self) -> Option<i64> {
        self.num += self.offset;
        self.offset += self.increment;
        self.increment += 6;
        Some(self.num)
    }
}

