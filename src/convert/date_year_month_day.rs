#!/usr/bin/env rust

//! Functions to enable `serde` conversion between date-time stamp from/to JSON value similar to
//!
//! ```json
//! {
//!   "updatedDate" : "2021.10.20",
//!   "createdDate" : "2020.02.01"
//! }
//! ```
//!
//! Check following links for further information:
//!
//! - https://serde.rs/custom-date-format.html
//! - https://en.wikipedia.org/wiki/ISO_8601

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Deserializer, Serializer};

/// Warning; this format string may be changed at the whims of Mr. Musk
///
/// Currently looks like: "2023-08-30T23:20:03.000Z"
/// Which may be close, though not exactly, ISO 8601
///
/// - %Y  -> Four digit year
/// - %m  -> Two digit month
/// - %d  -> Two digit day
pub const FORMAT: &str = "%Y.%m.%d";

/// Convert `DateTime` data structure into date time stamp string
///
/// ## Example
///
/// ```
/// use chrono::{DateTime, NaiveDate, Utc};
/// use derive_more::Display;
/// use serde::{Deserialize, Serialize};
///
/// use twitter_archive::convert;
/// use twitter_archive::convert::date_year_month_day::FORMAT;
///
/// #[derive(Deserialize, Serialize, Debug, Clone, Display)]
/// #[serde(rename_all = "camelCase")]
/// struct Test {
///     #[serde(with = "convert::date_year_month_day")]
///     updated_date: DateTime<Utc>,
/// }
///
/// let date_string = "2021.10.20";
///
/// let date_time = NaiveDate::parse_from_str(&date_string, FORMAT).unwrap();
///
/// let data = Test {
///     updated_date: DateTime::<Utc>::from_naive_utc_and_offset(date_time.into(), Utc),
/// };
///
/// let json_serialize = serde_json::to_string(&data).unwrap();
///
/// let json_expected = format!(r#"{{"updatedDate":"{date_string}"}}"#);
///
/// assert_eq!(json_serialize, json_expected);
/// ```
pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	let s = format!("{}", date.format(FORMAT));
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
///     #[serde(with = "convert::date_year_month_day")]
///     created_date: DateTime<Utc>,
/// }
///
/// let date_string = "2021.10.20";
/// let json = format!(r#"{{"createdDate":"{date_string}"}}"#);
/// let data: Test = serde_json::from_str(&json).unwrap();
///
/// assert_eq!(data.created_date.format(convert::date_year_month_day::FORMAT).to_string(), "2021.10.20");
/// assert_eq!(data.created_date.format("%Y").to_string(), "2021");
/// assert_eq!(data.created_date.format("%m").to_string(), "10");
/// assert_eq!(data.created_date.format("%d").to_string(), "20");
/// ```
pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
	D: Deserializer<'de>,
{
	let s = String::deserialize(deserializer)?;
	let dt = NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
	Ok(DateTime::<Utc>::from_naive_utc_and_offset(dt.into(), Utc))
}
