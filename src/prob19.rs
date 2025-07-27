use chrono::{Datelike, Duration, NaiveDate, Weekday};

pub fn prob19() -> u64 {
    let begin = NaiveDate::from_ymd_opt(1901, 1, 1)
        .expect("invalid date!");
    let end = NaiveDate::from_ymd_opt(2000, 12, 31)
        .expect("invalid date!");

    let mut sundays = 0;
    let mut current = begin;

    while current <= end {
        if current.weekday() == Weekday::Sun && current.day0() == 1 {
            sundays += 1;
        }

        current = current + Duration::days(1);
    }

    sundays
}
