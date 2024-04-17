#!/usr/bin/env rust

//! Tweeter archives as of 2023-08-31 have public tweets found under;
//!
//!   twitter-<DATE>-<UID>.zip:data/tweets.js
//!   twitter-<DATE>-<UID>.zip:data/deleted-tweets.js
//!
//! ## Example file reader for `data/tweets.js`
//!
//! ```no_build
//! use std::io::Read;
//! use std::{fs, path};
//! use zip::read::ZipArchive;
//!
//! use twitter_archive::structs::tweets;
//!
//! fn main() {
//!     let input_file = "~/Downloads/twitter-archive.zip";
//!
//!     let file_descriptor = fs::File::open(input_file).expect("Unable to read --input-file");
//!     let mut zip_archive = ZipArchive::new(file_descriptor).unwrap();
//!     let mut zip_file = zip_archive.by_name("data/tweets.js").unwrap();
//!     let mut buff = String::new();
//!     zip_file.read_to_string(&mut buff).unwrap();
//!
//!     let json = buff.replacen("window.YTD.tweets.part0 = ", "", 1);
//!     let data: Vec<tweets::TweetObject> = serde_json::from_str(&json).expect("Unable to parse");
//!
//!     for (index, object) in data.iter().enumerate() {
//!         /* Do stuff with each Tweet */
//!         println!("Index: {index}");
//!         println!("Created at: {}", object.tweet.created_at);
//!         println!("vvv Content\n{}\n^^^ Content", object.tweet.full_text);
//!     }
//! }
//! ```
//!
//! ## Example file reader for `deleted-tweets.js`
//!
//! ```no_build
//! use std::io::Read;
//! use std::{fs, path};
//! use zip::read::ZipArchive;
//!
//! use twitter_archive::structs::tweets;
//!
//! fn main() {
//!     let input_file = "~/Downloads/twitter-archive.zip";
//!
//!     let file_descriptor = fs::File::open(input_file).expect("Unable to read --input-file");
//!     let mut zip_archive = ZipArchive::new(file_descriptor).unwrap();
//!     let mut zip_file = zip_archive.by_name("data/deleted-tweets.js").unwrap();
//!     let mut buff = String::new();
//!     zip_file.read_to_string(&mut buff).unwrap();
//!
//!     let json = buff.replacen("window.YTD.deleted_tweets.part0 = ", "", 1);
//!     let data: Vec<tweets::TweetObject> = serde_json::from_str(&json).expect("Unable to parse");
//!
//!     for (index, object) in data.iter().enumerate() {
//!         /* Do stuff with each Tweet */
//!         println!("Index: {index}");
//!         println!("Created at: {}", object.tweet.created_at);
//!         println!("vvv Content\n{}\n^^^ Content", object.tweet.full_text);
//!     }
//! }
//! ```
//!
//! ## Example content for `twitter-<DATE>-<UID>.zip:data/tweets.js`
//!
//! ```javascript
//! window.YTD.tweets.part0 = [
//!   {
//!     "tweet": {
//!       "edit_info": {
//!         "initial": {
//!           "editTweetIds": ["1690395372546301952"],
//!           "editableUntil": "2023-08-12T17:10:37.000Z",
//!           "editsRemaining": "5",
//!           "isEditEligible": true
//!         }
//!       },
//!       "retweeted": false,
//!       "source": "<a href=\"https://mobile.twitter.com\" rel=\"nofollow\">Twitter Web App</a>",
//!       "entities": {
//!         "hashtags": [],
//!         "symbols": [],
//!         "user_mentions": [
//!           {
//!             "name": "ThePrimeagen",
//!             "screen_name": "ThePrimeagen",
//!             "indices": ["0", "13"],
//!             "id_str": "291797158",
//!             "id": "291797158"
//!           }
//!         ],
//!         "urls": [
//!           {
//!             "url": "https://t.co/4LBPKIGBzf",
//!             "expanded_url": "https://www.youtube.com/watch?v=J7bX5dPUw0g",
//!             "display_url": "youtube.com/watch?v=J7bX5d…",
//!             "indices": ["132", "155"]
//!           }
//!         ]
//!       },
//!       "display_text_range": ["0", "276"],
//!       "favorite_count": "0",
//!       "id_str": "1690395372546301952",
//!       "in_reply_to_user_id": "291797158",
//!       "truncated": false,
//!       "retweet_count": "0",
//!       "id": "1690395372546301952",
//!       "possibly_sensitive": false,
//!       "created_at": "Sat Aug 12 16:10:37 +0000 2023",
//!       "favorited": false,
//!       "full_text": "@ThePrimeagen to answer your question about when writing interfaces, without the intention to change or test, is a good idea from;\n\nhttps://t.co/4LBPKIGBzf\n\n... Solidity interfaces are cheaper to store (S3), and pass over-the-wire, than shipping full contract(s) to consumers.",
//!       "lang": "en",
//!       "in_reply_to_screen_name": "ThePrimeagen",
//!       "in_reply_to_user_id_str": "291797158"
//!     }
//!   }
//! ]
//! ```
//!
//! Tip, to parse deleted tweets only requires one change in preparation;
//!
//! ```diff
//! -window.YTD.tweets.part0
//! +window.YTD.deleted_tweets.part0
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
/// use twitter_archive::structs::tweets::TweetObject;
///
/// let editable_until_string = "2023-08-12T17:10:37.000Z";
/// let editable_until_native_time = NaiveDateTime::parse_from_str(&editable_until_string, date_time_iso_8601::FORMAT).unwrap();
/// let editable_until_date_time = DateTime::<Utc>::from_naive_utc_and_offset(editable_until_native_time, Utc);
///
/// let created_at_string = "Sat Aug 12 16:10:37 +0000 2023";
/// let created_at_date_time: DateTime<Utc> = DateTime::parse_from_str(&created_at_string, created_at::FORMAT)
///     .unwrap()
///     .into();
///
/// let json = format!(r#"{{
///   "tweet": {{
///     "edit_info": {{
///       "initial": {{
///         "editTweetIds": [
///           "1690395372546301952"
///         ],
///         "editableUntil": "{editable_until_string}",
///         "editsRemaining": "5",
///         "isEditEligible": true
///       }}
///     }},
///     "retweeted": false,
///     "source": "<a href=\"https://mobile.twitter.com\" rel=\"nofollow\">Twitter Web App</a>",
///     "entities": {{
///       "hashtags": [],
///       "symbols": [],
///       "user_mentions": [
///         {{
///           "name": "ThePrimeagen",
///           "screen_name": "ThePrimeagen",
///           "indices": [
///             "0",
///             "13"
///           ],
///           "id_str": "291797158",
///           "id": "291797158"
///         }}
///       ],
///       "urls": [
///         {{
///           "url": "https://t.co/4LBPKIGBzf",
///           "expanded_url": "https://www.youtube.com/watch?v=J7bX5dPUw0g",
///           "display_url": "youtube.com/watch?v=J7bX5d…",
///           "indices": [
///             "132",
///             "155"
///           ]
///         }}
///       ]
///     }},
///     "display_text_range": [
///       "0",
///       "276"
///     ],
///     "favorite_count": "0",
///     "id_str": "1690395372546301952",
///     "in_reply_to_user_id": "291797158",
///     "truncated": false,
///     "retweet_count": "0",
///     "id": "1690395372546301952",
///     "possibly_sensitive": false,
///     "created_at": "{created_at_string}",
///     "favorited": false,
///     "full_text": "@ThePrimeagen to answer your question about when writing interfaces, without the intention to change or test, is a good idea from;\n\nhttps://t.co/4LBPKIGBzf\n\n... Solidity interfaces are cheaper to store (S3), and pass over-the-wire, than shipping full contract(s) to consumers.",
///     "lang": "en",
///     "in_reply_to_screen_name": "ThePrimeagen",
///     "in_reply_to_user_id_str": "291797158"
///   }}
/// }}"#);
///
/// let data: TweetObject = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.tweet.retweeted, false);
/// assert_eq!(data.tweet.source, "<a href=\"https://mobile.twitter.com\" rel=\"nofollow\">Twitter Web App</a>");
/// assert_eq!(data.tweet.display_text_range, [0, 276]);
/// assert_eq!(data.tweet.favorite_count, 0);
/// assert_eq!(data.tweet.id_str, "1690395372546301952");
/// assert_eq!(data.tweet.in_reply_to_user_id, Some("291797158".to_string()));
/// assert_eq!(data.tweet.truncated, false);
/// assert_eq!(data.tweet.retweet_count, 0);
/// assert_eq!(data.tweet.id, "1690395372546301952");
/// assert_eq!(data.tweet.possibly_sensitive, Some(false));
/// assert_eq!(data.tweet.created_at, created_at_date_time);
/// assert_eq!(data.tweet.favorited, false);
/// assert_eq!(data.tweet.full_text, "@ThePrimeagen to answer your question about when writing interfaces, without the intention to change or test, is a good idea from;\n\nhttps://t.co/4LBPKIGBzf\n\n... Solidity interfaces are cheaper to store (S3), and pass over-the-wire, than shipping full contract(s) to consumers.");
/// assert_eq!(data.tweet.lang, "en");
/// assert_eq!(data.tweet.in_reply_to_screen_name, Some("ThePrimeagen".to_string()));
/// assert_eq!(data.tweet.in_reply_to_user_id_str, Some("291797158".to_string()));
///
/// // Re-serialize is equivalent to original data
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
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
	///   "tweet": {
	///     "edit_info": {
	///       "initial": {
	///         "editTweetIds": ["1690395372546301952"],
	///         "editableUntil": "2023-08-12T17:10:37.000Z",
	///         "editsRemaining": "5",
	///         "isEditEligible": true
	///       }
	///     },
	///     "retweeted": false,
	///     "source": "<a href=\"https://mobile.twitter.com\" rel=\"nofollow\">Twitter Web App</a>",
	///     "entities": {
	///       "hashtags": [],
	///       "symbols": [],
	///       "user_mentions": [
	///         {
	///           "name": "ThePrimeagen",
	///           "screen_name": "ThePrimeagen",
	///           "indices": ["0", "13"],
	///           "id_str": "291797158",
	///           "id": "291797158"
	///         }
	///       ],
	///       "urls": [
	///         {
	///           "url": "https://t.co/4LBPKIGBzf",
	///           "expanded_url": "https://www.youtube.com/watch?v=J7bX5dPUw0g",
	///           "display_url": "youtube.com/watch?v=J7bX5d…",
	///           "indices": ["132", "155"]
	///         }
	///       ]
	///     },
	///     "display_text_range": ["0", "276"],
	///     "favorite_count": "0",
	///     "id_str": "1690395372546301952",
	///     "in_reply_to_user_id": "291797158",
	///     "truncated": false,
	///     "retweet_count": "0",
	///     "id": "1690395372546301952",
	///     "possibly_sensitive": false,
	///     "created_at": "Sat Aug 12 16:10:37 +0000 2023",
	///     "favorited": false,
	///     "full_text": "@ThePrimeagen to answer your question about when writing interfaces, without the intention to change or test, is a good idea from;\n\nhttps://t.co/4LBPKIGBzf\n\n... Solidity interfaces are cheaper to store (S3), and pass over-the-wire, than shipping full contract(s) to consumers.",
	///     "lang": "en",
	///     "in_reply_to_screen_name": "ThePrimeagen",
	///     "in_reply_to_user_id_str": "291797158"
	///   }
	/// }
	/// ```
	pub tweet: Tweet,
}

