#!/usr/bin/env rust

//! Tweeter archives as of 2023-08-31 have public verified found under;
//!
//!   twitter-<DATE>-<UID>.zip:data/verified.js
//!
//! ## Example file reader
//!
//! ```no_build
//! use std::io::Read;
//! use std::{fs, path};
//! use zip::read::ZipArchive;
//!
//! use twitter_archive::structs::verified;
//!
//! fn main() {
//!     let input_file = "path/to/twitter.zip";
//!
//!     let file_descriptor = fs::File::open(input_file).expect("Unable to read --input-file");
//!     let mut zip_archive = ZipArchive::new(file_descriptor).unwrap();
//!     let mut zip_file = zip_archive.by_name("data/verified.js").unwrap();
//!     let mut buff = String::new();
//!     zip_file.read_to_string(&mut buff).unwrap();
//!
//!     let json = buff.replacen("window.YTD.verified.part0 = ", "", 1);
//!     let data: Vec<verified::VerifiedObject> = serde_json::from_str(&json).expect("Unable to parse");
//!
//!     for (index, object) in data.iter().enumerate() {
//!         /* Do stuff with each Tweet */
//!         println!("Index: {index}");
//!         println!("Account ID: {}", object.verified.account_id);
//!         println!("Verified: {}", object.verified.verified);
//!     }
//! }
//! ```
//!
//! ## Example `twitter-<DATE>-<UID>.zip:data/verified.js` content
//!
//! ```javascript
//! window.YTD.verified.part0 = [
//!   {
//!     "verified" : {
//!       "accountId" : "435455769",
//!       "verified" : false
//!     }
//!   }
//! ]
//! ```

use derive_more::Display;
use serde::{Deserialize, Serialize};

/// ## Example
///
/// ```
/// use twitter_archive::structs::verified::VerifiedObject;
///
/// let json = r#"{
///   "verified": {
///     "accountId": "435455769",
///     "verified": false
///   }
/// }"#;
///
/// let data: VerifiedObject = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.verified.account_id, "435455769");
/// assert_eq!(data.verified.verified, false);
///
/// // Re-serialize is equivalent to original data
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
pub struct VerifiedObject {
	/// Why they wrapped a list of Verified data within unnecessary object label is anyone's guess
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "verified": {
	///     "accountId": "435455769",
	///     "verified": false
	///   }
	/// }
	/// ```
	pub verified: Verified,
}

/// ## Example
///
/// ```
/// use twitter_archive::structs::verified::Verified;
///
/// let json = r#"{
///   "accountId": "435455769",
///   "verified": false
/// }"#;
///
/// let data: Verified = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.account_id, "435455769");
/// assert_eq!(data.verified, false);
///
/// // Re-serialize is equivalent to original data
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct Verified {
	/// ## Example JSON data
	///
	/// ```json
	/// { "accountId": "435455769" }
	/// ```
	pub account_id: String,

	/// ## Example JSON data
	///
	/// ```json
	/// { "verified": false }
	/// ```
	pub verified: bool,
}
