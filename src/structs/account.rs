#!/usr/bin/env rust

//! Tweeter archives as of 2023-08-31 have private blocking data found under;
//!
//!   twitter-<DATE>-<UID>.zip:data/account.js
//!
//! ## Example file reader for `twitter-<DATE>-<UID>.zip:data/account.js`
//!
//! ```no_build
//! use std::io::Read;
//! use std::{fs, path};
//! use zip::read::ZipArchive;
//!
//! use twitter_archive::structs::account;
//!
//! fn main() {
//!     let input_file = "~/Downloads/twitter-archive.zip";
//!
//!     let file_descriptor = fs::File::open(input_file).expect("Unable to read --input-file");
//!     let mut zip_archive = ZipArchive::new(file_descriptor).unwrap();
//!     let mut zip_file = zip_archive.by_name("data/account.js").unwrap();
//!     let mut buff = String::new();
//!     zip_file.read_to_string(&mut buff).unwrap();
//!
//!     let json = buff.replacen("window.YTD.account.part0 = ", "", 1);
//!     let data: Vec<account::AccountObject> = serde_json::from_str(&json).expect("Unable to parse");
//!
//!     for (index, object) in data.iter().enumerate() {
//!         /* Do stuff with each advertisement */
//!         println!("Account index: {index}");
//!         println!("Email: {}", object.account.email);
//!         println!("Created via: {}", object.account.created_via);
//!         println!("Account ID: {}", object.account.account_id);
//!         println!("Created at: {}", object.account.created_at);
//!         println!("Account display name: {}", object.account.account_display_name);
//!     }
//! }
//! ```
//!
//! ## Example `twitter-<DATE>-<UID>.zip:data/account.js` content
//!
//! ```javascript
//! window.YTD.account.part0 = [
//!   {
//!     "account": {
//!       "email": "user@example.com",
//!       "createdVia": "web",
//!       "username": "S0_And_S0",
//!       "accountId": "111111111",
//!       "createdAt": "2023-08-30T23:20:03.000Z",
//!       "accountDisplayName": "S0AndS0.eth"
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
/// use twitter_archive::convert::date_time_iso_8601::FORMAT;
///
/// use twitter_archive::structs::account::AccountObject;
///
/// let created_at_string = "2023-08-30T23:20:03.000Z";
/// let created_at_native_time = NaiveDateTime::parse_from_str(&created_at_string, FORMAT).unwrap();
/// let created_at_date_time = DateTime::<Utc>::from_naive_utc_and_offset(created_at_native_time, Utc);
///
/// let json = format!(r#"{{
///   "account": {{
///     "email": "user@example.com",
///     "createdVia": "web",
///     "username": "S0_And_S0",
///     "accountId": "111111111",
///     "createdAt": "{created_at_string}",
///     "accountDisplayName": "S0AndS0.eth"
///   }}
/// }}"#);
///
/// let data: AccountObject = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.account.email, "user@example.com");
/// assert_eq!(data.account.created_via, "web");
/// assert_eq!(data.account.username, "S0_And_S0");
/// assert_eq!(data.account.account_id, "111111111");
/// assert_eq!(data.account.created_at, created_at_date_time);
/// assert_eq!(data.account.account_display_name, "S0AndS0.eth");
///
/// // Re-serialize is equivalent to original data without pretty printing
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
pub struct AccountObject {
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "account": {
	///     "email": "user@example.com",
	///     "createdVia": "web",
	///     "username": "S0_And_S0",
	///     "accountId": "111111111",
	///     "createdAt": "2023-08-30T23:20:03.000Z",
	///     "accountDisplayName": "S0AndS0.eth"
	///   }
	/// }
	/// ```
	pub account: Account,
}

/// ## Example
///
/// ```
/// use chrono::{DateTime, NaiveDateTime, Utc};
///
/// use twitter_archive::convert::date_time_iso_8601::FORMAT;
///
/// use twitter_archive::structs::account::Account;
///
/// let created_at_string = "2023-08-30T23:20:03.000Z";
/// let created_at_native_time = NaiveDateTime::parse_from_str(&created_at_string, FORMAT).unwrap();
/// let created_at_date_time = DateTime::<Utc>::from_naive_utc_and_offset(created_at_native_time, Utc);
///
/// let json = format!(r#"{{
///   "email": "user@example.com",
///   "createdVia": "web",
///   "username": "S0_And_S0",
///   "accountId": "111111111",
///   "createdAt": "{created_at_string}",
///   "accountDisplayName": "S0AndS0.eth"
/// }}"#);
///
/// let data: Account = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.email, "user@example.com");
/// assert_eq!(data.created_via, "web");
/// assert_eq!(data.username, "S0_And_S0");
/// assert_eq!(data.account_id, "111111111");
/// assert_eq!(data.created_at, created_at_date_time);
/// assert_eq!(data.account_display_name, "S0AndS0.eth");
///
/// // Re-serialize is equivalent to original data without pretty printing
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct Account {
	/// Email address for account
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "email": "user@example.com" }
	/// ```
	pub email: String,

	/// Type of device that created account
	///
	/// TODO: Maybe convert to `enum` in future major version release
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "createdVia": "web" }
	/// ```
	pub created_via: String,

	/// The at-able name of account, e.g. `@{username}` -> `@S0_And_S0`
	///
	/// URL formats;
	///
	/// - Desktop: https://twitter.com/{username}
	///
	/// > Note; does **not** work if not logged-in.  Thanks be to Mr. Musk !-D
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "username": "S0_And_S0" }
	/// ```
	pub username: String,

	/// URL formats;
	///
	/// - Desktop: https://twitter.com/i/user/{username}
	///
	/// > Note; does **not** work if not logged-in.  Thanks be to Mr. Musk !-D
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "accountId": "111111111" }
	/// ```
	pub account_id: String,

	/// Date time stamp of account creation
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "createdAt": "2023-08-30T23:20:03.000Z" }
	/// ```
	#[serde(with = "convert::date_time_iso_8601")]
	pub created_at: DateTime<Utc>,

	/// Displayed to clients and may, for now, be changed via settings page
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "accountDisplayName": "S0AndS0.eth" }
	/// ```
	pub account_display_name: String,
}
