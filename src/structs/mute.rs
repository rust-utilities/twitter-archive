#!/usr/bin/env rust

//! Tweeter archives as of 2023-08-31 have private data found under;
//!
//!   twitter-<DATE>-<UID>.zip:data/mute.js
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
//! use twitter_archive::structs::mute;
//!
//! fn main() {
//!     let input_file = "~/Downloads/twitter-archive.zip";
//!
//!     let file_descriptor = fs::File::open(input_file).expect("Unable to read --input-file");
//!     let mut zip_archive = ZipArchive::new(file_descriptor).unwrap();
//!     let mut zip_file = zip_archive.by_name("data/mute.js").unwrap();
//!     let mut buff = String::new();
//!     zip_file.read_to_string(&mut buff).unwrap();
//!
//!     let json = buff.replacen("window.YTD.mute.part0 = ", "", 1);
//!     let data: Vec<mute::MutingObject> = serde_json::from_str(&json).expect("Unable to parse");
//!
//!     for (index, object) in data.iter().enumerate() {
//!         /* Do stuff with each `niDeviceResponse` entry */
//!         println!("Muting index: {index}");
//!         println!("Account ID: {}", object.muting.account_id);
//!         println!("User Link: {}", object.muting.user_link);
//!     }
//! }
//! ```
//!
//! ## Example `twitter-<DATE>-<UID>.zip:data/mute.js` content
//!
//! ```javascript
//! window.YTD.mute.part0 = [
//!   {
//!     "muting" : {
//!       "accountId" : "3769699761",
//!       "userLink" : "https://twitter.com/intent/user?user_id=3769699761"
//!     }
//!   },
//!   {
//!     "muting" : {
//!       "accountId" : "272825223",
//!       "userLink" : "https://twitter.com/intent/user?user_id=272825223"
//!     }
//!   }
//! ]
//! ```

use derive_more::Display;
use serde::{Deserialize, Serialize};

/// ## Example
///
/// ```
/// use twitter_archive::structs::mute::MutingObject;
///
/// let json = r#"{
///   "muting": {
///     "accountId": "3769699761",
///     "userLink": "https://twitter.com/intent/user?user_id=3769699761"
///   }
/// }"#;
///
/// let data: MutingObject = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.muting.account_id, "3769699761");
/// assert_eq!(data.muting.user_link, "https://twitter.com/intent/user?user_id=3769699761");
///
/// // Re-serialize is equivalent to original data
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct MutingObject {
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "muting": {
	///     "accountId": "3769699761",
	///     "userLink": "https://twitter.com/intent/user?user_id=3769699761"
	///   }
	/// }
	/// ```
	pub muting: Muting,
}

/// ## Example
///
/// ```
/// use twitter_archive::structs::mute::Muting;
///
/// let json = r#"{
///   "accountId": "3769699761",
///   "userLink": "https://twitter.com/intent/user?user_id=3769699761"
/// }"#;
///
/// let data: Muting = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.account_id, "3769699761");
/// assert_eq!(data.user_link, "https://twitter.com/intent/user?user_id=3769699761");
///
/// // Re-serialize is equivalent to original data
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct Muting {
	/// ## Example JSON data
	///
	/// ```json
	/// { "accountId": "3769699761" }
	/// ```
	pub account_id: String,

	/// ## Example JSON data
	///
	/// ```json
	/// { "userLink": "https://twitter.com/intent/user?user_id=3769699761" }
	/// ```
	pub user_link: String,
}
