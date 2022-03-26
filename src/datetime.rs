
#![allow(non_snake_case)]

use chrono::DateTime;
use chrono::Datelike;
use chrono::TimeZone;
use chrono::Timelike;
use chrono::offset::Local;
use chrono::offset::Utc;


fn get_current_time_from<T>(timezone: DateTime<T>) -> String
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


fn get_current_date_from<T>(timezone: DateTime<T>) -> String
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


fn get_current_second_from<T>(timezone: DateTime<T>) -> String
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


fn get_current_minute_from<T>(timezone: DateTime<T>) -> String
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


fn get_current_day_from<T>(timezone: DateTime<T>) -> String
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


fn get_current_month_from<T>(timezone: DateTime<T>) -> u32
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


fn get_current_year_from<T>(timezone: DateTime<T>) -> i32
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

pub trait Strftime {
    fn to_datetime_string(&self) -> String;
}

impl Strftime for DateTime<Local> {
    fn to_datetime_string(&self) -> String {
        let d = self.day();
        let m = self.month();
        let Y = self.year();

        let S = self.second();
        let M = self.minute();
        let H = self.hour();
        format!("{d}.{m}.{Y}-{H}:{M}:{S}")
    }
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
