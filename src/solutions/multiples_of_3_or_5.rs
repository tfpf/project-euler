use crate::utils;
pub fn solve() -> i64 {
// π(10i64.pow(3)) = 168
// π(10i64.pow(4)) = 1229
// π(10i64.pow(5)) = 9592
// π(10i64.pow(6)) = 78498
// π(10i64.pow(7)) = 664579
// π(10i64.pow(8)) = 5761455
// π(10i64.pow(9)) = 50847534
// π(2i64.pow(32)) = 203280221
// let v = (0..10i64.pow(6)).filter(|&num| utils::is_prime(num)).collect::<Vec<i64>>();
// println!("{:?}", v);
// for n in &v {println!("{}", n);}
// println!("{:?}", v.len());
println!("{}", utils::is_prime_bpsw(100003));
    let sum = (1..1000).filter(|num| num % 3 == 0 || num % 5 == 0).sum();

    assert_eq!(sum, 233168);
    sum
}
