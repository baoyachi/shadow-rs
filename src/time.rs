use std::error::Error;
use crate::Format;
use chrono::{DateTime, Local, NaiveDateTime, SecondsFormat, TimeZone, Utc};

pub enum BuildTime {
    Local(tz::DateTime),
    Utc(time::OffsetDateTime),
}

fn local_now() -> Result<tz::DateTime,tz::TzError>{
    let current_utc_date_time = tz::UtcDateTime::now()?;
    let date_time = current_utc_date_time.project(tz::TimeZone::local().unwrap().as_ref())?;
    Ok(date_time)
}

pub fn now_data_time() -> BuildTime {
    // Enable reproducibility for uses of `now_data_time` by respecting the
    // `SOURCE_DATE_EPOCH` env variable.
    //
    // https://reproducible-builds.org/docs/source-date-epoch/
    println!("cargo:rerun-if-env-changed=SOURCE_DATE_EPOCH");
    match std::env::var_os("SOURCE_DATE_EPOCH") {
        None => BuildTime::Local(local_now().unwrap()),
        Some(timestamp) => {
            let epoch = timestamp
                .into_string()
                .expect("Input SOURCE_DATE_EPOCH could not be parsed")
                .parse::<i64>()
                .expect("Input SOURCE_DATE_EPOCH could not be cast to a number");
            // BuildTime::Utc(Utc.timestamp(epoch, 0))
            BuildTime::Utc(OffsetDateTime::from_unix_timestamp(epoch).unwrap())
        }
    }
}

impl BuildTime {
    pub fn local_now() -> Self {
        BuildTime::Local(local_now().unwrap())
    }

    pub fn timestamp_2_utc(time_stamp: i64) -> Self {
        let time = OffsetDateTime::from_unix_timestamp(time_stamp).unwrap();
        BuildTime::Utc(time)
    }

    pub fn to_rfc2822(&self) -> String {
        match self {
            BuildTime::Local(dt) => dt.to_rfc2822(),
            BuildTime::Utc(dt) => dt.to_rfc2822(),
        }
    }

    pub fn to_rfc3339(&self) -> String {
        let sec_form = SecondsFormat::Secs;
        let use_z = true;
        match self {
            BuildTime::Local(dt) => dt.to_rfc3339_opts(sec_form, use_z),
            BuildTime::Utc(dt) => dt.to_rfc3339_opts(sec_form, use_z),
        }
    }
}

impl Format for BuildTime {
    fn human_format(&self) -> String {
        let fmt = "%Y-%m-%d %H:%M:%S %:z";
        match self {
            BuildTime::Local(dt) => dt.format(fmt).to_string(),
            BuildTime::Utc(dt) => dt.format(fmt).to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_source_date_epoch() {
        std::env::set_var("SOURCE_DATE_EPOCH", "1628080443");
        let time = now_data_time();
        assert_eq!(time.human_format(), "2021-08-04 12:34:03 +00:00");
    }

    #[test]
    fn test_local_now_human_format() {
        let time = BuildTime::local_now().human_format();
        println!("{}",time);// 2022-07-14 00:40:05 +08:00

        let current_utc_date_time = tz::UtcDateTime::now().unwrap();
        let date_time = current_utc_date_time.project(tz::TimeZone::local().unwrap().as_ref()).unwrap();
        println!("{}",tz_local_time_format(date_time).unwrap());
        assert_eq!(time,tz_local_time_format(date_time).unwrap())
    }
}

use std::fmt::Write;
use time::OffsetDateTime;

fn tz_local_time_format(date_time:tz::DateTime) -> std::result::Result<String,Box<dyn Error>>{
    let ut_offset = date_time.local_time_type().ut_offset();

    /// Number of seconds in one minute
    pub const SECONDS_PER_MINUTE: i64 = 60;
    /// Number of minutes in one hour
    pub const MINUTES_PER_HOUR: i64 = 60;
    /// Number of seconds in one hour
    pub const SECONDS_PER_HOUR: i64 = 3600;


    let mut f = String::new();
    write!(f, "{}-{:02}-{:02} {:02}:{:02}:{:02} ",
           date_time.year(),
           date_time.month(),
           date_time.month_day(),
           date_time.hour(),
           date_time.minute(),
           date_time.second())?;

    if ut_offset != 0 {
        let ut_offset = ut_offset as i64;
        let ut_offset_abs = ut_offset.abs();

        let offset_hour = ut_offset / SECONDS_PER_HOUR;
        let offset_minute = (ut_offset_abs / SECONDS_PER_MINUTE) % MINUTES_PER_HOUR;
        let offset_second = ut_offset_abs % SECONDS_PER_MINUTE;

        write!(f, "{:+03}:{:02}", offset_hour, offset_minute)?;

        if offset_second != 0 {
            write!(f, ":{:02}", offset_second)?;
        }
    } else {
        write!(f, "Z")?;
    }
    Ok(f)
}

