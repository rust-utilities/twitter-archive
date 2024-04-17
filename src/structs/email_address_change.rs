#!/usr/bin/env rust

//! Tweeter archives as of 2023-08-31 have private data found under;
//!
//!   twitter-<DATE>-<UID>.zip:data/email-address-change.js
//!
//! ## Example file reader
//!
//! ```no_build
//! use std::io::Read;
//! use std::{fs, path};
//! use zip::read::ZipArchive;
//!
//! use twitter_archive::structs::email_address_change;
//!
//! fn main() {
//!     let input_file = "~/Downloads/twitter-archive.zip";
//!
//!     let file_descriptor = fs::File::open(input_file).expect("Unable to read --input-file");
//!     let mut zip_archive = ZipArchive::new(file_descriptor).unwrap();
//!     let mut zip_file = zip_archive.by_name("data/email-address-change.js").unwrap();
//!     let mut buff = String::new();
//!     zip_file.read_to_string(&mut buff).unwrap();
//!
//!     let json = buff.replacen("window.YTD.email_address_change.part0 = ", "", 1);
//!     let data: Vec<email_address_change::EmailAddressChangeObject> = serde_json::from_str(&json).expect("Unable to parse");
//!
//!     for (index, object) in data.iter().enumerate() {
//!         /* Do stuff with each `RegisteredDevices` entry */
//!         println!("IP audit index: {index}");
//!         println!("Account ID: {}", object.email_address_change.account_id);
//!         println!("Changed at: {}", object.email_address_change.email_change.changed_at);
//!         println!("Changed to: {}", object.email_address_change.email_change.changed_to);
//!     }
//! }
//! ```
//!
//! ## Example `twitter-<DATE>-<UID>.zip:data/email-address-change.js` content
//!
//! ```javascript
//! window.YTD.email_address_change.part0 = [
//!   {
//!     "emailAddressChange" : {
//!       "accountId" : "12345",
//!       "emailChange" : {
//!         "changedAt" : "2023-08-12T17:10:37.000Z",
//!         "changedTo" : "someone@example.com"
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
/// use twitter_archive::convert::date_time_iso_8601;
///
/// use twitter_archive::structs::email_address_change::EmailAddressChangeObject;
///
/// let changed_at_string = "2023-08-12T17:10:37.000Z";
/// let changed_at_native_time = NaiveDateTime::parse_from_str(&changed_at_string, date_time_iso_8601::FORMAT).unwrap();
/// let changed_at_date_time = DateTime::<Utc>::from_naive_utc_and_offset(changed_at_native_time, Utc);
///
/// let json = format!(r#"{{
///   "emailAddressChange": {{
///     "accountId": "12345",
///     "emailChange": {{
///       "changedAt": "{changed_at_string}",
///       "changedTo": "someone@example.com"
///     }}
///   }}
/// }}"#);
///
/// let data: EmailAddressChangeObject = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.email_address_change.account_id, "12345");
/// assert_eq!(data.email_address_change.email_change.changed_at, changed_at_date_time);
/// assert_eq!(data.email_address_change.email_change.changed_to, "someone@example.com");
///
/// // Re-serialize is equivalent to original data
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct EmailAddressChangeObject {
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "emailAddressChange": {
	///     "accountId": "12345",
	///     "emailChange": {
	///       "changedAt": "2023-08-12T17:10:37.000Z",
	///       "changedTo": "someone@example.com"
	///     }
	///   }
	/// }"#);
	/// ```
	pub email_address_change: EmailAddressChange,
}

/// ## Example
///
/// ```
/// use chrono::{DateTime, NaiveDateTime, Utc};
///
/// use twitter_archive::convert::date_time_iso_8601;
///
/// use twitter_archive::structs::email_address_change::EmailAddressChange;
///
/// let changed_at_string = "2023-08-12T17:10:37.000Z";
/// let changed_at_native_time = NaiveDateTime::parse_from_str(&changed_at_string, date_time_iso_8601::FORMAT).unwrap();
/// let changed_at_date_time = DateTime::<Utc>::from_naive_utc_and_offset(changed_at_native_time, Utc);
///
/// let json = format!(r#"{{
///   "accountId": "12345",
///   "emailChange": {{
///     "changedAt": "{changed_at_string}",
///     "changedTo": "someone@example.com"
///   }}
/// }}"#);
///
/// let data: EmailAddressChange = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.account_id, "12345");
/// assert_eq!(data.email_change.changed_at, changed_at_date_time);
/// assert_eq!(data.email_change.changed_to, "someone@example.com");
///
/// // Re-serialize is equivalent to original data
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct EmailAddressChange {
	/// ## Example JSON data
	///
	/// ```json
	/// { "accountId": "12345" }
	/// ```
	pub account_id: String,

	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "emailChange": {
	///     "changedAt": "2023-08-12T17:10:37.000Z",
	///     "changedTo": "someone@example.com"
	///   }
	/// }
	/// ```
	pub email_change: EmailChange,
}

/// ## Example
///
/// ```
/// use chrono::{DateTime, NaiveDateTime, Utc};
///
/// use twitter_archive::convert::date_time_iso_8601;
///
/// use twitter_archive::structs::email_address_change::EmailChange;
///
/// let changed_at_string = "2023-08-12T17:10:37.000Z";
/// let changed_at_native_time = NaiveDateTime::parse_from_str(&changed_at_string, date_time_iso_8601::FORMAT).unwrap();
/// let changed_at_date_time = DateTime::<Utc>::from_naive_utc_and_offset(changed_at_native_time, Utc);
///
/// let json = format!(r#"{{
///   "changedAt": "{changed_at_string}",
///   "changedTo": "someone@example.com"
/// }}"#);
///
/// let data: EmailChange = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.changed_at, changed_at_date_time);
/// assert_eq!(data.changed_to, "someone@example.com");
///
/// // Re-serialize is equivalent to original data
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct EmailChange {
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
	/// { "changedTo": "someone@example.com" }
	/// ```
	pub changed_to: String,
}
