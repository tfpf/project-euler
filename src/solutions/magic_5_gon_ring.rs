use crate::utils;

pub fn solve() -> i64 {
    // Assign an index to each of the 10 nodes of the 5-gon ring. Write the
    // indices of each triplet of nodes one after the other. In particular, if
    // the indices of one triplet of nodes are 0, 1 and 2, those of the next
    // node (moving clockwise) will be 3, 2 and 4. Effectively, this maps a
    // permutation to a potential solution.
    const INDICES: [usize; 15] = [0, 1, 2, 3, 2, 4, 5, 4, 6, 7, 6, 8, 9, 8, 1];

    // These will be written into the nodes. Starting with this order ensures
    // that the first solution we find is the largest one.
    let mut numbers = [7, 1, 2, 3, 4, 5, 6, 8, 9, 10];

    let mut result = 0;

    // Skipping the first permutation is safe because it cannot be the answer.
    // If any permutation is a solution, it must begin with at most 6, because
    // the first external node must be the smallest of all external nodes.
    'numbers: while utils::prev_permutation(&mut numbers) {
        let sum = numbers.iter().take(3).sum();
        for i in (0..INDICES.len()).step_by(3).skip(1) {
            // The first external node must be the smallest of all external
            // nodes.
            if numbers[INDICES[i]] < numbers[0] {
                continue 'numbers;
            }
            // The sum of the nodes in each triplet must be the same.
            if numbers[INDICES[i]] + numbers[INDICES[i + 1]] + numbers[INDICES[i + 2]] != sum {
                continue 'numbers;
            }
        }

        let solution = INDICES.iter().fold(0, |solution, &index| {
            let multiplier = if numbers[index] == 10 { 100 } else { 10 };
            solution * multiplier + numbers[index]
        });
        if solution < 10i64.pow(16) {
            result = solution;
            break;
        }
    }

    assert_eq!(result, 6531031914842725);
    result
}
