#!/usr/bin/env rust

//! Functions to enable `serde` conversion between date-time stamp from/to JSON value similar to
//!
//! ```json
//! {
//!   "created_at": "Sat Aug 12 16:10:37 +0000 2023"
//! }
//! ```
//!
//! See: https://serde.rs/custom-date-format.html

use chrono::{DateTime, Utc};
use serde::de;
use serde::{Deserialize, Deserializer, Serializer};

/// Warning; this format string may be changed at the whims of Mr. Musk
///
/// - %a -> Abbreviated day (e.g. Sun)
/// - %b -> Abbreviated month (e.g. Jan)
/// - %d -> Day of the month (e.g. 01)
/// - %T -> %H:%M:%S zero padded hour, minute, and second
/// - %z -> Numeric time zone (e.g. -0400)
/// - %Y -> Four digit year
pub const FORMAT: &str = "%a %b %d %T %z %Y";

/// Convert `DateTime` data structure into date time stamp string
///
/// ## Example
///
/// ```
/// use chrono::{DateTime, Utc};
/// use derive_more::Display;
/// use serde::{Deserialize, Serialize};
///
/// use twitter_archive::convert;
/// use twitter_archive::convert::created_at::FORMAT;
///
/// #[derive(Deserialize, Serialize, Debug, Clone, Display)]
/// struct Test {
///     #[serde(with = "convert::created_at")]
///     created_at: DateTime<Utc>,
/// }
///
/// let time = "Sat Aug 12 16:10:37 +0000 2023";
///
/// let data = Test {
///     created_at: Into::<DateTime<Utc>>::into(DateTime::parse_from_str(&time, FORMAT).unwrap()),
/// };
///
/// let json_serialize = serde_json::to_string(&data).unwrap();
///
/// let json_expected = format!(r#"{{"created_at":"{time}"}}"#);
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
/// struct Test {
///     #[serde(with = "convert::created_at")]
///     created_at: DateTime<Utc>,
/// }
///
/// let json = r#"{ "created_at": "Sat Aug 12 16:10:37 +0000 2023" }"#;
/// let data: Test = serde_json::from_str(&json).unwrap();
///
/// assert_eq!(data.created_at.format("%F").to_string(), "2023-08-12");
/// assert_eq!(data.created_at.format("%T").to_string(), "16:10:37");
/// ```
pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
	D: Deserializer<'de>,
{
	let value = String::deserialize(deserializer)?;

	let date_time: DateTime<Utc> = DateTime::parse_from_str(&value, FORMAT)
		.map_err(de::Error::custom)?
		.into();

	Ok(date_time)
}
