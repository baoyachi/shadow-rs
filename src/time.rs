use crate::Format;
use chrono::{DateTime, Local, TimeZone};

pub fn now_data_time() -> DateTime<Local> {
    match std::env::var_os("SOURCE_DATE_EPOCH") {
        None => Local::now(),
        Some(timestamp) => {
            let epoch = timestamp
                .into_string()
                .expect("Input SOURCE_DATE_EPOCH could not be parsed")
                .parse::<i64>()
                .expect("Input SOURCE_DATE_EPOCH could not be cast to a number");
            Local.timestamp(epoch, 0)
        }
    }
}

impl<Tz: TimeZone> Format for DateTime<Tz>
where
    Tz::Offset: std::fmt::Display,
{
    fn human_format(&self) -> String {
        self.format("%Y-%m-%d %H:%M:%S %:z").to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_now_data_time() {
        std::env::set_var("SOURCE_DATE_EPOCH", "1628080443");
        let time = now_data_time();
        let now = Local::now();
        assert!(time < now);
    }

    #[test]
    fn test_timezone_utc() {
        std::env::set_var("SOURCE_DATE_EPOCH", "1628080443");
        std::env::set_var("TZ", "UTC");
        let time = now_data_time();
        let utc = Utc.timestamp(1628080443, 0);
        assert_eq!(time.human_format(), utc.human_format())
    }

    #[test]
    fn test_timezone_shanghai() {
        std::env::set_var("SOURCE_DATE_EPOCH", "1628080443");
        std::env::set_var("TZ", "Asia/Shanghai");
        let time = now_data_time();
        let utc = Utc.timestamp(1628080443, 0);
        assert_eq!(time.human_format(), "2021-08-04 20:34:03 +08:00");
        assert_eq!(utc.human_format(), "2021-08-04 12:34:03 +00:00");
    }
}
