#!/usr/bin/env rust

//! Tweeter archives as of 2023-08-31 have private data found under;
//!
//!   twitter-<DATE>-<UID>.zip:data/profile.js
//!
//! ## Example file reader
//!
//! ```no_build
//! use std::io::Read;
//! use std::{fs, path};
//! use zip::read::ZipArchive;
//!
//! use twitter_archive::structs::profile;
//!
//! fn main() {
//!     let input_file = "~/Downloads/twitter-archive.zip";
//!
//!     let file_descriptor = fs::File::open(input_file).expect("Unable to read --input-file");
//!     let mut zip_archive = ZipArchive::new(file_descriptor).unwrap();
//!     let mut zip_file = zip_archive.by_name("data/profile.js").unwrap();
//!     let mut buff = String::new();
//!     zip_file.read_to_string(&mut buff).unwrap();
//!
//!     let json = buff.replacen("window.YTD.profile.part0 = ", "", 1);
//!     let data: Vec<profile::ProfileObject> = serde_json::from_str(&json).expect("Unable to parse");
//!
//!     for (index, object) in data.iter().enumerate() {
//!         /* Do stuff with each Profile */
//!         println!("Profile index: {index}");
//!         println!("Avatar Media URL: {}", object.profile.avatar_media_url);
//!         println!("Description Bio: {}", object.profile.description.bio);
//!         println!("Description Website: {}", object.profile.description.website);
//!         println!("Description Location: {}", object.profile.description.location);
//!     }
//! }
//! ```
//!
//! ## Example `twitter-<DATE>-<UID>.zip:data/profile.js` content
//!
//! ```javascript
//! window.YTD.profile.part0 = [
//!   {
//!     "profile" : {
//!       "description" : {
//!         "bio" : "Tweet sized technical tips, and links to Open Source projects are what I generally share here.\n\nThanks for stopping by lifeforms, and good luck to y'all!",
//!         "website" : "https://t.co/6VtgySlriu",
//!         "location" : ""
//!       },
//!       "avatarMediaUrl" : "https://pbs.twimg.com/profile_images/575070434267279361/HSLiX96Z.jpeg"
//!     }
//!   }
//! ]
//! ```

use derive_more::Display;
use serde::{Deserialize, Serialize};

/// ## Example
///
/// ```
/// use twitter_archive::structs::profile::ProfileObject;
///
/// let json = r#"{
///   "profile": {
///     "description": {
///       "bio": "Howdy!",
///       "website": "https://t.co/6VtgySlriu",
///       "location": ""
///     },
///     "avatarMediaUrl": "https://pbs.twimg.com/profile_images/575070434267279361/HSLiX96Z.jpeg"
///   }
/// }"#;
///
/// let data: ProfileObject = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.profile.description.bio, "Howdy!");
/// assert_eq!(data.profile.description.website, "https://t.co/6VtgySlriu");
/// assert_eq!(data.profile.description.location, "");
/// assert_eq!(data.profile.avatar_media_url, "https://pbs.twimg.com/profile_images/575070434267279361/HSLiX96Z.jpeg");
///
/// // Re-serialize is equivalent to original data
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct ProfileObject {
	/// Why they wrapped a list of name changes within unnecessary object label is anyone's guess
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "profile": {
	///     "description": {
	///       "bio": "Howdy!",
	///       "website": "https://t.co/6VtgySlriu",
	///       "location": ""
	///     },
	///     "avatarMediaUrl": "https://pbs.twimg.com/profile_images/575070434267279361/HSLiX96Z.jpeg"
	///   }
	/// }
	/// ```
	pub profile: Profile,
}

/// ## Example
///
/// ```
/// use twitter_archive::structs::profile::Profile;
///
/// let json = r#"{
///   "description": {
///     "bio": "Howdy!",
///     "website": "https://t.co/6VtgySlriu",
///     "location": ""
///   },
///   "avatarMediaUrl": "https://pbs.twimg.com/profile_images/575070434267279361/HSLiX96Z.jpeg"
/// }"#;
///
/// let data: Profile = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.description.bio, "Howdy!");
/// assert_eq!(data.description.website, "https://t.co/6VtgySlriu");
/// assert_eq!(data.description.location, "");
/// assert_eq!(data.avatar_media_url, "https://pbs.twimg.com/profile_images/575070434267279361/HSLiX96Z.jpeg");
///
/// // Re-serialize is equivalent to original data
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct Profile {
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "description": {
	///     "bio": "Howdy!",
	///     "website": "https://t.co/6VtgySlriu",
	///     "location": ""
	///   }
	/// }
	/// ```
	pub description: ProfileDescription,

	/// ## Example JSON data
	///
	/// ```json
	/// { "avatarMediaUrl": "https://pbs.twimg.com/profile_images/575070434267279361/HSLiX96Z.jpeg" }
	/// ```
	pub avatar_media_url: String,
}

/// ## Example
///
/// ```
/// use twitter_archive::structs::profile::ProfileDescription;
///
/// let json = r#"{
///   "bio": "Howdy!",
///   "website": "https://t.co/6VtgySlriu",
///   "location": ""
/// }"#;
///
/// let data: ProfileDescription = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.bio, "Howdy!");
/// assert_eq!(data.website, "https://t.co/6VtgySlriu");
/// assert_eq!(data.location, "");
///
/// // Re-serialize is equivalent to original data
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
pub struct ProfileDescription {
	/// ## Example JSON data
	///
	/// ```json
	/// { "bio": "Howdy!" }
	/// ```
	pub bio: String,

	/// ## Example JSON data
	///
	/// ```json
	/// { "website": "https://t.co/6VtgySlriu" }
	/// ```
	pub website: String,

	/// ## Example JSON data
	///
	/// ```json
	/// { "location": "" }
	/// ```
	pub location: String,
}
