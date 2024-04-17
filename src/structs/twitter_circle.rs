#!/usr/bin/env rust

//! Tweeter archives as of 2023-08-31 have public twitter_circle found under;
//!
//!   twitter-<DATE>-<UID>.zip:data/twitter-circle.js
//!
//! ## Example file reader
//!
//! ```no_build
//! use std::io::Read;
//! use std::{fs, path};
//! use zip::read::ZipArchive;
//!
//! use twitter_archive::structs::twitter_circle;
//!
//! fn main() {
//!     let input_file = "path/to/twitter.zip";
//!
//!     let file_descriptor = fs::File::open(input_file).expect("Unable to read --input-file");
//!     let mut zip_archive = ZipArchive::new(file_descriptor).unwrap();
//!     let mut zip_file = zip_archive.by_name("data/twitter-circle.js").unwrap();
//!     let mut buff = String::new();
//!     zip_file.read_to_string(&mut buff).unwrap();
//!
//!     let json = buff.replacen("window.YTD.twitter_circle.part0 = ", "", 1);
//!     let data: Vec<twitter_circle::TwitterCircleObject> = serde_json::from_str(&json).expect("Unable to parse");
//!
//!     for (index, object) in data.iter().enumerate() {
//!         /* Do stuff with each Tweet */
//!         println!("Index: {index}");
//!         println!("Circle ID: {}", object.twitter_circle.id);
//!         println!("Owner User ID: {}", object.twitter_circle.owner_user_id);
//!         println!("Created At: {}", object.twitter_circle.created_at);
//!     }
//! }
//! ```
//!
//! ## Example `twitter-<DATE>-<UID>.zip:data/twitter_circle.js` content
//!
//! ```javascript
//! window.YTD.twitter_circle.part0 = [
//!   {
//!     "twitterCircle" : {
//!       "id" : "1564790306968592384",
//!       "ownerUserId" : "435455769",
//!       "createdAt" : "2022-08-31T01:40:56.235Z"
//!     }
//!   }
//! ]
//! ```

use chrono::{DateTime, Utc};
use derive_more::Display;
use serde::{Deserialize, Serialize};

use crate::convert;

/// ## Example
///
/// ```
/// use chrono::{DateTime, NaiveDateTime, Utc};
///
/// use twitter_archive::convert::{created_at, date_time_iso_8601};
///
/// use twitter_archive::structs::twitter_circle::TwitterCircleObject;
///
/// let created_at_string = "2022-08-31T01:40:56.235Z";
/// let created_at_native_time = NaiveDateTime::parse_from_str(&created_at_string, date_time_iso_8601::FORMAT).unwrap();
/// let created_at_date_time = DateTime::<Utc>::from_naive_utc_and_offset(created_at_native_time, Utc);
///
/// let json = format!(r#"{{
///   "twitterCircle": {{
///     "id": "1564790306968592384",
///     "ownerUserId": "435455769",
///     "createdAt": "{created_at_string}"
///   }}
/// }}"#);
///
/// let data: TwitterCircleObject = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.twitter_circle.id, "1564790306968592384");
/// assert_eq!(data.twitter_circle.owner_user_id, "435455769");
/// assert_eq!(data.twitter_circle.created_at, created_at_date_time);
///
/// // Re-serialize is equivalent to original data
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct TwitterCircleObject {
	/// Why they wrapped a list of TwitterCircle data within unnecessary object label is anyone's guess
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "twitterCircle": {
	///     "id": "1564790306968592384",
	///     "ownerUserId": "435455769",
	///     "createdAt": "2022-08-31T01:40:56.235Z"
	///   }
	/// }
	/// ```
	pub twitter_circle: TwitterCircle,
}

/// ## Example
///
/// ```
/// use chrono::{DateTime, NaiveDateTime, Utc};
///
/// use twitter_archive::convert::{created_at, date_time_iso_8601};
///
/// use twitter_archive::structs::twitter_circle::TwitterCircle;
///
/// let created_at_string = "2022-08-31T01:40:56.235Z";
/// let created_at_native_time = NaiveDateTime::parse_from_str(&created_at_string, date_time_iso_8601::FORMAT).unwrap();
/// let created_at_date_time = DateTime::<Utc>::from_naive_utc_and_offset(created_at_native_time, Utc);
///
/// let json = format!(r#"{{
///   "id": "1564790306968592384",
///   "ownerUserId": "435455769",
///   "createdAt": "{created_at_string}"
/// }}"#);
///
/// let data: TwitterCircle = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.id, "1564790306968592384");
/// assert_eq!(data.owner_user_id, "435455769");
/// assert_eq!(data.created_at, created_at_date_time);
///
/// // Re-serialize is equivalent to original data
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct TwitterCircle {
	/// Possibly unique ID for Circle
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "id": "1564790306968592384" }
	/// ```
	pub id: String,

	/// URL formats;
	///
	/// - Desktop: https://twitter.com/i/user/{owner_user_id}
	/// - Mobile: https://mobile.twitter.com/i/user/{owner_user_id}
	///
	/// > Note; does **not** work if not logged-in.  Thanks be to Mr. Musk !-D
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "ownerUserId": "435455769" }
	/// ```
	pub owner_user_id: String,

	/// Date time-stamp of when Circle was originally tweeted
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "createdAt": "2022-08-31T01:40:56.235Z" }
	/// ```
	#[serde(with = "convert::date_time_iso_8601")]
	pub created_at: DateTime<Utc>,
}
