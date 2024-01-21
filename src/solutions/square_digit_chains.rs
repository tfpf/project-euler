use crate::utils;

pub fn solve() -> i64 {
    let mut chain_stuck = vec![0u8; 10000000];
    chain_stuck[1] = 1;
    chain_stuck[89] = 89;

    let result = (1..chain_stuck.len()).filter(|&num| {
        let mut chain = vec![];
        let mut num = num;
        while chain_stuck[num] == 0 {
            chain.push(num);
            num = utils::Digits::new(num as i64).map(|digit| digit.pow(2)).sum::<i64>() as usize;
        }
        for chain_node in chain {
            chain_stuck[chain_node] = chain_stuck[num];
        }
        chain_stuck[num] == 89
    }).count();

    assert_eq!(result, 8581146);
    result as i64
}
