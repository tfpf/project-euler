use crate::utils;

pub fn solve() -> i64 {
    for p in (12..=130).step_by(2) {
        println!("{} {:?}", p, utils::PythagoreanTriplets::new(p).collect::<Vec<(i64,i64,i64)>>());
    }
    let result = (12..=1000)
        .map(|p| (utils::PythagoreanTriplets::new(p).count(), p))
        .max()
        .unwrap()
        .1;
    result as i64
}
