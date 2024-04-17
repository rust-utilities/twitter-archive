#!/usr/bin/env rust

//! Tweeter archives as of 2023-08-31 have private blocking data found under;
//!
//!   twitter-<DATE>-<UID>.zip:data/account-timezone.js
//!
//! ## Example file reader for `twitter-<DATE>-<UID>.zip:data/account-timezone.js`
//!
//! ```no_build
//! use std::io::Read;
//! use std::{fs, path};
//! use zip::read::ZipArchive;
//!
//! use twitter_archive::structs::account_timezone;
//!
//! fn main() {
//!     let input_file = "~/Downloads/twitter-archive.zip";
//!
//!     let file_descriptor = fs::File::open(input_file).expect("Unable to read --input-file");
//!     let mut zip_archive = ZipArchive::new(file_descriptor).unwrap();
//!     let mut zip_file = zip_archive.by_name("data/account-timezone.js").unwrap();
//!     let mut buff = String::new();
//!     zip_file.read_to_string(&mut buff).unwrap();
//!
//!     let json = buff.replacen("window.YTD.account_timezone.part0 = ", "", 1);
//!     let data: Vec<account_timezone::AccountTimezoneObject> = serde_json::from_str(&json).expect("Unable to parse");
//!
//!     for (index, object) in data.iter().enumerate() {
//!         /* Do stuff with each advertisement */
//!         println!("Account index: {index}");
//!         println!("Account ID: {}", object.account_timezone.account_id);
//!         println!("Time zone: {}", object.account_timezone.time_zone);
//!     }
//! }
//! ```
//!
//! ## Example `twitter-<DATE>-<UID>.zip:data/account-timezone.js` content
//!
//! ```javascript
//! window.YTD.account_timezone.part0 = [
//!   {
//!     "accountTimezone" : {
//!       "accountId" : "111111111",
//!       "timeZone" : "Arizona",
//!     }
//!   }
//! ]
//! ```

use derive_more::Display;
use serde::{Deserialize, Serialize};

/// ## Example
///
/// ```
/// use twitter_archive::structs::account_timezone::AccountTimezoneObject;
///
/// let json = r#"{
///   "accountTimezone": {
///     "accountId": "111111111",
///     "timeZone": "Arizona"
///   }
/// }"#;
///
/// let data: AccountTimezoneObject = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.account_timezone.account_id, "111111111");
/// assert_eq!(data.account_timezone.time_zone, "Arizona");
///
/// // Re-serialize is equivalent to original data without pretty printing
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct AccountTimezoneObject {
	/// Why they wrapped a list of time zones within unnecessary object label is anyone's guess
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "accountTimezone": {
	///     "accountId": "111111111",
	///     "timeZone": "Arizona"
	///   }
	/// }
	/// ```
	pub account_timezone: AccountTimezone,
}

/// ## Example
///
/// ```
/// use twitter_archive::structs::account_timezone::AccountTimezone;
///
/// let json = r#"{
///   "accountId": "111111111",
///   "timeZone": "Arizona"
/// }"#;
///
/// let data: AccountTimezone = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.account_id, "111111111");
/// assert_eq!(data.time_zone, "Arizona");
///
/// // Re-serialize is equivalent to original data without pretty printing
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct AccountTimezone {
	/// URL formats;
	///
	/// - Desktop: https://twitter.com/i/user/{account_id}
	///
	/// > Note; does **not** work if not logged-in.  Thanks be to Mr. Musk !-D
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "accountId": "111111111" }
	/// ```
	pub account_id: String,

	/// Best guess at account time-zone
	///
	/// TODO: Maybe convert to `enum` in future major version release
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "timeZone": "Arizona" }
	/// ```
	pub time_zone: String,
}
