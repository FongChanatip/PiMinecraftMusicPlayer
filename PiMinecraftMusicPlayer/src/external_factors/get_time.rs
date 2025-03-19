use chrono::{DateTime, Datelike, TimeZone, Timelike, Utc};
use chrono_tz::America::Los_Angeles;

pub fn get_pacific_dt() -> DateTime<chrono_tz::Tz> {
    let utc: DateTime<Utc> = Utc::now();

    let pacific: DateTime<chrono_tz::Tz> = Los_Angeles.from_utc_datetime(&utc.naive_utc());

    pacific
}

pub fn get_pacific_year(cur_dt: DateTime<chrono_tz::Tz>) -> i16 {
    cur_dt.year() as i16
}

pub fn get_pacific_month(cur_dt: DateTime<chrono_tz::Tz>) -> i8 {
    cur_dt.month() as i8
}

pub fn get_pacific_day(cur_dt: DateTime<chrono_tz::Tz>) -> i8 {
    cur_dt.day() as i8
}

pub fn get_pacific_hour(cur_dt: DateTime<chrono_tz::Tz>) -> i8 {
    cur_dt.hour() as i8
}

pub fn get_pacific_minute(cur_dt: DateTime<chrono_tz::Tz>) -> i8 {
    cur_dt.minute() as i8
}
pub fn get_time_of_day(cur_hour: i8, cur_min: i8) -> f32 {
    let cur_time: f32 = cur_hour as f32 + (cur_min as f32 / 60.);
    cur_time
}
