use crate::utils;

/// Find out where we would get stuck if we started at each number from 1 to
/// 567.
///
/// The greatest possible sum of digit squares in this problem is 567. Hence,
/// if we know the answer for those numbers, we can know the answer for any
/// number by taking the sum of the squares of its digits.
///
/// -> Array showing where the chain gets stuck in a loop.
fn minimal_stuck() -> [u8; 568] {
    let mut chain_stuck = [0u8; 568];
    chain_stuck[1] = 1;
    chain_stuck[89] = 89;
    for num in 1..chain_stuck.len() {
        let mut chain = vec![];
        let mut num = num;
        while chain_stuck[num] == 0 {
            chain.push(num);
            num = utils::Digits::new(num as i64).map(|digit| digit.pow(2)).sum::<i64>() as usize;
        }
        for chain_node in chain {
            chain_stuck[chain_node] = chain_stuck[num];
        }
    }
    chain_stuck
}

/// Produce 7-digit numbers in ascending order. Maintain the digit square sum.
///
/// * `chain_stuck` Where the chain of each number till 567 gets stuck.
/// * `digits` Digits of the number, ordered from most to least significant.
/// * `sqsum` Sum of the squares of the digits.
///
/// -> How many with these digits have digit square sum chains ending at 89.
fn generate_ascending(chain_stuck: &[u8; 568], digits: &mut Vec<usize>, sqsum: usize) -> i32 {
    const FACTORIAL: [i32; 10] = [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];

    // Termination.
    if digits.len() == 7 {
        if chain_stuck[sqsum] != 89 {
            return 0;
        }
        // All numbers formed by permuting these digits will get stuck at 89.
        // How many permutations are possible?
        let mut frequencies = [0u8; 10];
        for digit in digits {
            frequencies[*digit] += 1;
        }
        let denominator: i32 = frequencies
            .iter()
            .map(|&frequency| FACTORIAL[frequency as usize])
            .product();
        let numerator = 5040;
        return numerator / denominator;
    }

    let last = match digits.iter().last() {
        Some(&last) => last,
        None => 0,
    };
    (last..=9)
        .map(|digit| {
            digits.push(digit);
            let count = generate_ascending(chain_stuck, digits, sqsum + digit.pow(2));
            digits.pop();
            count
        })
        .sum()
}

pub fn solve() -> i64 {
    let chain_stuck = minimal_stuck();
    let mut digits = vec![];
    let result = generate_ascending(&chain_stuck, &mut digits, 0);

    assert_eq!(result, 8581146);
    result as i64
}
