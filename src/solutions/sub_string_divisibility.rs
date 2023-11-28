/// A crude way to represent a set of single-digit numbers. Usage: when
/// removing an element, swap it with the last element and decrement the
/// length. When adding it back, do the reverse.
struct DigitsSet {
    digits: [i64; 10],
    len: usize,
}
impl DigitsSet {
    fn new() -> DigitsSet {
        DigitsSet {
            digits: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
            len: 10,
        }
    }
}

/// Build sub-string divisible numbers.
///
/// * `ds` - Set of digits which can be used to build the numbers.
/// * `idx` - Index of the digit of the number to build.
/// * `value` - Partially-built sub-string divisible number.
///
/// -> Sum of all numbers which can be built starting from the given resources.
fn sub_string_divisible_sum(ds: &mut DigitsSet, idx: usize, value: i64) -> i64 {
    if idx >= 10 {
        return value;
    }

    // Try to place each available digit at the current index in turn.
    (0..ds.len)
        .map(|i| {
            let digit = ds.digits[i];
            let value = value * 10 + digit;
            let subvalue = value % 1000;

            // Allow any digit to be placed at the first three indices. At any
            // other index, allow a particular digit only if the number formed
            // by taking the three least significant digits of the
            // partially-built number is divisible by the appropriate prime
            // number.
            if idx <= 2
                || (idx == 3 && subvalue % 2 == 0)
                || (idx == 4 && subvalue % 3 == 0)
                || (idx == 5 && subvalue % 5 == 0)
                || (idx == 6 && subvalue % 7 == 0)
                || (idx == 7 && subvalue % 11 == 0)
                || (idx == 8 && subvalue % 13 == 0)
                || (idx == 9 && subvalue % 17 == 0)
            {
                // Changing this number here does not affect the range iterator
                // we are inside, because it received a copy.
                ds.len -= 1;
                (ds.digits[i], ds.digits[ds.len]) = (ds.digits[ds.len], ds.digits[i]);
                let sum = sub_string_divisible_sum(ds, idx + 1, value);
                (ds.digits[i], ds.digits[ds.len]) = (ds.digits[ds.len], ds.digits[i]);
                ds.len += 1;
                return sum;
            }
            0
        })
        .sum()
}

pub fn solve() -> i64 {
    let mut ds = DigitsSet::new();
    let sum = sub_string_divisible_sum(&mut ds, 0, 0);

    assert_eq!(sum, 16695334890);
    sum
}