/// ```
/// use chrono::{DateTime, NaiveDateTime, Utc};
///
/// use twitter_archive::convert::{created_at, date_time_iso_8601};
///
/// use twitter_archive::structs::tweets::Tweet;
///
/// let editable_until_string = "2023-08-12T17:10:37.000Z";
/// let editable_until_native_time = NaiveDateTime::parse_from_str(&editable_until_string, date_time_iso_8601::FORMAT).unwrap();
/// let editable_until_date_time = DateTime::<Utc>::from_naive_utc_and_offset(editable_until_native_time, Utc);
///
/// let created_at_string = "Sat Aug 12 16:10:37 +0000 2023";
/// let created_at_date_time: DateTime<Utc> = DateTime::parse_from_str(&created_at_string, created_at::FORMAT)
///     .unwrap()
///     .into();
///
/// let json = format!(r#"{{
///   "edit_info": {{
///     "initial": {{
///       "editTweetIds": [
///         "1690395372546301952"
///       ],
///       "editableUntil": "{editable_until_string}",
///       "editsRemaining": "5",
///       "isEditEligible": true
///     }}
///   }},
///   "retweeted": false,
///   "source": "<a href=\"https://mobile.twitter.com\" rel=\"nofollow\">Twitter Web App</a>",
///   "entities": {{
///     "hashtags": [],
///     "symbols": [],
///     "user_mentions": [
///       {{
///         "name": "ThePrimeagen",
///         "screen_name": "ThePrimeagen",
///         "indices": [
///           "0",
///           "13"
///         ],
///         "id_str": "291797158",
///         "id": "291797158"
///       }}
///     ],
///     "urls": [
///       {{
///         "url": "https://t.co/4LBPKIGBzf",
///         "expanded_url": "https://www.youtube.com/watch?v=J7bX5dPUw0g",
///         "display_url": "youtube.com/watch?v=J7bX5d…",
///         "indices": [
///           "132",
///           "155"
///         ]
///       }}
///     ]
///   }},
///   "display_text_range": [
///     "0",
///     "276"
///   ],
///   "favorite_count": "0",
///   "id_str": "1690395372546301952",
///   "in_reply_to_user_id": "291797158",
///   "truncated": false,
///   "retweet_count": "0",
///   "id": "1690395372546301952",
///   "possibly_sensitive": false,
///   "created_at": "{created_at_string}",
///   "favorited": false,
///   "full_text": "@ThePrimeagen to answer your question about when writing interfaces, without the intention to change or test, is a good idea from;\n\nhttps://t.co/4LBPKIGBzf\n\n... Solidity interfaces are cheaper to store (S3), and pass over-the-wire, than shipping full contract(s) to consumers.",
///   "lang": "en",
///   "in_reply_to_screen_name": "ThePrimeagen",
///   "in_reply_to_user_id_str": "291797158"
/// }}"#);
///
/// let data: Tweet = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.retweeted, false);
/// assert_eq!(data.source, "<a href=\"https://mobile.twitter.com\" rel=\"nofollow\">Twitter Web App</a>");
/// assert_eq!(data.display_text_range, [0, 276]);
/// assert_eq!(data.favorite_count, 0);
/// assert_eq!(data.id_str, "1690395372546301952");
/// assert_eq!(data.in_reply_to_user_id, Some("291797158".to_string()));
/// assert_eq!(data.truncated, false);
/// assert_eq!(data.retweet_count, 0);
/// assert_eq!(data.id, "1690395372546301952");
/// assert_eq!(data.possibly_sensitive, Some(false));
/// assert_eq!(data.created_at, created_at_date_time);
/// assert_eq!(data.favorited, false);
/// assert_eq!(data.full_text, "@ThePrimeagen to answer your question about when writing interfaces, without the intention to change or test, is a good idea from;\n\nhttps://t.co/4LBPKIGBzf\n\n... Solidity interfaces are cheaper to store (S3), and pass over-the-wire, than shipping full contract(s) to consumers.");
/// assert_eq!(data.lang, "en");
/// assert_eq!(data.in_reply_to_screen_name, Some("ThePrimeagen".to_string()));
/// assert_eq!(data.in_reply_to_user_id_str, Some("291797158".to_string()));
///
/// // Re-serialize is equivalent to original data
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
pub struct Tweet {
	/// Data about edit history and availability for further edits
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "edit_info": {
	///     "initial": {
	///       "editTweetIds": ["1690395372546301952"],
	///       "editableUntil": "2023-08-12T17:10:37.000Z",
	///       "editsRemaining": "5",
	///       "isEditEligible": true
	///     }
	///   }
	/// }
	/// ```
	pub edit_info: TweetEditInfo,

	/// Is or is not retweeted
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "retweeted": false }
	/// ```
	pub retweeted: bool,

