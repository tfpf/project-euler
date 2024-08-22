/// Bits iterator. Generates the binary digits of a number from least
/// significant to most significant. Positive numbers only!
pub struct Bits {
    num: i64,
}

impl Bits {
    pub fn new(num: i64) -> Bits {
        Bits { num }
    }
}

impl Iterator for Bits {
    type Item = i64;
    fn next(&mut self) -> Option<i64> {
        if self.num == 0 {
            None
        } else {
            let bit = self.num & 1;
            self.num >>= 1;
            Some(bit)
        }
    }
}
