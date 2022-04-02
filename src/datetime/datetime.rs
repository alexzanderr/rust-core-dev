#![allow(non_snake_case)]

use std::collections::BTreeSet;
// std imports
use std::collections::HashMap;


// 3rd party lazy_statuc
use lazy_static::lazy_static;

// 3rd party tz and tzdb (related)
use tz;
use tzdb::time_zone;

// 3rd party chrono
use chrono::DateTime;
use chrono::Datelike;
use chrono::TimeZone;
use chrono::Timelike;
use chrono::offset::Local;
use chrono::offset::Utc;


pub fn get_current_tokyo_time() -> String {
    let time = tz::DateTime::now(time_zone::asia::TOKYO).unwrap();
    time.to_time_string()
}

pub fn get_current_tokyo_date() -> String {
    let time = tz::DateTime::now(time_zone::asia::TOKYO).unwrap();
    time.to_date_string()
}

pub fn get_current_tokyo_datetime() -> String {
    let time = tz::DateTime::now(time_zone::asia::TOKYO).unwrap();
    time.to_datetime_string()
}

pub fn get_current_bucharest_time() -> String {
    let time = tz::DateTime::now(time_zone::europe::BUCHAREST).unwrap();
    time.to_time_string()
}

pub trait Strftime {
    fn to_time_string(&self) -> String;
    fn to_date_string(&self) -> String;
    fn to_datetime_string(&self) -> String;
}

impl Strftime for tz::DateTime {
    fn to_time_string(&self) -> String {
        let second = self.second();
        let second_string = if second < 10 {
            format!("0{}", second)
        } else {
            format!("{}", second)
        };

        let minute = self.minute();
        let minute_string = if minute < 10 {
            format!("0{}", minute)
        } else {
            format!("{}", minute)
        };


        let hour = self.hour();
        let hour_string = if hour < 10 {
            format!("0{}", hour)
        } else {
            format!("{}", hour)
        };

        format!("{}:{}:{}", hour_string, minute_string, second_string)
    }

    fn to_date_string(&self) -> String {
        let day = self.year_day();
        let day_string = if day < 10 {
            format!("0{}", day)
        } else {
            day.to_string()
        };

        let month = self.month();
        let month_string = if month < 10 {
            format!("0{}", month)
        } else {
            month.to_string()
        };

        let year = self.year();

        format!("{}.{}.{}", day_string, month_string, year)
    }

    fn to_datetime_string(&self) -> String {
        let date = self.to_date_string();
        let time = self.to_time_string();
        format!("{date}-{time}")
    }
}


pub fn get_current_time_from<T>(timezone: DateTime<T>) -> String
where
    T: TimeZone, {
    let second = timezone.second();
    let second_string = if second < 10 {
        format!("0{}", second)
    } else {
        format!("{}", second)
    };

    let minute = timezone.minute();
    let minute_string = if minute < 10 {
        format!("0{}", minute)
    } else {
        format!("{}", minute)
    };


    let hour = timezone.hour();
    let hour_string = if hour < 10 {
        format!("0{}", hour)
    } else {
        format!("{}", hour)
    };

    format!("{}:{}:{}", hour_string, minute_string, second_string)
}


pub fn get_current_date_from<T>(timezone: DateTime<T>) -> String
where
    T: TimeZone, {
    let day = timezone.day();
    let day_string = if day < 10 {
        format!("0{}", day)
    } else {
        day.to_string()
    };

    let month = timezone.month();
    let month_string = if month < 10 {
        format!("0{}", month)
    } else {
        month.to_string()
    };

    let year = timezone.year();

    format!("{}.{}.{}", day_string, month_string, year)
}


pub fn get_current_time() -> String {
    get_current_time_from(Local::now())
}


pub fn get_current_time_utc() -> String {
    get_current_time_from(Utc::now())
}

pub fn get_current_date() -> String {
    get_current_date_from(Local::now())
}

pub fn get_current_date_utc() -> String {
    get_current_date_from(Utc::now())
}


pub fn get_current_datetime() -> String {
    let current_time = get_current_time();
    let current_date = get_current_date();
    format!("{current_date}-{current_time}")
}


pub fn get_current_datetime_utc() -> String {
    let current_time = get_current_time_utc();
    let current_date = get_current_date_utc();
    format!("{current_date}-{current_time}")
}


pub fn get_current_second_from<T>(timezone: DateTime<T>) -> String
where
    T: TimeZone, {
    let second = timezone.second();
    if second < 10 {
        format!("0{}", second)
    } else {
        second.to_string()
    }
}


pub fn get_current_second() -> String {
    get_current_second_from(Local::now())
}


pub fn get_current_second_utc() -> String {
    get_current_second_from(Utc::now())
}


