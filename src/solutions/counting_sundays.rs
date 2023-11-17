use crate::utils;

pub fn solve() {
    // If 1 January 1900 was a Monday, then 1 January 1901 was a Tuesday.
    // Represent each day as an offset from Sunday. (e.g. Tuesday is 2.)
    let mut day = 2;
    let mut sundays = 0;
    for year in 1901..=2000 {
        for month in 1..=12 {
            if day == 0 {
                sundays += 1;
            }
            day = (day + utils::days_in(year, month)) % 7;
        }
    }

    print!("{}", sundays);
    assert_eq!(sundays, 171);
}
