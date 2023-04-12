/// Check for leap years.
///
/// * `year`
///
/// -> Whether `year` is a leap year.
fn is_leap(year: i32) -> bool
{
    year % 4 == 0 && year % 100 != 0 || year % 400 == 0
}

/// Determine the number of days in the given month of the given year.
///
/// * `year`
/// * `month` - Number from 1 to 12.
///
/// -> Number of days.
fn days_in(year: i32, month: i32) -> i32
{
    match month
    {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => if is_leap(year) { 29 } else { 28 },
        _ => -1,
    }
}

/// Main function.
fn main()
{
    // If 1 January 1900 fell on a Monday, 1 January 1901 must have fallen on a
    // Tuesday. Represent each day as an offset from Sunday. Tuesday is 2 days
    // away from Sunday.
    let mut day = 2;
    let mut sundays = 0;
    for year in 1901..=2000
    {
        for month in 1..=12
        {
            if day == 0
            {
                sundays += 1;
            }
            day = (day + days_in(year, month)) % 7;
        }
    }
    println!("{}", sundays);

    assert_eq!(sundays, 171);
}