pub fn get_current_minute_from<T>(timezone: DateTime<T>) -> String
where
    T: TimeZone, {
    let minute = timezone.minute();
    if minute < 10 {
        format!("0{}", minute)
    } else {
        minute.to_string()
    }
}


pub fn get_current_minute() -> String {
    get_current_minute_from(Local::now())
}


pub fn get_current_minute_utc() -> String {
    get_current_minute_from(Utc::now())
}


fn get_current_hour_from<T>(timezone: DateTime<T>) -> String
where
    T: TimeZone, {
    let hour = timezone.hour();
    if hour < 10 {
        format!("0{}", hour)
    } else {
        hour.to_string()
    }
}


pub fn get_current_hour() -> String {
    get_current_hour_from(Local::now())
}


pub fn get_current_hour_utc() -> String {
    get_current_hour_from(Utc::now())
}


pub fn get_current_day_from<T>(timezone: DateTime<T>) -> String
where
    T: TimeZone, {
    let day = timezone.day();
    if day < 10 {
        format!("0{}", day)
    } else {
        day.to_string()
    }
}


pub fn get_current_day() -> String {
    get_current_day_from(Local::now())
}


pub fn get_current_day_utc() -> String {
    get_current_day_from(Utc::now())
}


pub fn get_current_month_from<T>(timezone: DateTime<T>) -> u32
where
    T: TimeZone, {
    timezone.month()
}


pub fn get_current_month() -> u32 {
    get_current_month_from(Local::now())
}


pub fn get_current_month_utc() -> u32 {
    get_current_month_from(Utc::now())
}


pub fn get_current_year_from<T>(timezone: DateTime<T>) -> i32
where
    T: TimeZone, {
    timezone.year()
}


pub fn get_current_year() -> i32 {
    get_current_year_from(Local::now())
}


pub fn get_current_year_utc() -> i32 {
    get_current_year_from(Utc::now())
}

pub trait TimeDiff<T>
where
    T: TimeZone, {
    fn nano_secs_diff(&self, other: DateTime<T>) -> i64;
    fn micro_secs_diff(&self, other: DateTime<T>) -> i64;
    fn milli_secs_diff(&self, other: DateTime<T>) -> i64;

    fn seconds_diff(&self, other: DateTime<T>) -> i64;
    fn minutes_diff(&self, other: DateTime<T>) -> i64;
    fn hours_diff(&self, other: DateTime<T>) -> i64;

    fn days_diff(&self, other: DateTime<T>) -> i64;
    fn weeks_diff(&self, other: DateTime<T>) -> i64;
    fn years_diff(&self, other: DateTime<T>) -> i64;
}


impl TimeDiff<Local> for DateTime<Local> {
    fn nano_secs_diff(&self, other: DateTime<Local>) -> i64 {
        let diff = *self - other;
        match diff.num_nanoseconds() {
            Some(nano_secs) => nano_secs,
            None => 0,
        }
    }

    fn micro_secs_diff(&self, other: DateTime<Local>) -> i64 {
        let diff = *self - other;
        match diff.num_microseconds() {
            Some(micro_secs) => micro_secs,
            None => 0,
        }
    }

    fn milli_secs_diff(&self, other: DateTime<Local>) -> i64 {
        let diff = *self - other;
        diff.num_milliseconds()
    }

    fn seconds_diff(&self, other: DateTime<Local>) -> i64 {
        let diff = *self - other;
        diff.num_seconds()
    }

    fn minutes_diff(&self, other: DateTime<Local>) -> i64 {
        let diff = *self - other;
        diff.num_minutes()
    }

    fn hours_diff(&self, other: DateTime<Local>) -> i64 {
        let diff = *self - other;
        diff.num_hours()
    }

    fn days_diff(&self, other: DateTime<Local>) -> i64 {
        let diff = *self - other;
        diff.num_days()
    }

    fn weeks_diff(&self, other: DateTime<Local>) -> i64 {
        let diff = *self - other;
        diff.num_weeks()
    }

    fn years_diff(&self, other: DateTime<Local>) -> i64 {
        let diff = *self - other;
        diff.num_days() / 365
    }
}


impl TimeDiff<Utc> for DateTime<Utc> {
    fn nano_secs_diff(&self, other: DateTime<Utc>) -> i64 {
        let diff = *self - other;
        match diff.num_nanoseconds() {
            Some(nano_secs) => nano_secs,
            None => 0,
        }
    }

    fn micro_secs_diff(&self, other: DateTime<Utc>) -> i64 {
        let diff = *self - other;
        match diff.num_microseconds() {
            Some(micro_secs) => micro_secs,
            None => 0,
        }
    }