	/// URL that almost, if not, always points to `"<a href=\"https://mobile.twitter.com\" rel=\"nofollow\">Twitter Web App</a>"`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "source": "<a href=\"https://mobile.twitter.com\" rel=\"nofollow\">Twitter Web App</a>" }
	/// ```
	pub source: String,

	/// Additional data within Tweet such as hashtags and URLs
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "entries": {
	///     "hashtags": [],
	///     "symbols": [],
	///     "user_mentions": [
	///       {
	///         "name": "ThePrimeagen",
	///         "screen_name": "ThePrimeagen",
	///         "indices": ["0", "13"],
	///         "id_str": "291797158",
	///         "id": "291797158"
	///       }
	///     ],
	///     "urls": [
	///       {
	///         "url": "https://t.co/4LBPKIGBzf",
	///         "expanded_url": "https://www.youtube.com/watch?v=J7bX5dPUw0g",
	///         "display_url": "youtube.com/watch?v=J7bX5d…",
	///         "indices": ["132", "155"]
	///       }
	///     ]
	///   }
	/// }
	/// ```
	pub entities: TweetEntities,

	/// Indexes of beginning and end of Tweeted text
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "display_text_range": ["0", "276"]
	/// }
	/// ```
	#[serde(with = "convert::indices")]
	pub display_text_range: [usize; 2],

	/// How many hearts have been clicked for Tweet
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "favorite_count": "0" }
	/// ```
	#[serde(with = "convert::number_like_string")]
	pub favorite_count: usize,

