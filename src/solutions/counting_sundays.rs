/// Check for leap years.
///
/// * `year`
///
/// -> Whether the input is a leap year.
fn is_leap(year: i64) -> bool {
    year % 4 == 0 && year % 100 != 0 || year % 400 == 0
}

/// Determine the number of days in the given month of the given year.
///
/// * `year`
/// * `month` - Number from 1 to 12.
///
/// -> Number of days, or 0 if the month is invalid.
fn days_in(year: i64, month: i64) -> i64 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap(year) {
                29
            } else {
                28
            }
        }
        _ => 0,
    }
}

pub fn solve() -> i64 {
    // If 1 January 1900 was a Monday, then 1 January 1901 was a Tuesday.
    // Represent each day as an offset from Sunday. (e.g. Tuesday is 2.)
    let mut day = 2;
    let mut sundays = 0;
    for year in 1901..=2000 {
        for month in 1..=12 {
            if day == 0 {
                sundays += 1;
            }
            day = (day + days_in(year, month)) % 7;
        }
    }

    assert_eq!(sundays, 171);
    sundays
}