    fn milli_secs_diff(&self, other: DateTime<Utc>) -> i64 {
        let diff = *self - other;
        diff.num_milliseconds()
    }

    fn seconds_diff(&self, other: DateTime<Utc>) -> i64 {
        let diff = *self - other;
        diff.num_seconds()
    }

    fn minutes_diff(&self, other: DateTime<Utc>) -> i64 {
        let diff = *self - other;
        diff.num_minutes()
    }

    fn hours_diff(&self, other: DateTime<Utc>) -> i64 {
        let diff = *self - other;
        diff.num_hours()
    }

    fn days_diff(&self, other: DateTime<Utc>) -> i64 {
        let diff = *self - other;
        diff.num_days()
    }

    fn weeks_diff(&self, other: DateTime<Utc>) -> i64 {
        let diff = *self - other;
        diff.num_weeks()
    }

    fn years_diff(&self, other: DateTime<Utc>) -> i64 {
        let diff = *self - other;
        diff.num_days() / 365
    }
}


// impl Strftime for DateTime<Local> {
//     fn to_datetime_string(&self) -> String {
//         let d = self.day();
//         let m = self.month();
//         let Y = self.year();

//         let S = self.second();
//         let M = self.minute();
//         let H = self.hour();
//         format!("{d}.{m}.{Y}-{H}:{M}:{S}")
//     }
// }


pub const MONTHS: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

pub fn get_current_month_name() -> String {
    let now = Local::now().month0() as usize;
    MONTHS[now].to_string()
}


pub const MONTHS_RO: [&str; 12] = [
    "Ianuarie",
    "Februarie",
    "Martie",
    "Aprilie",
    "Mai",
    "Iunie",
    "Iulie",
    "August",
    "Septembrie",
    "Octombrie",
    "Noiembrie",
    "Decembrie",
];


pub fn get_current_month_ro_name() -> String {
    let now = Local::now().month0() as usize;
    MONTHS_RO[now].to_string()
}


const TIME_INTERVALS: [&'static str; 9] = [
    "millennials",
    "centuries",
    "decades",
    "years",
    "weeks",
    "days",
    "hours",
    "minutes",
    "seconds",
];

use std::collections::BTreeMap;


#[derive(Default)]
pub struct TimeAttributes {
    pub millennials: usize,
    pub centuries:   usize,
    pub decades:     usize,
    pub years:       usize,
    pub weeks:       usize,
    pub days:        usize,
    pub hours:       usize,
    pub minutes:     usize,
    pub seconds:     usize,
}


impl std::fmt::Display for TimeAttributes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("3h 3m 3s")
    }
}


pub const TIME_INTERVALS_AS_SECONDS: [(&'static str, i128); 9] = [
    ("millennia", 60 * 60 * 24 * 365 * 1000),
    ("century", 60 * 60 * 24 * 365 * 100),
    ("decade", 60 * 60 * 24 * 365 * 10),
    ("year", 60 * 60 * 24 * 365),
    ("week", 60 * 60 * 24 * 7),
    ("day", 60 * 60 * 24),
    ("hour", 60 * 60),
    ("minute", 60),
    ("second", 1),
];


pub fn seconds_to_time_map<'a>(
    mut seconds: usize,
) -> HashMap<&'a str, i128> {
    let mut time_intervals_values: HashMap<&'a str, i128> = vec![
        ("millennials", 0),
        ("centuries", 0),
        ("decades", 0),
        ("years", 0),
        ("weeks", 0),
        ("days", 0),
        ("hours", 0),
        ("minutes", 0),
        ("seconds", 0),
    ]
    .into_iter()
    .collect();

    // dbg!(&time_intervals_values);

    for (_seconds, _interval) in TIME_INTERVALS_AS_SECONDS
        .iter()
        .map(|f| f.1)
        .zip(TIME_INTERVALS.iter())
    {
        // dbg!((&_seconds, &_interval));
        let result = seconds / _seconds as usize;
        seconds -= result * _seconds as usize;
        time_intervals_values
            .entry(&_interval)
            .and_modify(|e| *e = result as i128);
    }

    // dbg!(&time_intervals_values);
    time_intervals_values
}


#[cfg(test)]
mod tests {
    use super::TimeDiff;
    use super::Local;
    use super::get_current_year_from;

    use pretty_assertions::assert_eq;

    #[test]
    fn from_local() {
        let cy = get_current_year_from(Local::now());
        assert_eq!(cy > 21, true)
    }

    #[test]
    fn test_diff() {
        let acum = Local::now();
        std::thread::sleep(std::time::Duration::from_secs(5));
        let maine = Local::now();
        println!("{}", acum.days_diff(maine));
        println!("{}", acum.seconds_diff(maine));
    }
}