	/// URL formats;
	///
	/// - Desktop: `https://twitter.com/i/web/status/{in_reply_to_status_id_str}`
	/// - Mobile: `https://mobile.twitter.com/i/web/status/{in_reply_to_status_id_str}`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "in_reply_to_status_id_str": "1111111111111111111" }
	/// ```
	#[serde(skip_serializing_if = "Option::is_none")]
	pub in_reply_to_status_id_str: Option<String>,

	/// URL formats;
	///
	/// - Desktop: `https://twitter.com/i/web/status/{id_str}`
	/// - Mobile: `https://mobile.twitter.com/i/web/status/{id_str}`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "id_str": "1690395372546301952" }
	/// ```
	pub id_str: String,

	/// URL formats;
	///
	/// - Desktop: `https://twitter.com/i/web/status/{in_reply_to_user_id}`
	/// - Mobile: `https://mobile.twitter.com/i/web/status/{in_reply_to_user_id}`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "in_reply_to_user_id": "291797158" }
	/// ```
	#[serde(skip_serializing_if = "Option::is_none")]
	pub in_reply_to_user_id: Option<String>,

	/// Is Tweet too long for most Twitter readers to wanna read?
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "truncated": false, }
	/// ```
	pub truncated: bool,

	/// How many felt Tweet worthy to re-Tweet?
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "retweet_count": "0" }
	/// ```
	#[serde(with = "convert::number_like_string")]
	pub retweet_count: usize,

	/// URL formats;
	///
	/// - Desktop: `https://twitter.com/i/web/status/{id}`
	/// - Mobile: `https://mobile.twitter.com/i/web/status/{id}`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "id": "1690395372546301952" }
	/// ```
	pub id: String,

	/// URL formats;
	///
	/// - Desktop: `https://twitter.com/i/web/status/{in_reply_to_status_id}`
	/// - Mobile: `https://mobile.twitter.com/i/web/status/{in_reply_to_status_id}`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "in_reply_to_status_id": "1111111111111111111" }
	/// ```
	#[serde(skip_serializing_if = "Option::is_none")]
	pub in_reply_to_status_id: Option<String>,

	/// Is the Tweet maybe ticklish?
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "possibly_sensitive": false }
	/// ```
	#[serde(skip_serializing_if = "Option::is_none")]
	pub possibly_sensitive: Option<bool>,

	/// Date time-stamp of when Tweet was originally tweeted
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "created_at": "Sat Aug 12 16:10:37 +0000 2023" }
	/// ```
	#[serde(with = "convert::created_at")]
	pub created_at: DateTime<Utc>,

	/// Is the Tweet a for sure favored Tweet?
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "favorited": false }
	/// ```
	pub favorited: bool,

	/// Content of Tweet with embedded newlines `\n` where applicable
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "full_text": "@ThePrimeagen to answer your question about when writing interfaces, without the intention to change or test, is a good idea from;\n\nhttps://t.co/4LBPKIGBzf\n\n... Solidity interfaces are cheaper to store (S3), and pass over-the-wire, than shipping full contract(s) to consumers."
	/// }
	/// ```
	pub full_text: String,

	/// Two letter string representing language Tweet was authored in (e.g. "en")
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "lang": "en" }
	/// ```
	pub lang: String,

	/// Same value as is found in `.tweets[].tweet.entries.user_mentions[].screen_name`
	///
	/// URL formats;
	///
	/// - Desktop: `https://twitter.com/{in_reply_to_screen_name}`
	///
	/// > Note; redirects to log-in if not logged in, and redirections may be broken.  Thanks be to
	/// > Mr. Musk !-D
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "in_reply_to_screen_name": "ThePrimeagen" }
	/// ```
	#[serde(skip_serializing_if = "Option::is_none")]
	pub in_reply_to_screen_name: Option<String>,

	/// URL formats;
	///
	/// - Desktop: `https://twitter.com/i/user/{in_reply_to_user_id_str}`
	///
	/// > Note; does **not** work if not logged-in.  Thanks be to Mr. Musk !-D
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "in_reply_to_user_id_str": "291797158" }
	/// ```
	#[serde(skip_serializing_if = "Option::is_none")]
	pub in_reply_to_user_id_str: Option<String>,
}

