pub fn solve() -> i64 {
    let (mut best_a, mut best_b, mut best_off) = (0, 0, i64::MAX);
    for a in (0..6000).rev() {
        for b in 0..6000 {
            let rectangles: i64 = a * (a + 1) * b * (b + 1) / 4;
            let off = (2000000 - rectangles).abs();
            if off < best_off {
                (best_a, best_b, best_off) = (a, b, off);
            }
        }
    }
    println!("{} {}", best_a, best_b);
    let result = best_a * best_b;

    result
}
