#!/usr/bin/env rust

//! Tweeter archives as of 2023-08-31 have private data found under;
//!
//!   twitter-<DATE>-<UID>.zip:data/lists-member.js
//!
//! ## Warnings
//!
//! - `.[].<KEY_NAME>.LocationHistory` data structure is subject to future changes
//!
//! ## Example file reader
//!
//! ```no_build
//! use std::io::Read;
//! use std::{fs, path};
//! use zip::read::ZipArchive;
//!
//! use twitter_archive::structs::lists_member;
//!
//! fn main() {
//!     let input_file = "~/Downloads/twitter-archive.zip";
//!
//!     let file_descriptor = fs::File::open(input_file).expect("Unable to read --input-file");
//!     let mut zip_archive = ZipArchive::new(file_descriptor).unwrap();
//!     let mut zip_file = zip_archive.by_name("data/lists-member.js").unwrap();
//!     let mut buff = String::new();
//!     zip_file.read_to_string(&mut buff).unwrap();
//!
//!     let json = buff.replacen("window.YTD.lists_member.part0 = ", "", 1);
//!     let data: Vec<lists_member::UserListInfoObject> = serde_json::from_str(&json).expect("Unable to parse");
//!
//!     for (index, object) in data.iter().enumerate() {
//!         /* Do stuff with each `niDeviceResponse` entry */
//!         println!("Lists member index: {index}");
//!         println!("URL: {}", object.user_list_info.url);
//!     }
//! }
//! ```
//!
//! ## Example `twitter-<DATE>-<UID>.zip:data/lists-member.js` content
//!
//! ```javascript
//! window.YTD.lists_member.part0 = [
//!   {
//!     "userListInfo" : {
//!       "url" : "https://twitter.com/M16229Myers/lists/1696117177802211514"
//!     }
//!   },
//!   {
//!     "userListInfo" : {
//!       "url" : "https://twitter.com/R0oTk1t/lists/1572592337959944198"
//!     }
//!   }
//! ]
//! ```

use derive_more::Display;
use serde::{Deserialize, Serialize};

/// ## Example
///
/// ```
/// use twitter_archive::structs::lists_member::UserListInfoObject;
///
/// let json = r#"{
///   "userListInfo": {
///     "url": "https://twitter.com/R0oTk1t/lists/1572592337959944198"
///   }
/// }"#;
///
/// let data: UserListInfoObject = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.user_list_info.url, "https://twitter.com/R0oTk1t/lists/1572592337959944198");
///
/// // Re-serialize is equivalent to original data
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct UserListInfoObject {
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "userListInfo": {
	///     "url": "https://twitter.com/R0oTk1t/lists/1572592337959944198"
	///   }
	/// }
	/// ```
	pub user_list_info: UserListInfo,
}

/// ## Example
///
/// ```
/// use twitter_archive::structs::lists_member::UserListInfo;
///
/// let json = r#"{
///   "url": "https://twitter.com/R0oTk1t/lists/1572592337959944198"
/// }"#;
///
/// let data: UserListInfo = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.url, "https://twitter.com/R0oTk1t/lists/1572592337959944198");
///
/// // Re-serialize is equivalent to original data
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
pub struct UserListInfo {
	/// ## Example JSON data
	///
	/// ```json
	/// { "url": "https://twitter.com/R0oTk1t/lists/1572592337959944198" }
	/// ```
	pub url: String,
}
