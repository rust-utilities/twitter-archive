#!/usr/bin/env rust

//! Tweeter archives as of 2023-08-31 have private blocking data found under;
//!
//!   twitter-<DATE>-<UID>.zip:data/block.js
//!
//! ## Example file reader
//!
//! ```no_build
//! use std::io::Read;
//! use std::{fs, path};
//! use zip::read::ZipArchive;
//!
//! use twitter_archive::structs::block;
//!
//! fn main() {
//!     let input_file = "~/Downloads/twitter-archive.zip";
//!
//!     let file_descriptor = fs::File::open(input_file).expect("Unable to read --input-file");
//!     let mut zip_archive = ZipArchive::new(file_descriptor).unwrap();
//!     let mut zip_file = zip_archive.by_name("data/block.js").unwrap();
//!     let mut buff = String::new();
//!     zip_file.read_to_string(&mut buff).unwrap();
//!
//!     let json = buff.replacen("window.YTD.block.part0 = ", "", 1);
//!     let data: Vec<block::BlockingObject> = serde_json::from_str(&json).expect("Unable to parse");
//!
//!     for (index, object) in data.iter().enumerate() {
//!         /* Do stuff with each deleted Tweet */
//!         println!("Blocking index: {index}");
//!         println!("Account ID: {}", object.blocking.account_id);
//!         println!("User link: {}", object.blocking.user_link);
//!     }
//! }
//! ```
//!
//! ## Example `twitter-<DATE>-<UID>.zip:data/block.js` content
//!
//! ```javascript
//! window.YTD.block.part0 = [
//!   "blocking" : {
//!     "accountId" : "3333333333333333333",
//!     "userLink" : "https://twitter.com/intent/user?user_id=3333333333333333333"
//!   }
//! ]
//! ```

use derive_more::Display;
use serde::{Deserialize, Serialize};

/// ## Example
///
/// ```
/// use chrono::{DateTime, NaiveDateTime, Utc};
///
/// use twitter_archive::convert::date_time_iso_8601::FORMAT;
///
/// use twitter_archive::structs::block::BlockingObject;
///
/// let created_at_string = "2020-01-20T21:42:09.068Z";
/// let created_at_native_time = NaiveDateTime::parse_from_str(&created_at_string, FORMAT).unwrap();
/// let created_at_date_time = DateTime::<Utc>::from_naive_utc_and_offset(created_at_native_time, Utc);
///
/// let json = r#"{
///   "blocking" : {
///     "accountId" : "3333333333333333333",
///     "userLink" : "https://twitter.com/intent/user?user_id=3333333333333333333"
///   }
/// }"#;
///
/// let data: BlockingObject = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.blocking.account_id, "3333333333333333333");
/// assert_eq!(data.blocking.user_link, "https://twitter.com/intent/user?user_id=3333333333333333333");
///
/// // Re-serialize is equivalent to original data without pretty printing
/// // assert_eq!(serde_json::to_string(&data).unwrap(), json.replace("\n", "").replace(" ", ""));
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct BlockingObject {
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "blocking" : {
	///     "accountId" : "3333333333333333333",
	///     "userLink" : "https://twitter.com/intent/user?user_id=3333333333333333333"
	///   }
	/// }
	/// ```
	pub blocking: Blocking,
}

/// ## Example
///
/// ```
/// use chrono::{DateTime, NaiveDateTime, Utc};
///
/// use twitter_archive::convert::date_time_iso_8601::FORMAT;
///
/// use twitter_archive::structs::block::Blocking;
///
/// let created_at_string = "2020-01-20T21:42:09.068Z";
/// let created_at_native_time = NaiveDateTime::parse_from_str(&created_at_string, FORMAT).unwrap();
/// let created_at_date_time = DateTime::<Utc>::from_naive_utc_and_offset(created_at_native_time, Utc);
///
/// let json = r#"{
///   "accountId" : "3333333333333333333",
///   "userLink" : "https://twitter.com/intent/user?user_id=3333333333333333333"
/// }"#;
///
/// let data: Blocking = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.account_id, "3333333333333333333");
/// assert_eq!(data.user_link, "https://twitter.com/intent/user?user_id=3333333333333333333");
///
/// // Re-serialize is equivalent to original data without pretty printing
/// assert_eq!(serde_json::to_string(&data).unwrap(), json.replace("\n", "").replace(" ", ""));
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct Blocking {
	/// URL formats;
	///
	/// - Desktop: `https://twitter.com/i/user/{account_id}`
	///
	/// > Note; does **not** work if not logged-in.  Thanks be to Mr. Musk !-D
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "accountId" : "3333333333333333333" }
	/// ```
	pub account_id: String,

	/// Alternate way of directly linking to account by ID, with added side effect of prompting
	/// client to follow profile regardless of following status
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "userLink" : "https://twitter.com/intent/user?user_id=3333333333333333333" }
	/// ```
	pub user_link: String,
}
