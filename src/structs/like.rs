#!/usr/bin/env rust

//! Tweeter archives as of 2023-08-31 have private data found under;
//!
//!   twitter-<DATE>-<UID>.zip:data/like.js
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
//! use twitter_archive::structs::like;
//!
//! fn main() {
//!     let input_file = "~/Downloads/twitter-archive.zip";
//!
//!     let file_descriptor = fs::File::open(input_file).expect("Unable to read --input-file");
//!     let mut zip_archive = ZipArchive::new(file_descriptor).unwrap();
//!     let mut zip_file = zip_archive.by_name("data/like.js").unwrap();
//!     let mut buff = String::new();
//!     zip_file.read_to_string(&mut buff).unwrap();
//!
//!     let json = buff.replacen("window.YTD.like.part0 = ", "", 1);
//!     let data: Vec<like::LikeObject> = serde_json::from_str(&json).expect("Unable to parse");
//!
//!     for (index, object) in data.iter().enumerate() {
//!         /* Do stuff with each `niDeviceResponse` entry */
//!         println!("Lists member index: {index}");
//!         println!("Tweet ID: {}", object.like.tweet_id);
//!         println!("Expanded URL: {}", object.like.expanded_url);
//!
//!         if let Some(full_text) = &object.like.full_text {
//!             println!("vvv Full text\n{}\n^^^ Full text", full_text);
//!         }
//!     }
//! }
//! ```
//!
//! ## Example `twitter-<DATE>-<UID>.zip:data/like.js` content
//!
//! ```javascript
//! window.YTD.like.part0 = [
//!   {
//!     "like" : {
//!       "tweetId" : "1697051672621597026",
//!       "fullText" : "https://t.co/IaCJlkaweW",
//!       "expandedUrl" : "https://twitter.com/i/web/status/1697051672621597026"
//!     }
//!   }
//! ]
//! ```

use derive_more::Display;
use serde::{Deserialize, Serialize};

/// ## Example
///
/// ```
/// use twitter_archive::structs::like::LikeObject;
///
/// let json = r#"{
///   "like": {
///     "tweetId": "1697051672621597026",
///     "fullText": "https://t.co/IaCJlkaweW",
///     "expandedUrl": "https://twitter.com/i/web/status/1697051672621597026"
///   }
/// }"#;
///
/// let data: LikeObject = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.like.tweet_id, "1697051672621597026");
/// assert_eq!(data.like.full_text.clone().unwrap(), "https://t.co/IaCJlkaweW");
/// assert_eq!(data.like.expanded_url, "https://twitter.com/i/web/status/1697051672621597026");
///
/// // Re-serialize is equivalent to original data
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct LikeObject {
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "like": {
	///     "tweetId": "1697051672621597026",
	///     "fullText": "https://t.co/IaCJlkaweW",
	///     "expandedUrl": "https://twitter.com/i/web/status/1697051672621597026"
	///   }
	/// }
	/// ```
	pub like: Like,
}

/// ## Example
///
/// ```
/// use twitter_archive::structs::like::Like;
///
/// let json = r#"{
///   "tweetId": "1697051672621597026",
///   "fullText": "https://t.co/IaCJlkaweW",
///   "expandedUrl": "https://twitter.com/i/web/status/1697051672621597026"
/// }"#;
///
/// let data: Like = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.tweet_id, "1697051672621597026");
/// assert_eq!(data.full_text.clone().unwrap(), "https://t.co/IaCJlkaweW");
/// assert_eq!(data.expanded_url, "https://twitter.com/i/web/status/1697051672621597026");
///
/// // Re-serialize is equivalent to original data
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct Like {
	/// ## Example JSON data
	///
	/// ```json
	/// { "tweetId": "1697051672621597026" }
	/// ```
	pub tweet_id: String,

	/// Property possibly may not exist if;
	///
	/// - Tweeter deletes account
	/// - Tweeter deletes post
	/// - Tweeter makes posts private
	/// - Tweeter blocks archiving account
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "fullText": "https://t.co/IaCJlkaweW" }
	/// ```
	#[serde(skip_serializing_if = "Option::is_none")]
	pub full_text: Option<String>,

	/// ## Example JSON data
	///
	/// ```json
	/// { "expandedUrl": "https://twitter.com/i/web/status/1697051672621597026" }
	/// ```
	pub expanded_url: String,
}
