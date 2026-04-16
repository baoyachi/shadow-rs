use crate::{Format, SdResult, ShadowError};

pub struct DateTime(jiff::Zoned);

pub(crate) const DEFINE_SOURCE_DATE_EPOCH: &str = "SOURCE_DATE_EPOCH";

pub fn now_date_time() -> DateTime {
    // Enable reproducibility for uses of `now_date_time` by respecting the
    // `SOURCE_DATE_EPOCH` env variable.
    //
    // https://reproducible-builds.org/docs/source-date-epoch/
    match std::env::var_os(DEFINE_SOURCE_DATE_EPOCH) {
        None => DateTime::now(),
        Some(timestamp) => {
            let epoch = timestamp
                .into_string()
                .unwrap_or_else(|_| panic!("Input {DEFINE_SOURCE_DATE_EPOCH} could not be parsed"))
                .parse::<i64>()
                .unwrap_or_else(|_| {
                    panic!("Input {DEFINE_SOURCE_DATE_EPOCH} could not be cast to a number")
                });
            DateTime(
                jiff::Timestamp::from_second(epoch)
                    .unwrap()
                    .to_zoned(jiff::tz::TimeZone::UTC),
            )
        }
    }
}

impl Default for DateTime {
    fn default() -> Self {
        Self::now()
    }
}

impl DateTime {
    pub fn new(zoned: jiff::Zoned) -> Self {
        Self(zoned)
    }

    pub fn now() -> Self {
        Self(jiff::Zoned::now())
    }

    pub fn timestamp_2_utc(time_stamp: i64) -> SdResult<Self> {
        let utc_time = jiff::Timestamp::from_second(time_stamp).map_err(ShadowError::new)?;
        let zoned = utc_time.to_zoned(jiff::tz::TimeZone::UTC);
        Ok(DateTime::new(zoned))
    }

    pub fn from_iso8601_string(iso_string: &str) -> SdResult<Self> {
        let pieces = jiff::fmt::temporal::Pieces::parse(iso_string).map_err(ShadowError::new)?;

        let time = match pieces.time() {
            Some(time) => time,
            None => {
                return Err(ShadowError::from(format!(
                    "iso string has no time, and thus cannot be parsed into a datetime",
                )));
            }
        };
        let dt = pieces.date().to_datetime(time);
        let offset = match pieces.to_numeric_offset() {
            Some(offset) => offset,
            None => {
                return Err(ShadowError::from(format!(
                    "iso string has no offset, and thus cannot be parsed into a datetime",
                )));
            }
        };
        let zoned = jiff::tz::TimeZone::fixed(offset)
            .to_zoned(dt)
            .map_err(ShadowError::new)?;

        Ok(DateTime::new(zoned))
    }

    pub fn to_rfc2822(&self) -> String {
        jiff::fmt::rfc2822::to_string(&self.0).unwrap_or_default()
    }

    pub fn to_rfc3339(&self) -> String {
        let ts = self.0.timestamp();
        let offset = self.0.offset();
        if self.0.time_zone() == &jiff::tz::TimeZone::UTC {
            ts.to_string()
        } else {
            ts.display_with_offset(offset).to_string()
        }
    }

    pub fn timestamp(&self) -> i64 {
        self.0.timestamp().as_second()
    }
}

impl Format for DateTime {
    fn human_format(&self) -> String {
        self.0.strftime("%Y-%m-%d %H:%M:%S %:z").to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod human_format_validate {
        use std::num::{NonZeroU32, NonZeroU8};
        use winnow::ascii::{digit1, space1};
        use winnow::error::{ContextError, ParseError};
        use winnow::token::{literal, take};
        use winnow::{ModalResult, Parser};

        fn u8_len2(input: &mut &str) -> ModalResult<u8> {
            take(2_usize).try_map(str::parse).parse_next(input)
        }

        fn non_zero_u8_len2<const LIMIT: u8>(input: &mut &str) -> ModalResult<NonZeroU8> {
            take(2_usize)
                .try_map(str::parse)
                .verify(|x| *x <= unsafe { NonZeroU8::new_unchecked(LIMIT) })
                .parse_next(input)
        }

        //
        fn non_zero_u32(input: &mut &str) -> ModalResult<NonZeroU32> {
            digit1.try_map(str::parse).parse_next(input)
        }

        // 2022-07-14 00:40:05 +08:00
        pub(crate) fn parse_human_format(
            input: &str,
        ) -> Result<(), ParseError<&str, ContextError>> {
            (
                non_zero_u32,
                literal('-'),
                non_zero_u8_len2::<12>,
                literal('-'),
                non_zero_u8_len2::<31>,
                space1,
                u8_len2,
                literal(':'),
                u8_len2,
                literal(':'),
                u8_len2,
                space1,
                literal('+'),
                u8_len2,
                literal(':'),
                u8_len2,
            )
                .parse(input)?;
            Ok(())
        }

        #[test]
        fn test_parse() {
            assert!(parse_human_format("2022-07-14 00:40:05 +08:00").is_ok());
            assert!(parse_human_format("2022-12-14 00:40:05 +08:00").is_ok());
            assert!(parse_human_format("2022-13-14 00:40:05 +08:00").is_err());
            assert!(parse_human_format("2022-12-31 00:40:05 +08:00").is_ok());
            assert!(parse_human_format("2022-12-32 00:40:05 +08:00").is_err());
            assert!(parse_human_format("2022-07-14 00:40:05 +08:0").is_err());
            assert!(parse_human_format("2022-07-14 00:40:05 -08:0").is_err());
            assert!(parse_human_format("2022-07-00 00:40:05 +08:00").is_err());
            assert!(parse_human_format("2022-00-01 00:40:05 +08:00").is_err());
            assert!(parse_human_format("2022-00-01 00:40:05 08:00").is_err());
            assert!(parse_human_format("2022-00-01 00:40:05+08:00").is_err());
            assert!(parse_human_format("20221-00-01 00:40:05+08:00").is_err());
            assert!(parse_human_format("20221-01-01 00:40:05 +08:00").is_ok());
        }
    }

    #[test]
    fn test_source_date_epoch() {
        std::env::set_var(DEFINE_SOURCE_DATE_EPOCH, "1628080443");
        let time = now_date_time();
        assert_eq!(time.human_format(), "2021-08-04 12:34:03 +00:00");
    }

    #[test]
    fn test_timestamp_2_utc() {
        let time = DateTime::timestamp_2_utc(1628080443).unwrap();
        assert_eq!(time.to_rfc2822(), "Wed, 4 Aug 2021 12:34:03 +0000");
        assert_eq!(time.to_rfc3339(), "2021-08-04T12:34:03Z");
        assert_eq!(time.human_format(), "2021-08-04 12:34:03 +00:00");
        assert_eq!(time.timestamp(), 1628080443);
    }

    #[test]
    fn test_from_iso8601_string() {
        let time = DateTime::from_iso8601_string("2021-08-04T12:34:03+08:00").unwrap();
        assert_eq!(time.to_rfc2822(), "Wed, 4 Aug 2021 12:34:03 +0800");
        assert_eq!(time.to_rfc3339(), "2021-08-04T12:34:03+08:00");
        assert_eq!(time.human_format(), "2021-08-04 12:34:03 +08:00");
    }
}
