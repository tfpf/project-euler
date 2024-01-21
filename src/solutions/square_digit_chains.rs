use crate::utils;

pub fn solve() -> i64 {
    // The greatest possible sum of digit squares in this problem is 567. Find
    // out how many numbers from 1 to 567 get stuck at 89.
    let mut chain_stuck = vec![0u8; 568];
    chain_stuck[1] = 1;
    chain_stuck[89] = 89;
    let difficult = (1..568)
        .filter(|&num| {
            let mut chain = vec![];
            let mut num = num;
            while chain_stuck[num] == 0 {
                chain.push(num);
                num = utils::Digits::new(num as i64)
                    .map(|digit| digit.pow(2))
                    .sum::<i64>() as usize;
            }
            for chain_node in chain {
                chain_stuck[chain_node] = chain_stuck[num];
            }
            chain_stuck[num] == 89
        })
        .count();

    // Do the same for the rest of the numbers. This is simpler, because we can
    // just check where their sum of digit squares gets stuck, since it was
    // calculated above.
    let easy = (568..10000000)
        .filter(|&num| {
            let num = utils::Digits::new(num as i64)
                .map(|digit| digit.pow(2))
                .sum::<i64>() as usize;
            chain_stuck[num] == 89
        })
        .count();
    let result = difficult + easy;

    assert_eq!(result, 8581146);
    result as i64
}
