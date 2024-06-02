/// Convert a number into a string which indicates how it would be read.
///
/// * `num`
fn convert(num: usize) -> String {
    // Handle numbers without any discernible pattern in their names.
    let result = match num {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        10 => "ten",
        11 => "eleven",
        12 => "twelve",
        13 => "thirteen",
        14 => "fourteen",
        15 => "fifteen",
        16 => "sixteen",
        17 => "seventeen",
        18 => "eighteen",
        19 => "nineteen",
        20 => "twenty",
        30 => "thirty",
        40 => "forty",
        50 => "fifty",
        60 => "sixty",
        70 => "seventy",
        80 => "eighty",
        90 => "ninety",
        1000 => "onethousand",
        _ => "",
    };
    if !result.is_empty() {
        return result.to_string();
    }

    // Handle the hundreds place.
    let quotient = num / 100;
    let hundreds = if quotient > 0 {
        let mut hundreds = convert(quotient);
        hundreds.push_str("hundred");
        hundreds
    } else {
        "".to_string()
    };

    // Handle the rest of the number.
    let remainder_100 = num % 100;
    let remainder_10 = num % 10;
    let tens_units = if remainder_100 == 0 {
        "".to_string()
    } else if remainder_100 < 20 || remainder_10 == 0 {
        convert(remainder_100)
    } else {
        let units = convert(remainder_10);
        let tens = convert(remainder_100 - remainder_10);
        let mut tens_units = tens;
        tens_units.push_str(&units);
        tens_units
    };

    let mut result = hundreds;
    if !tens_units.is_empty() {
        if !result.is_empty() {
            result.push_str("and");
        }
        result.push_str(&tens_units);
    }
    result
}

pub fn solve() -> i64 {
    let result: usize = (1..=1000).map(|num| convert(num).len()).sum();

    assert_eq!(result, 21124);
    result as i64
}
