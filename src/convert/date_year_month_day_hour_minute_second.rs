#!/usr/bin/env rust

//! Functions to enable `serde` conversion between date-time stamp from/to JSON value similar to
//!
//! ```json
//! {
//!   "impressionTime" : "2023-06-05 17:00:52"
//! }
//! ```
//!
//! Check following links for further information:
//!
//! - https://serde.rs/custom-date-format.html
//! - https://en.wikipedia.org/wiki/ISO_8601

use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Deserializer, Serializer};

/// Warning; this format string may be changed at the whims of Mr. Musk
///
/// Currently looks like: "2023-06-05 17:00:52"
///
/// - %F -> Expands to "%Y-%m-%d"
///   - %Y  -> Four digit year
///   - %m  -> Two digit month
///   - %d  -> Two digit day
///
/// - %T -> Expands to "%H:%M:%S"
///   - %H -> Two digit hour, e.g. 00..23
///   - %M -> Two digit minute, e.g. 00..59
///   - %S -> Two digit second, e.g. 00..59
pub const FORMAT: &str = "%F %T";

/// Convert `DateTime` data structure into date time stamp string
///
/// ## Example
///
/// ```
/// use chrono::{DateTime, NaiveDateTime, Utc};
/// use derive_more::Display;
/// use serde::{Deserialize, Serialize};
///
/// use twitter_archive::convert;
/// use twitter_archive::convert::date_year_month_day_hour_minute_second::FORMAT;
///
/// #[derive(Deserialize, Serialize, Debug, Clone, Display)]
/// #[serde(rename_all = "camelCase")]
/// struct Test {
///     #[serde(with = "convert::date_year_month_day_hour_minute_second")]
///     impression_time: DateTime<Utc>,
/// }
///
/// let date_string = "2021-10-20 17:00:52";
///
/// let date_time = NaiveDateTime::parse_from_str(&date_string, FORMAT).unwrap();
///
/// let data = Test {
///     impression_time: DateTime::<Utc>::from_naive_utc_and_offset(date_time.into(), Utc),
/// };
///
/// let json_serialize = serde_json::to_string(&data).unwrap();
///
/// let json_expected = format!(r#"{{"impressionTime":"{date_string}"}}"#);
///
/// assert_eq!(json_serialize, json_expected);
/// ```
pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	let s = date.format(FORMAT).to_string();
	serializer.serialize_str(&s)
}

/// Convert date time stamp string into `DateTime` data structure
///
/// ## Example
///
/// ```
/// use chrono::{DateTime, Utc};
/// use derive_more::Display;
/// use serde::{Deserialize, Serialize};
///
/// use twitter_archive::convert;
///
/// #[derive(Deserialize, Serialize, Debug, Clone, Display)]
/// #[serde(rename_all = "camelCase")]
/// struct Test {
///     #[serde(with = "convert::date_year_month_day_hour_minute_second")]
///     impression_time: DateTime<Utc>,
/// }
///
/// let date_string = "2021-10-20 17:00:52";
/// let json = format!(r#"{{"impressionTime":"{date_string}"}}"#);
/// let data: Test = serde_json::from_str(&json).unwrap();
///
/// assert_eq!(data.impression_time.format(convert::date_year_month_day_hour_minute_second::FORMAT).to_string(), "2021-10-20 17:00:52");
///
/// assert_eq!(data.impression_time.format("%Y").to_string(), "2021");
/// assert_eq!(data.impression_time.format("%m").to_string(), "10");
/// assert_eq!(data.impression_time.format("%d").to_string(), "20");
///
/// assert_eq!(data.impression_time.format("%H").to_string(), "17");
/// assert_eq!(data.impression_time.format("%M").to_string(), "00");
/// assert_eq!(data.impression_time.format("%S").to_string(), "52");
/// ```
pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
	D: Deserializer<'de>,
{
	let s = String::deserialize(deserializer)?;
	let dt = NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
	Ok(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
}
