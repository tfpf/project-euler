use crate::utils;

/// Produce 6 cyclical figurate numbers, each of a different family. (Family
/// means the figure type: triangle, quadrilateral, pentagon, hexagon, heptagon
/// and octagon.)
///
/// * `numbitfield` - `(num, bitfield)` indicating the family `num` belongs to.
/// * `mask` - Bitfield indicating the families we have not yet produced.
/// * `cyclical` - Cyclical figurate numbers we have produced so far.
///
/// -> Whether the 6 numbers were produced.
fn generate_cyclical(numbitfield: &Vec<(i64, u8)>, mask: u8, cyclical: &mut Vec<i64>) -> bool {
    // If numbers of all families have been produced, check the first and last
    // numbers for the cyclic property.
    if mask == 0 {
        return cyclical.first().unwrap() / 100 == cyclical.last().unwrap() % 100;
    }

    // If no numbers have been produced yet, start with each number in turn.
    // Otherwise, start with the number whose two most significant digits match
    // the two least significant digits of the last produced number.
    let (begin_val, begin_idx, difference) = if cyclical.is_empty() {
        (0, 0, i64::MAX)
    } else {
        let begin_val = cyclical.last().unwrap() % 100 * 100;
        let begin_idx = match numbitfield.binary_search(&(begin_val, 0)) {
            Ok(begin_idx) => begin_idx,
            Err(begin_idx) => begin_idx,
        };
        (begin_val, begin_idx, 100)
    };

    // Try to use each of the earmarked numbers as the next number.
    for &(num, bitfield) in numbitfield[begin_idx..]
        .iter()
        .take_while(|&&(num, _)| num < begin_val + difference)
    {
        cyclical.push(num);
        // This number we produced may belong to multiple families. For each
        // family, mark that family as done and produce the next number.
        for shift in 0..6 {
            if bitfield >> shift & 1 == 1
                && mask >> shift & 1 == 1
                && generate_cyclical(numbitfield, mask & !(1 << shift), cyclical)
            {
                return true;
            }
        }
        cyclical.pop();
    }
    false
}

pub fn solve() -> i64 {
    // Indicate whether each number is a triangle, quadrilateral, pentagon,
    // hexagon, heptagon or octagon number using bits 0 through 5
    // (respectively) of an 8-bitfield.
    let mut numbitfield = std::collections::BTreeMap::<i64, u8>::new();
    for sides in 3..=8 {
        for num in utils::Polygonal::new(sides)
            .skip_while(|&num| num < 1000)
            .take_while(|&num| num < 10000)
        {
            let curr_mask = 1u8 << (sides - 3);
            let prev_mask = match numbitfield.get(&num) {
                Some(&bitfield) => bitfield,
                None => 0,
            };
            numbitfield.insert(num, prev_mask | curr_mask);
        }
    }

    // Convert it into a vector, which can be binary-searched to obtain an
    // index into it.
    let numbitfield = numbitfield.into_iter().collect::<Vec<(i64, u8)>>();

    let mut cyclical = vec![];
    generate_cyclical(&numbitfield, 63, &mut cyclical);
    let result = cyclical.iter().sum();

    assert_eq!(result, 28684);
    result
}
