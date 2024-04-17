#!/usr/bin/env rust

//! Functions to enable `serde` conversion between number like string values similar to
//!
//! ```json
//! {
//!   "editsRemaining": "5"
//! }
//! ```

use serde::ser::Serializer;
use serde::{Deserialize, Deserializer};

/// Convert `usize` type into JSON number like string
///
/// ## Example
///
/// ```
/// use derive_more::Display;
/// use serde::{Deserialize, Serialize};
///
/// use twitter_archive::convert;
///
/// #[derive(Deserialize, Serialize, Debug, Clone, Display)]
/// struct Test {
///     #[serde(with = "convert::number_like_string")]
///     favorite_count: usize,
/// }
///
/// let value = 68419;
/// 
/// let data = Test { favorite_count: value };
///
/// let json_serialize = serde_json::to_string(&data).unwrap();
///
/// let json_expected = format!(r#"{{"favorite_count":"{value}"}}"#);
///
/// assert_eq!(json_serialize, json_expected);
/// ```
pub fn serialize<S>(value: &usize, serializer: S) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	serializer.serialize_str(&value.to_string())
}

/// Convert JSON number like string into `usize` type
///
/// ## Example
///
/// ```
/// use derive_more::Display;
/// use serde::{Deserialize, Serialize};
///
/// use twitter_archive::convert;
///
/// #[derive(Deserialize, Serialize, Debug, Clone, Display)]
/// struct Test {
///     #[serde(with = "convert::number_like_string")]
///     favorite_count: usize,
/// }
///
/// let value = 68419;
/// let json = format!(r#"{{ "favorite_count": "{value}" }}"#);
/// let data: Test = serde_json::from_str(&json).unwrap();
///
/// assert_eq!(data.favorite_count, value);
/// ```
pub fn deserialize<'de, D>(deserializer: D) -> Result<usize, D::Error>
where
	D: Deserializer<'de>,
{
	let number: String = Deserialize::deserialize(deserializer)?;
	Ok(number.parse::<usize>().unwrap())
}
