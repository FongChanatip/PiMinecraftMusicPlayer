use chrono::{DateTime, Datelike, TimeZone, Timelike, Utc};
use chrono_tz::America::Los_Angeles;

pub struct Time{
    pub min: u8,
    pub hour: u8,
    pub day: u8,
    pub month: u8,
    pub year: u16,
    pub season: String
}

pub fn get_pacific_dt() -> DateTime<chrono_tz::Tz> {
    let utc: DateTime<Utc> = Utc::now();

    let pacific: DateTime<chrono_tz::Tz> = Los_Angeles.from_utc_datetime(&utc.naive_utc());

    pacific
}

pub fn get_pacific_time() -> Time {
    let t = get_pacific_dt();
    Time {
        min: get_pacific_minute(t), 
        hour: get_pacific_hour(t),
        day: get_pacific_day(t),
        month: get_pacific_month(t),
        year: get_pacific_year(t),
        season: match get_pacific_month(t) {
            11 | 0..=2 => "winter".to_string(),
            3..=5 => "spring".to_string(),
            6..=8 => "summer".to_string(),
            9 | 10 => "fall".to_string(),
            _ => "summer".to_string()
        }
    }
}

pub fn get_pacific_year(cur_dt: DateTime<chrono_tz::Tz>) -> u16 {
    cur_dt.year() as u16
}

pub fn get_pacific_month(cur_dt: DateTime<chrono_tz::Tz>) -> u8 {
    cur_dt.month() as u8
}

pub fn get_pacific_day(cur_dt: DateTime<chrono_tz::Tz>) -> u8 {
    cur_dt.day() as u8
}

pub fn get_pacific_hour(cur_dt: DateTime<chrono_tz::Tz>) -> u8 {
    cur_dt.hour() as u8
}

pub fn get_pacific_minute(cur_dt: DateTime<chrono_tz::Tz>) -> u8 {
    cur_dt.minute() as u8
}
pub fn get_time_of_day(cur_hour: u8, cur_min: u8) -> f32 {
    let cur_time: f32 = cur_hour as f32 + (cur_min as f32 / 60.);
    cur_time
}

// Count fri,sat,sun as weekend
pub fn is_weekend(cur_dt: DateTime<chrono_tz::Tz>) -> bool {
    let day = cur_dt.weekday().num_days_from_monday();
    return day == 4 || day == 5 || day == 6;
}