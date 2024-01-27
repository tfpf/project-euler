use crate::utils;

fn generate_cyclical(numbitfield: &Vec<(i64, u8)>, mask: u8, cyclical: &mut Vec<i64>) -> bool {
    if mask == 0 {
        return cyclical.first().unwrap() / 100 == cyclical.last().unwrap() % 100;
    }
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
    for &(num, bitfield) in numbitfield[begin_idx..]
        .iter()
        .take_while(|&&(num, _)| num < begin_val + difference)
    {
        cyclical.push(num);
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
    let mut numbitfield = std::collections::BTreeMap::new();
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
    let numbitfield = numbitfield.into_iter().collect::<Vec<(i64, u8)>>();
    println!("{:?}", numbitfield);

    let mut cyclical = vec![];
    generate_cyclical(&numbitfield, 63, &mut cyclical);
    println!("{:?}", cyclical);

    0
}
