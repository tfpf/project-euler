use crate::utils;

fn set_polygonal(is_polygonal: &mut Vec<u8>, sides: i64, num: i64) {
    is_polygonal[num as usize - 1000] |= 1 << (sides - 3);
}

pub fn solve() -> i64 {
    let mut is_polygonal = vec![0u8; 9000];
    for sides in 3..=8 {
        for num in utils::Polygonal::new(sides)
            .skip_while(|&num| num < 1000)
            .take_while(|&num| num < 10000)
        {
            set_polygonal(&mut is_polygonal, sides, num);
        }
    }

    0
}
