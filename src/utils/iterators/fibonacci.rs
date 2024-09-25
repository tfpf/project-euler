/// Fibonacci sequence iterator.
pub struct Fibonacci {
    a: i64,
    b: i64,
}

impl Fibonacci {
    pub fn new(a: i64, b: i64) -> Fibonacci {
        Fibonacci { a, b }
    }
}

impl Iterator for Fibonacci {
    type Item = i64;
    fn next(&mut self) -> Option<i64> {
        let a = self.a;
        (self.a, self.b) = (self.b, self.a + self.b);
        Some(a)
    }
}
