#!/usr/bin/env rust

//! Tweeter archives as of 2023-08-31 have private data found under;
//!
//!   twitter-<DATE>-<UID>.zip:data/device-token.js
//!
//! ## Example file reader
//!
//! ```no_build
//! use std::io::Read;
//! use std::{fs, path};
//! use zip::read::ZipArchive;
//!
//! use twitter_archive::structs::device_token;
//!
//! fn main() {
//!     let input_file = "~/Downloads/twitter-archive.zip";
//!
//!     let file_descriptor = fs::File::open(input_file).expect("Unable to read --input-file");
//!     let mut zip_archive = ZipArchive::new(file_descriptor).unwrap();
//!     let mut zip_file = zip_archive.by_name("data/device-token.js").unwrap();
//!     let mut buff = String::new();
//!     zip_file.read_to_string(&mut buff).unwrap();
//!
//!     let json = buff.replacen("window.YTD.device_token.part0 = ", "", 1);
//!     let data: Vec<device_token::DeviceTokenObject> = serde_json::from_str(&json).expect("Unable to parse");
//!
//!     for (index, object) in data.iter().enumerate() {
//!         /* Do stuff with each `DeviceTokenObject` entry */
//!         println!("Device token index: {index}");
//!         println!("Application ID: {}", object.device_token.client_application_id);
//!         println!("Token: {}", object.device_token.token);
//!         println!("Created at: {}", object.device_token.created_at);
//!         println!("Last seen at: {}", object.device_token.last_seen_at);
//!         println!("Application name: {}", object.device_token.client_application_name);
//!     }
//! }
//! ```
//!
//! ## Example `twitter-<DATE>-<UID>.zip:data/device-token.js` content
//!
//! ```javascript
//! window.YTD.device_token.part0 = [
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
/// use twitter_archive::convert::date_time_iso_8601;
///
/// use twitter_archive::structs::device_token::DeviceTokenObject;
///
/// let created_at_string = "2023-08-12T17:10:37.000Z";
/// let created_at_native_time = NaiveDateTime::parse_from_str(&created_at_string, date_time_iso_8601::FORMAT).unwrap();
/// let created_at_date_time = DateTime::<Utc>::from_naive_utc_and_offset(created_at_native_time, Utc);
///
/// let last_seen_string = "2023-08-12T17:10:37.000Z";
/// let last_seen_native_time = NaiveDateTime::parse_from_str(&created_at_string, date_time_iso_8601::FORMAT).unwrap();
/// let last_seen_date_time = DateTime::<Utc>::from_naive_utc_and_offset(created_at_native_time, Utc);
///
/// let json = format!(r#"{{
///   "deviceToken": {{
///     "clientApplicationId": "1111111",
///     "token": "DEADBEEF",
///     "createdAt": "{created_at_string}",
///     "lastSeenAt": "{last_seen_string}",
///     "clientApplicationName": "Twitter Web App (Twitter. Inc)"
///   }}
/// }}"#);
///
/// let data: DeviceTokenObject = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.device_token.client_application_id, "1111111");
/// assert_eq!(data.device_token.token, "DEADBEEF");
/// assert_eq!(data.device_token.created_at, created_at_date_time);
/// assert_eq!(data.device_token.last_seen_at, last_seen_date_time);
/// assert_eq!(data.device_token.client_application_name, "Twitter Web App (Twitter. Inc)");
///
/// // Re-serialize is equivalent to original data without pretty printing
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct DeviceTokenObject {
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "deviceToken": {
	///     "clientApplicationId": "1111111",
	///     "token": "DEADBEEF",
	///     "createdAt": "2023-08-12T17:10:37.000Z",
	///     "lastSeenAt": "2023-08-12T17:10:37.000Z",
	///     "clientApplicationName": "Twitter Web App (Twitter. Inc)"
	///   }
	/// }
	/// ```
	pub device_token: DeviceToken,
}

/// ## Example
///
/// ```
/// use chrono::{DateTime, NaiveDateTime, Utc};
///
/// use twitter_archive::convert::date_time_iso_8601;
///
/// use twitter_archive::structs::device_token::DeviceToken;
///
/// let created_at_string = "2023-08-12T17:10:37.000Z";
/// let created_at_native_time = NaiveDateTime::parse_from_str(&created_at_string, date_time_iso_8601::FORMAT).unwrap();
/// let created_at_date_time = DateTime::<Utc>::from_naive_utc_and_offset(created_at_native_time, Utc);
///
/// let last_seen_string = "2023-08-12T17:10:37.000Z";
/// let last_seen_native_time = NaiveDateTime::parse_from_str(&created_at_string, date_time_iso_8601::FORMAT).unwrap();
/// let last_seen_date_time = DateTime::<Utc>::from_naive_utc_and_offset(created_at_native_time, Utc);
///
/// let json = format!(r#"{{
///   "clientApplicationId": "1111111",
///   "token": "DEADBEEF",
///   "createdAt": "{created_at_string}",
///   "lastSeenAt": "{last_seen_string}",
///   "clientApplicationName": "Twitter Web App (Twitter. Inc)"
/// }}"#);
///
/// let data: DeviceToken = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.client_application_id, "1111111");
/// assert_eq!(data.token, "DEADBEEF");
/// assert_eq!(data.created_at, created_at_date_time);
/// assert_eq!(data.last_seen_at, last_seen_date_time);
/// assert_eq!(data.client_application_name, "Twitter Web App (Twitter. Inc)");
///
/// // Re-serialize is equivalent to original data without pretty printing
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct DeviceToken {
	/// ## Example JSON data
	///
	/// ```json
	/// { "clientApplicationId": "1111111" }
	/// ```
	pub client_application_id: String,

	/// ## Example JSON data
	///
	/// ```json
	/// { "token": "DEADBEEF" }
	/// ```
	pub token: String,

	/// Date time stamp when DM was created
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "createdAt": "2023-08-12T17:10:37.000Z" }
	/// ```
	#[serde(with = "convert::date_time_iso_8601")]
	pub created_at: DateTime<Utc>,

	/// Date time stamp when DM was created
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "lastSeenAt": "2023-08-12T17:10:37.000Z" }
	/// ```
	#[serde(with = "convert::date_time_iso_8601")]
	pub last_seen_at: DateTime<Utc>,

	/// ## Example JSON data
	///
	/// ```json
	/// { "clientApplicationName": "Twitter Web App (Twitter. Inc)" }
	/// ```
	pub client_application_name: String,
}
