#!/usr/bin/env rust

//! Functions to enable `serde` conversion between date-time stamp from/to JSON value similar to
//!
//! ```json
//! {
//!   "editableUntil": "2023-08-30T23:20:03.000Z"
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
/// Currently looks like: "2023-08-30T23:20:03.000Z"
/// Which may be close, though not exactly, ISO 8601
///
/// - %F  -> %Y-%m-%d
/// - T   -> Time separator between date and time values
/// - %T  -> %H:%M:%S
/// - %3f -> millisecond
/// - Z   -> Zulu?
pub const FORMAT: &str = "%FT%T.%3fZ";

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
/// use twitter_archive::convert::date_time_iso_8601::FORMAT;
///
/// #[derive(Deserialize, Serialize, Debug, Clone, Display)]
/// #[serde(rename_all = "camelCase")]
/// struct Test {
///     #[serde(with = "convert::date_time_iso_8601")]
///     editable_until: DateTime<Utc>,
/// }
///
/// let time = "2023-08-12T17:10:37.000Z";
///
/// let date_time = NaiveDateTime::parse_from_str(&time, FORMAT).unwrap();
///
/// let data = Test {
///     editable_until: DateTime::<Utc>::from_naive_utc_and_offset(date_time, Utc),
/// };
///
/// let json_serialize = serde_json::to_string(&data).unwrap();
///
/// let json_expected = format!(r#"{{"editableUntil":"{time}"}}"#);
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
///     #[serde(with = "convert::date_time_iso_8601")]
///     editable_until: DateTime<Utc>,
/// }
///
/// let json = r#"{ "editableUntil": "2023-08-12T17:10:37.000Z" }"#;
/// let data: Test = serde_json::from_str(&json).unwrap();
///
/// assert_eq!(data.editable_until.format("%F").to_string(), "2023-08-12");
/// assert_eq!(data.editable_until.format("%T").to_string(), "17:10:37");
/// ```
pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
	D: Deserializer<'de>,
{
	let s = String::deserialize(deserializer)?;
	let dt = NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
	Ok(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
}
