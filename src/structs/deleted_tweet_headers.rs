#!/usr/bin/env rust

//! Tweeter archives as of 2023-08-31 have public deleted_tweet_headers found under;
//!
//!   twitter-<DATE>-<UID>.zip:data/deleted-tweet-headers.js
//!
//! ## Example file reader
//!
//! ```no_build
//! use std::io::Read;
//! use std::{fs, path};
//! use zip::read::ZipArchive;
//!
//! use twitter_archive::structs::deleted_tweet_headers;
//!
//! fn main() {
//!     let input_file = "~/Downloads/twitter-archive.zip";
//!
//!     let file_descriptor = fs::File::open(input_file).expect("Unable to read --input-file");
//!     let mut zip_archive = ZipArchive::new(file_descriptor).unwrap();
//!     let mut zip_file = zip_archive.by_name("data/deleted-tweet-headers.js").unwrap();
//!     let mut buff = String::new();
//!     zip_file.read_to_string(&mut buff).unwrap();
//!
//!     let json = buff.replacen("window.YTD.deleted_tweet_headers.part0 = ", "", 1);
//!     let data: Vec<deleted_tweet_headers::TweetObject> = serde_json::from_str(&json).expect("Unable to parse");
//!
//!     for (index, object) in data.iter().enumerate() {
//!         /* Do stuff with each deleted Tweet */
//!         println!("Deleted Tweet index: {index}");
//!         println!("Tweet ID: {}", object.tweet.tweet_id);
//!         println!("User ID: {}", object.tweet.user_id);
//!         println!("Created at: {}", object.tweet.created_at);
//!         println!("Deleted at: {}", object.tweet.deleted_at);
//!     }
//!     assert_eq!(true, false);
//! }
//! ```
//!
//! ## Example `twitter-<DATE>-<UID>.zip:data/deleted-tweet-headers.js` content
//!
//! ```javascript
//! window.YTD.deleted_tweet_headers.part0 = [
//!   {
//!     "tweet" : {
//!       "tweet_id" : "1697011324369178968",
//!       "user_id" : "111111111",
//!       "created_at" : "Wed Aug 30 22:20:03 +0000 2023",
//!       "deleted_at" : "Wed Aug 30 23:20:03 +0000 2023"
//!     }
//!   },
//!   {
//!     "tweet" : {
//!       "tweet_id" : "1696724445891535264",
//!       "user_id" : "111111111",
//!       "created_at" : "Wed Aug 30 03:20:06 +0000 2023",
//!       "deleted_at" : "Wed Aug 30 04:20:06 +0000 2023"
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
/// use twitter_archive::convert::created_at;
///
/// use twitter_archive::structs::deleted_tweet_headers::TweetObject;
///
/// let created_at_string = "Fri Jan 08 04:54:04 +0000 2021";
/// let created_at_date_time: DateTime<Utc> = DateTime::parse_from_str(&created_at_string, created_at::FORMAT)
///     .unwrap()
///     .into();
///
/// let deleted_at_string = "Fri Jan 08 05:54:04 +0000 2021";
/// let deleted_at_date_time: DateTime<Utc> = DateTime::parse_from_str(&deleted_at_string, created_at::FORMAT)
///     .unwrap()
///     .into();
///
/// let json = format!(r#"{{
///   "tweet" : {{
///     "tweet_id" : "2222222222222222222",
///     "user_id" : "111111111",
///     "created_at" : "{created_at_string}",
///     "deleted_at" : "{deleted_at_string}"
///   }}
/// }}"#);
///
/// let data: TweetObject = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.tweet.tweet_id, "2222222222222222222");
/// assert_eq!(data.tweet.user_id, "111111111");
/// assert_eq!(data.tweet.created_at, created_at_date_time);
/// assert_eq!(data.tweet.deleted_at, deleted_at_date_time);
///
/// // Re-serialize is equivalent to original data without pretty printing
/// // assert_eq!(serde_json::to_string(&data).unwrap(), json.replace("\n", "").replace(" ", ""));
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
pub struct TweetObject {
	/// Why they wrapped a list of Tweets within unnecessary object label is anyone's guess
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "tweet" : {
	///     "tweet_id" : "2222222222222222222",
	///     "user_id" : "111111111",
	///     "created_at" : "Fri Jan 08 04:54:04 +0000 2021",
	///     "deleted_at" : "Fri Jan 08 05:54:04 +0000 2021"
	///   }
	/// }
	/// ```
	pub tweet: Tweet,
}

/// ## Example
///
/// ```
/// use chrono::{DateTime, NaiveDateTime, Utc};
///
/// use twitter_archive::convert::created_at;
///
/// use twitter_archive::structs::deleted_tweet_headers::Tweet;
///
/// let created_at_string = "Fri Jan 08 04:54:04 +0000 2021";
/// let created_at_date_time: DateTime<Utc> = DateTime::parse_from_str(&created_at_string, created_at::FORMAT)
///     .unwrap()
///     .into();
///
/// let deleted_at_string = "Fri Jan 08 05:54:04 +0000 2021";
/// let deleted_at_date_time: DateTime<Utc> = DateTime::parse_from_str(&deleted_at_string, created_at::FORMAT)
///     .unwrap()
///     .into();
///
/// let json = format!(r#"{{
///   "tweet_id" : "2222222222222222222",
///   "user_id" : "111111111",
///   "created_at" : "{created_at_string}",
///   "deleted_at" : "{deleted_at_string}"
/// }}"#);
///
/// let data: Tweet = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.tweet_id, "2222222222222222222");
/// assert_eq!(data.user_id, "111111111");
/// assert_eq!(data.created_at, created_at_date_time);
/// assert_eq!(data.deleted_at, deleted_at_date_time);
///
/// // Re-serialize is equivalent to original data without pretty printing
/// // assert_eq!(serde_json::to_string(&data).unwrap(), json.replace("\n", "").replace(" ", ""));
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
pub struct Tweet {
	/// URL formats;
	///
	/// - Desktop: https://twitter.com/i/web/status/{tweet_ids}
	/// - Mobile: https://mobile.twitter.com/i/web/status/{tweet_ids}
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "tweet_id" : "2222222222222222222" }
	/// ```
	pub tweet_id: String,

	/// URL formats;
	///
	/// - Desktop: https://twitter.com/i/user/{user_id}
	///
	/// > Note; does **not** work if not logged-in.  Thanks be to Mr. Musk !-D
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "user_id" : "111111111" }
	/// ```
	pub user_id: String,

	/// Date time-stamp of when Tweet was originally tweeted
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "created_at" : "Fri Jan 08 04:54:04 +0000 2021" }
	/// ```
	#[serde(with = "convert::created_at")]
	pub created_at: DateTime<Utc>,

	/// Date time-stamp of when Tweet was deleted
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "deleted_at" : "Fri Jan 08 05:54:04 +0000 2021" }
	/// ```
	#[serde(with = "convert::created_at")]
	pub deleted_at: DateTime<Utc>,
}
