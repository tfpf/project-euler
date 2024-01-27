use crate::utils;

// fn set_polygonal(is_polygonal: &mut [u8], sides: i64, num: i64) {
//     is_polygonal[num as usize - 1000] |= 1 << (sides - 3);
// }

fn generate_cyclical(is_polygonal: &[u8], mask: u8, cyclical: &mut Vec<i64>) {
    if mask == 0 {
        return cyclical.first().unwrap() / 100 == cyclical.last().unwrap() % 100;
    }

    let (begin, end) = if mask == 63 {
        (1000, 10000)
    } else {
        let begin = cyclical.last().unwrap() % 100 * 100;
        let end = std::cmp::min(begin + 100, 10000);
        (begin, end)
    };
    for num in (begin..end).filter(|num| get_polygonal) {
    }
}

pub fn solve() -> i64 {
    let mut is_polygonal = vec![0u8; 10000];
    for sides in 3..=8 {
        for num in utils::Polygonal::new(sides)
            .skip_while(|&num| num < 1000)
            .take_while(|&num| num < 10000)
        {
            is_polygonal[num as usize] |= 1 << (sides - 3);
            // Turn this into a hashmap? Iterate over it and keep clearing mask bits?
            // set_polygonal(&mut is_polygonal, sides, num);
        }
    }

    let cyclical = vec![];
    generate_cyclical(&is_polygonal, 63, &mut cyclical);

    0
}
