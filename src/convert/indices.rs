#!/usr/bin/env rust

//! Functions to enable `serde` conversion between array of `usize` with length of two from/to JSON
//! value similar to
//!
//! ```json
//! {
//!   "indices": ["68", "419"]
//! }
//! ```

use serde::de::Error;
use serde::ser::{SerializeTuple, Serializer};
use serde::{Deserialize, Deserializer};

/// Convert `[usize; 2]` data structure into JSON array of number like strings
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
/// #[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
/// struct Test {
///     #[serde(with = "convert::indices")]
///     indices: [usize; 2],
/// }
///
/// let data = Test { indices: [ 68, 419 ] };
///
/// let json_serialize = serde_json::to_string(&data).unwrap();
///
/// let json_expected = r#"{"indices":["68","419"]}"#;
///
/// assert_eq!(json_serialize, json_expected);
/// ```
pub fn serialize<S>(indices: &[usize; 2], serializer: S) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	let mut tup = serializer.serialize_tuple(2)?;
	tup.serialize_element(&indices[0].to_string())?;
	tup.serialize_element(&indices[1].to_string())?;
	tup.end()
}

/// Convert JSON array of number like strings into `[usize; 2]` data structure
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
/// #[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
/// struct Test {
///     #[serde(with = "convert::indices")]
///     indices: [usize; 2],
/// }
///
/// let json = r#"{ "indices": ["68", "419"] }"#;
/// let data: Test = serde_json::from_str(&json).unwrap();
///
/// assert_eq!(data.indices[0], 68);
/// assert_eq!(data.indices[1], 419);
/// ```
pub fn deserialize<'de, D>(deserializer: D) -> Result<[usize; 2], D::Error>
where
	D: Deserializer<'de>,
{
	let seq: Vec<String> = Deserialize::deserialize(deserializer)?;

	if seq.len() != 2 {
		return Err(Error::custom("Expected a sequence of length 2"));
	}

	let mut result = [0; 2];
	result[0] = seq[0].parse::<usize>().unwrap();
	result[1] = seq[1].parse::<usize>().unwrap();

	Ok(result)
}