/// ## Example
///
/// ```
/// use chrono::{DateTime, NaiveDateTime, Utc};
///
/// use twitter_archive::structs::tweets::TweetEditInfo;
///
/// use twitter_archive::convert::date_time_iso_8601::FORMAT;
///
/// let editable_until_string = "2023-08-12T17:10:37.000Z";
/// let editable_until_native_time = NaiveDateTime::parse_from_str(&editable_until_string, FORMAT).unwrap();
/// let editable_until_date_time = DateTime::<Utc>::from_naive_utc_and_offset(editable_until_native_time, Utc);
///
/// let json = format!(r#"{{
///   "initial": {{
///     "editTweetIds": [
///       "1690395372546301952"
///     ],
///     "editableUntil": "{editable_until_string}",
///     "editsRemaining": "5",
///     "isEditEligible": true
///   }}
/// }}"#);
///
/// let data: TweetEditInfo = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.initial.edit_tweet_ids, ["1690395372546301952"]);
/// assert_eq!(data.initial.editable_until, editable_until_date_time);
/// assert_eq!(data.initial.edits_remaining, 5);
/// assert_eq!(data.initial.is_edit_eligible, true);
///
/// // Re-serialize is equivalent to original data
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
pub struct TweetEditInfo {
	/// Object/data-structure containing information about edited tweets
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///  "initial": {
	///    "editTweetIds": ["1690395372546301952"],
	///    "editableUntil": "2023-08-12T17:10:37.000Z",
	///    "editsRemaining": "5",
	///    "isEditEligible": true
	///  }
	/// }
	/// ```
	pub initial: TweetEditInfoInitial,
}

