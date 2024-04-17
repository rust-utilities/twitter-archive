#!/usr/bin/env rust

//! Tweeter archives as of 2023-08-31 have private data found under;
//!
//!   twitter-<DATE>-<UID>.zip:data/screen-name-change.js
//!
//! ## Example file reader
//!
//! ```no_build
//! use std::io::Read;
//! use std::{fs, path};
//! use zip::read::ZipArchive;
//!
//! use twitter_archive::structs::screen_name_change;
//!
//! fn main() {
//!     let input_file = "~/Downloads/twitter-archive.zip";
//!
//!     let file_descriptor = fs::File::open(input_file).expect("Unable to read --input-file");
//!     let mut zip_archive = ZipArchive::new(file_descriptor).unwrap();
//!     let mut zip_file = zip_archive.by_name("data/screen-name-change.js").unwrap();
//!     let mut buff = String::new();
//!     zip_file.read_to_string(&mut buff).unwrap();
//!
//!     let json = buff.replacen("window.YTD.screen_name_change.part0 = ", "", 1);
//!     let data: Vec<screen_name_change::ScreenNameChangeObject> = serde_json::from_str(&json).expect("Unable to parse");
//!
//!     for (index, object) in data.iter().enumerate() {
//!         /* Do stuff with each ScreenNameChange */
//!         println!("Screen name change index: {index}");
//!         println!("Account ID: {}", object.screen_name_change.account_id);
//!         println!("Changed At: {}", object.screen_name_change.screen_name_change.changed_at);
//!         println!("Changed From: {}", object.screen_name_change.screen_name_change.changed_from);
//!         println!("Changed To: {}", object.screen_name_change.screen_name_change.changed_to);
//!     }
//! }
//! ```
//!
//! ## Example `twitter-<DATE>-<UID>.zip:data/screen-name-change.js` content
//!
//! ```javascript
//! window.YTD.screen_name_change.part0 = [
//!   {
//!     "screenNameChange" : {
//!       "accountId" : "111111111",
//!       "screenNameChange" : {
//!         "changedAt" : "2023-08-12T17:10:37.000Z",
//!         "changedFrom" : "SomeOneElse",
//!         "changedTo" : "SomeOneNew"
//!       }
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
/// use twitter_archive::structs::screen_name_change::ScreenNameChangeObject;
///
/// let changed_at_string = "2023-08-12T17:10:37.000Z";
/// let changed_at_native_time = NaiveDateTime::parse_from_str(&changed_at_string, date_time_iso_8601::FORMAT).unwrap();
/// let changed_at_date_time = DateTime::<Utc>::from_naive_utc_and_offset(changed_at_native_time, Utc);
///
/// let json = format!(r#"{{
///   "screenNameChange": {{
///     "accountId": "111111111",
///     "screenNameChange": {{
///       "changedAt": "{changed_at_string}",
///       "changedFrom": "SomeOneElse",
///       "changedTo": "SomeOneNew"
///     }}
///   }}
/// }}"#);
///
/// let data: ScreenNameChangeObject = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.screen_name_change.account_id, "111111111");
/// assert_eq!(data.screen_name_change.screen_name_change.changed_at, changed_at_date_time);
/// assert_eq!(data.screen_name_change.screen_name_change.changed_from, "SomeOneElse");
/// assert_eq!(data.screen_name_change.screen_name_change.changed_to, "SomeOneNew");
///
/// // Re-serialize is equivalent to original data
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct ScreenNameChangeObject {
	/// Why they wrapped a list of name changes within unnecessary object label is anyone's guess
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "screenNameChange": {
	///     "accountId": "111111111",
	///     "screenNameChange": {
	///       "changedAt": "2023-08-12T17:10:37.000Z",
	///       "changedFrom": "SomeOneElse",
	///       "changedTo": "SomeOneNew"
	///     }
	///   }
	/// }
	/// ```
	pub screen_name_change: ScreenNameChangeEntry,
}

/// ## Example
///
/// ```
/// use chrono::{DateTime, NaiveDateTime, Utc};
///
/// use twitter_archive::convert::{created_at, date_time_iso_8601};
///
/// use twitter_archive::structs::screen_name_change::ScreenNameChangeEntry;
///
/// let changed_at_string = "2023-08-12T17:10:37.000Z";
/// let changed_at_native_time = NaiveDateTime::parse_from_str(&changed_at_string, date_time_iso_8601::FORMAT).unwrap();
/// let changed_at_date_time = DateTime::<Utc>::from_naive_utc_and_offset(changed_at_native_time, Utc);
///
/// let json = format!(r#"{{
///   "accountId": "111111111",
///   "screenNameChange": {{
///     "changedAt": "{changed_at_string}",
///     "changedFrom": "SomeOneElse",
///     "changedTo": "SomeOneNew"
///   }}
/// }}"#);
///
/// let data: ScreenNameChangeEntry = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.account_id, "111111111");
/// assert_eq!(data.screen_name_change.changed_at, changed_at_date_time);
/// assert_eq!(data.screen_name_change.changed_from, "SomeOneElse");
/// assert_eq!(data.screen_name_change.changed_to, "SomeOneNew");
///
/// // Re-serialize is equivalent to original data
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct ScreenNameChangeEntry {
	/// ## Example JSON data
	///
	/// ```json
	/// { "accountId": "111111111" }
	/// ```
	pub account_id: String,

	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "screenNameChange": {
	///     "changedAt": "2023-08-12T17:10:37.000Z",
	///     "changedFrom": "SomeOneElse",
	///     "changedTo": "SomeOneNew"
	///    }
	/// }
	/// ```
	pub screen_name_change: ScreenNameChange,
}

/// ## Example
///
/// ```
/// use chrono::{DateTime, NaiveDateTime, Utc};
///
/// use twitter_archive::convert::{created_at, date_time_iso_8601};
///
/// use twitter_archive::structs::screen_name_change::ScreenNameChange;
///
/// let changed_at_string = "2023-08-12T17:10:37.000Z";
/// let changed_at_native_time = NaiveDateTime::parse_from_str(&changed_at_string, date_time_iso_8601::FORMAT).unwrap();
/// let changed_at_date_time = DateTime::<Utc>::from_naive_utc_and_offset(changed_at_native_time, Utc);
///
/// let json = format!(r#"{{
///   "changedAt": "{changed_at_string}",
///   "changedFrom": "SomeOneElse",
///   "changedTo": "SomeOneNew"
/// }}"#);
///
/// let data: ScreenNameChange = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.changed_at, changed_at_date_time);
/// assert_eq!(data.changed_from, "SomeOneElse");
/// assert_eq!(data.changed_to, "SomeOneNew");
///
/// // Re-serialize is equivalent to original data
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct ScreenNameChange {
	/// ## Example JSON data
	///
	/// ```json
	/// { "changedAt": "2023-08-12T17:10:37.000Z" }
	/// ```
	#[serde(with = "convert::date_time_iso_8601")]
	pub changed_at: DateTime<Utc>,

	/// ## Example JSON data
	///
	/// ```json
	/// { "changedFrom": "SomeOneElse" }
	/// ```
	pub changed_from: String,

	/// ## Example JSON data
	///
	/// ```json
	/// { "changedTo": "SomeOneNew" }
	/// ```
	pub changed_to: String,
}
