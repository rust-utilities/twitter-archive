#!/usr/bin/env rust

//! Tweeter archives as of 2023-08-31 have private data found under;
//!
//!   twitter-<DATE>-<UID>.zip:data/following.js
//!
//! ## Example file reader
//!
//! ```no_build
//! use std::io::Read;
//! use std::{fs, path};
//! use zip::read::ZipArchive;
//!
//! use twitter_archive::structs::following;
//!
//! fn main() {
//!     let input_file = "~/Downloads/twitter-archive.zip";
//!
//!     let file_descriptor = fs::File::open(input_file).expect("Unable to read --input-file");
//!     let mut zip_archive = ZipArchive::new(file_descriptor).unwrap();
//!     let mut zip_file = zip_archive.by_name("data/following.js").unwrap();
//!     let mut buff = String::new();
//!     zip_file.read_to_string(&mut buff).unwrap();
//!
//!     let json = buff.replacen("window.YTD.following.part0 = ", "", 1);
//!     let data: Vec<following::FollowingObject> = serde_json::from_str(&json).expect("Unable to parse");
//!
//!     for (index, object) in data.iter().enumerate() {
//!         /* Do stuff with each `RegisteredDevices` entry */
//!         println!("IP audit index: {index}");
//!         println!("Account ID: {}", object.following.account_id);
//!         println!("User link: {}", object.following.user_link);
//!     }
//! }
//! ```
//!
//! ## Example `twitter-<DATE>-<UID>.zip:data/following.js` content
//!
//! ```javascript
//! window.YTD.following.part0 = [
//!   {
//!     "following" : {
//!       "accountId" : "1111111111111111111",
//!       "userLink" : "https://twitter.com/intent/user?user_id=1111111111111111111"
//!     }
//!   }
//! ]
//! ```

use derive_more::Display;
use serde::{Deserialize, Serialize};

use crate::structs::follow::Follow;

/// ## Example
///
/// ```
/// use twitter_archive::structs::following::FollowingObject;
///
/// let json = r#"{
///   "following": {
///     "accountId": "1111111111111111111",
///     "userLink": "https://twitter.com/intent/user?user_id=1111111111111111111"
///   }
/// }"#;
///
/// let data: FollowingObject = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.following.account_id, "1111111111111111111");
/// assert_eq!(data.following.user_link, "https://twitter.com/intent/user?user_id=1111111111111111111");
///
/// // Re-serialize is equivalent to original data
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
pub struct FollowingObject {
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "following": {
	///     "accountId": "1111111111111111111",
	///     "userLink": "https://twitter.com/intent/user?user_id=1111111111111111111"
	///   }
	/// }
	/// ```
	pub following: Follow,
}