/// Whom-ever originally added the edit feature seems to have said, "F existing conventions, we're
/// doing this camel style" X-D
///
/// ```
/// use chrono::{DateTime, NaiveDateTime, Utc};
///
/// use twitter_archive::convert::date_time_iso_8601::FORMAT;
/// use twitter_archive::structs::tweets::TweetEditInfoInitial;
///
/// let time = "2023-08-12T17:10:37.000Z";
/// let date_time = NaiveDateTime::parse_from_str(&time, FORMAT).unwrap();
/// let editable_until = DateTime::<Utc>::from_naive_utc_and_offset(date_time, Utc);
///
/// let json = format!(r#"{{
///   "editTweetIds": [
///     "1690395372546301952"
///   ],
///   "editableUntil": "{time}",
///   "editsRemaining": "5",
///   "isEditEligible": true
/// }}"#);
///
/// let data: TweetEditInfoInitial = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.edit_tweet_ids, ["1690395372546301952"]);
/// assert_eq!(data.editable_until, editable_until);
/// assert_eq!(data.edits_remaining, 5);
/// assert_eq!(data.is_edit_eligible, true);
///
/// // Re-serialize is equivalent to original data
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct TweetEditInfoInitial {
	/// URL formats;
	///
	/// - Desktop: `https://twitter.com/i/web/status/{edit_tweet_ids}`
	/// - Mobile: `https://mobile.twitter.com/i/web/status/{edit_tweet_ids}`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///    "editTweetIds": ["1690395372546301952"]
	/// }
	/// ```
	pub edit_tweet_ids: Vec<String>,

	/// Date time stamp until editing is no longer allowed, even if paying for Mr. Musk perks
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "editableUntil": "2023-08-12T17:10:37.000Z" }
	/// ```
	#[serde(with = "convert::date_time_iso_8601")]
	pub editable_until: DateTime<Utc>,

	/// Remaining edits available, if account is currently paying Mr. Musk for check-mark parks
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "editsRemaining": "5" }
	/// ```
	#[serde(with = "convert::number_like_string")]
	pub edits_remaining: usize,

	/// State is a lie unless user of this data structure is paying member.  Thanks be to Mr. Musk
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "isEditEligible": true }
	/// ```
	pub is_edit_eligible: bool,
}

