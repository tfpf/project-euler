use crate::utils;
use std::io::BufRead;

pub fn solve() -> i64 {
    let fhandle = std::fs::File::open("res/poker_hands.txt").unwrap();
    let reader = std::io::BufReader::new(fhandle);
    let result = reader
        .lines()
        .filter(|line| {
            let split = line.as_ref().unwrap().split(" ").collect::<Vec<&str>>();
            let p1_hand = utils::PokerHand::new(&split[..5]);
            let p2_hand = utils::PokerHand::new(&split[5..]);
            p1_hand > p2_hand
        })
        .count();

    assert_eq!(result, 376);
    result as i64
}
