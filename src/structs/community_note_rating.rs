#!/usr/bin/env rust

//! Tweeter archives as of 2023-08-31 have public community_note_rating found under;
//!
//!   twitter-<DATE>-<UID>.zip:data/community-note-rating.js
//!
//! ## Example file reader
//!
//! ```no_build
//! use std::io::Read;
//! use std::{fs, path};
//! use zip::read::ZipArchive;
//!
//! use twitter_archive::structs::community_note_rating;
//!
//! fn main() {
//!     let input_file = "~/Downloads/twitter-archive.zip";
//!
//!     let file_descriptor = fs::File::open(input_file).expect("Unable to read --input-file");
//!     let mut zip_archive = ZipArchive::new(file_descriptor).unwrap();
//!     let mut zip_file = zip_archive.by_name("data/community-note-rating.js").unwrap();
//!     let mut buff = String::new();
//!     zip_file.read_to_string(&mut buff).unwrap();
//!
//!     let json = buff.replacen("window.YTD.community_note_rating.part0 = ", "", 1);
//!     let data: Vec<community_note_rating::CommunityNoteRatingObject> = serde_json::from_str(&json).expect("Unable to parse");
//!
//!     for (index, object) in data.iter().enumerate() {
//!         /* Do stuff with each deleted Tweet */
//!         println!("Community note rating index: {index}");
//!         println!("Not helpful tags: {:?}", object.community_note_rating.not_helpful_tags);
//!         println!("Note ID: {}", object.community_note_rating.note_id);
//!         println!("Helpfulness level: {}", object.community_note_rating.helpfulness_level);
//!         println!("Created at: {}", object.community_note_rating.created_at);
//!         println!("User ID: {}", object.community_note_rating.user_id);
//!     }
//! }
//! ```
//!
//! ## Example `twitter-<DATE>-<UID>.zip:data/community-note-rating.js` content
//!
//! ```javascript
//! window.YTD.community_note_rating.part0 = [
//!   {
//!     "communityNoteRating" : {
//!       "notHelpfulTags" : [
//!         "OpinionSpeculation",
//!         "NoteNotNeeded"
//!       ],
//!       "noteId" : "9999999999999999999",
//!       "helpfulnessLevel" : "NotHelpful",
//!       "createdAt" : "2020-01-20T21:42:09.068Z",
//!       "userId" : "111111111"
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
/// use twitter_archive::structs::community_note_rating::CommunityNoteRatingObject;
///
/// let created_at_string = "2020-01-20T21:42:09.068Z";
/// let created_at_native_time = NaiveDateTime::parse_from_str(&created_at_string, FORMAT).unwrap();
/// let created_at_date_time = DateTime::<Utc>::from_naive_utc_and_offset(created_at_native_time, Utc);
///
/// let json = format!(r#"{{
///   "communityNoteRating" : {{
///     "notHelpfulTags" : [
///       "OpinionSpeculation",
///       "NoteNotNeeded"
///     ],
///     "noteId" : "9999999999999999999",
///     "helpfulnessLevel" : "NotHelpful",
///     "createdAt" : "{created_at_string}",
///     "userId" : "111111111"
///   }}
/// }}"#);
///
/// let data: CommunityNoteRatingObject = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.community_note_rating.not_helpful_tags.len(), 2);
/// assert_eq!(data.community_note_rating.not_helpful_tags[0], "OpinionSpeculation");
/// assert_eq!(data.community_note_rating.not_helpful_tags[1], "NoteNotNeeded");
///
/// assert_eq!(data.community_note_rating.note_id, "9999999999999999999");
/// assert_eq!(data.community_note_rating.helpfulness_level, "NotHelpful");
/// assert_eq!(data.community_note_rating.created_at, created_at_date_time);
/// assert_eq!(data.community_note_rating.user_id, "111111111");
///
/// assert_eq!(data.community_note_rating.created_at, created_at_date_time);
///
/// // Re-serialize is equivalent to original data without pretty printing
/// // assert_eq!(serde_json::to_string(&data).unwrap(), json.replace("\n", "").replace(" ", ""));
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct CommunityNoteRatingObject {
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "communityNoteRating" : {
	///     "notHelpfulTags" : [
	///       "OpinionSpeculation",
	///       "NoteNotNeeded"
	///     ],
	///     "noteId" : "9999999999999999999",
	///     "helpfulnessLevel" : "NotHelpful",
	///     "createdAt" : "2020-01-20T21:42:09.068Z",
	///     "userId" : "111111111"
	///   }
	/// }
	/// ```
	pub community_note_rating: CommunityNoteRating,
}

/// ## Example
///
/// ```
/// use chrono::{DateTime, NaiveDateTime, Utc};
///
/// use twitter_archive::convert::date_time_iso_8601::FORMAT;
///
/// use twitter_archive::structs::community_note_rating::CommunityNoteRating;
///
/// let created_at_string = "2020-01-20T21:42:09.068Z";
/// let created_at_native_time = NaiveDateTime::parse_from_str(&created_at_string, FORMAT).unwrap();
/// let created_at_date_time = DateTime::<Utc>::from_naive_utc_and_offset(created_at_native_time, Utc);
///
/// let json = format!(r#"{{
///   "notHelpfulTags" : [
///     "OpinionSpeculation",
///     "NoteNotNeeded"
///   ],
///   "noteId" : "9999999999999999999",
///   "helpfulnessLevel" : "NotHelpful",
///   "createdAt" : "{created_at_string}",
///   "userId" : "111111111"
/// }}"#);
///
/// let data: CommunityNoteRating = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.not_helpful_tags.len(), 2);
/// assert_eq!(data.not_helpful_tags[0], "OpinionSpeculation");
/// assert_eq!(data.not_helpful_tags[1], "NoteNotNeeded");
///
/// assert_eq!(data.note_id, "9999999999999999999");
/// assert_eq!(data.helpfulness_level, "NotHelpful");
/// assert_eq!(data.created_at, created_at_date_time);
/// assert_eq!(data.user_id, "111111111");
///
/// // Re-serialize is equivalent to original data without pretty printing
/// assert_eq!(serde_json::to_string(&data).unwrap(), json.replace("\n", "").replace(" ", ""));
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct CommunityNoteRating {
	/// List of tags about why community note was not helpful
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "notHelpfulTags" : [
	///     "OpinionSpeculation",
	///     "NoteNotNeeded"
	///   ]
	/// }
	/// ```
	pub not_helpful_tags: Vec<String>,

	/// ## Example JSON data
	///
	/// ```json
	/// { "noteId" : "9999999999999999999" }
	/// ```
	pub note_id: String,

	/// ## Example JSON data
	///
	/// ```json
	/// { "helpfulnessLevel" : "NotHelpful" }
	/// ```
	pub helpfulness_level: String,

	/// ## Example JSON data
	///
	/// ```json
	/// { "createdAt" : "2020-01-20T21:42:09.068Z" }
	/// ```
	#[serde(with = "convert::date_time_iso_8601")]
	pub created_at: DateTime<Utc>,

	/// ## Example JSON data
	///
	/// ```json
	/// { "userId" : "111111111" }
	/// ```
	pub user_id: String,
}