/// ## Example
///
/// ```
/// use twitter_archive::structs::tweets::TweetEntities;
///
/// let json = r#"{
///   "hashtags": [],
///   "symbols": [],
///   "user_mentions": [
///     {
///       "name": "ThePrimeagen",
///       "screen_name": "ThePrimeagen",
///       "indices": [
///         "0",
///         "13"
///       ],
///       "id_str": "291797158",
///       "id": "291797158"
///     }
///   ],
///   "urls": [
///     {
///       "url": "https://t.co/4LBPKIGBzf",
///       "expanded_url": "https://www.youtube.com/watch?v=J7bX5dPUw0g",
///       "display_url": "youtube.com/watch?v=J7bX5d…",
///       "indices": [
///         "132",
///         "155"
///       ]
///     }
///   ]
/// }"#;
///
/// let data: TweetEntities = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.hashtags.len(), 0);
/// assert_eq!(data.symbols.len(), 0);
/// assert_eq!(data.user_mentions.len(), 1);
/// assert_eq!(data.urls.len(), 1);
///
/// // Re-serialize is equivalent to original data
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
pub struct TweetEntities {
	/// List of hashtags (string prefixed by `#`) data within Tweet
	///
	/// TODO: Add example JSON data
	pub hashtags: Vec<TweetEntitiesEntry>,

	/// List of symbols (string prefixed by `$`) data within Tweet
	///
	/// TODO: Add example JSON data
	pub symbols: Vec<TweetEntitiesEntry>,

	/// List of user data mentioned by Tweet
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "user_mentions": [
	///     {
	///       "name": "ThePrimeagen",
	///       "screen_name": "ThePrimeagen",
	///       "indices": ["0", "13"],
	///       "id_str": "291797158",
	///       "id": "291797158"
	///     }
	///   ]
	/// }
	/// ```
	pub user_mentions: Vec<TweetEntitiesUserMention>,

	/// List of URL data within Tweet
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "urls" [
	///     {
	///       "url": "https://t.co/4LBPKIGBzf",
	///       "expanded_url": "https://www.youtube.com/watch?v=J7bX5dPUw0g",
	///       "display_url": "youtube.com/watch?v=J7bX5d…",
	///       "indices": ["132", "155"]
	///     }
	///   ]
	/// }
	/// ```
	pub urls: Vec<TweetEntitiesUserUrl>,
}

/// Common structure for;
///
/// - `tweets[].tweet.entities.hashtags[]`
/// - `tweets[].tweet.entities.symbols[]`
///
/// TODO: Add doc-tests
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
pub struct TweetEntitiesEntry {
	/// String representation of hashtag or symbol entry
	///
	/// TODO: Add example JSON data
	pub text: String,

	/// Start and stop indexes within `.tweets[].tweet.full_text`
	///
	/// TODO: Add example JSON data
	#[serde(with = "convert::indices")]
	pub indices: [usize; 2],
}

/// ## Example
///
/// ```
/// use twitter_archive::structs::tweets::TweetEntitiesUserMention;
///
/// let json = r#"{
///   "name": "ThePrimeagen",
///   "screen_name": "ThePrimeagen",
///   "indices": [
///     "0",
///     "13"
///   ],
///   "id_str": "291797158",
///   "id": "291797158"
/// }"#;
///
/// let data: TweetEntitiesUserMention = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.name, "ThePrimeagen");
/// assert_eq!(data.screen_name, "ThePrimeagen");
/// assert_eq!(data.indices, [0, 13]);
/// assert_eq!(data.id_str, "291797158");
/// assert_eq!(data.id, "291797158");
///
/// // Re-serialize is equivalent to original data
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
pub struct TweetEntitiesUserMention {
	/// Who to _@_ when mentioning a user
	///
	/// URL formats;
	///
	/// - Desktop: `https://twitter.com/{name}`
	///
	/// > Note; redirects to log-in if not logged in, and redirections may be broken.  Thanks be to
	/// > Mr. Musk !-D
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "name": "ThePrimeagen" }
	/// ```
	pub name: String,

	/// Contains one value identical to `.tweets[].tweet.in_reply_to_screen_name`
	///
	/// URL formats;
	///
	/// - Desktop: `https://twitter.com/{screen_name}`
	///
	/// > Note; redirects to log-in if not logged in, and redirections may be broken.  Thanks be to
	/// > Mr. Musk !-D
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "screen_name": "ThePrimeagen" }
	/// ```
	pub screen_name: String,

	/// Start and stop indexes within `.tweets[].tweet.full_text`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "indices": ["0", "13"]
	/// }
	/// ```
	#[serde(with = "convert::indices")]
	pub indices: [usize; 2],

	/// URL formats;
	///
	/// - Desktop: `https://twitter.com/i/user/{id_str}`
	///
	/// > Note; does **not** work if not logged-in.  Thanks be to Mr. Musk !-D
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "id_str": "291797158" }
	/// ```
	pub id_str: String,

	/// URL formats;
	///
	/// - Desktop: `https://twitter.com/i/user/{id}`
	///
	/// > Note; does **not** work if not logged-in.  Thanks be to Mr. Musk !-D
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "id": "291797158" }
	/// ```
	pub id: String,
}

/// ## Example
///
/// ```
/// use twitter_archive::structs::tweets::TweetEntitiesUserUrl;
///
/// let json = r#"{
///   "url": "https://t.co/4LBPKIGBzf",
///   "expanded_url": "https://www.youtube.com/watch?v=J7bX5dPUw0g",
///   "display_url": "youtube.com/watch?v=J7bX5d…",
///   "indices": [
///     "132",
///     "155"
///   ]
/// }"#;
///
/// let data: TweetEntitiesUserUrl = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.url, "https://t.co/4LBPKIGBzf");
/// assert_eq!(data.expanded_url, "https://www.youtube.com/watch?v=J7bX5dPUw0g");
/// assert_eq!(data.display_url, "youtube.com/watch?v=J7bX5d…");
/// assert_eq!(data.indices, [132, 155]);
///
/// // Re-serialize is equivalent to original data
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
pub struct TweetEntitiesUserUrl {
	/// Twitter shortened, and tracking, URL
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "url": "https://t.co/4LBPKIGBzf" }
	/// ```
	pub url: String,

	/// The _real_ URL
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "expanded_url": "https://www.youtube.com/watch?v=J7bX5dPUw0g" }
	/// ```
	pub expanded_url: String,

	/// What clients are able to view of URL within text
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "display_url": "youtube.com/watch?v=J7bX5d…" }
	/// ```
	pub display_url: String,

	/// Start and stop indexes within `.tweets[].tweet.full_text`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "indices": ["132", "155"]
	/// }
	/// ```
	#[serde(with = "convert::indices")]
	pub indices: [usize; 2],
}
