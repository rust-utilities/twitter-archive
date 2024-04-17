#!/usr/bin/env rust

//! Tweeter archives as of 2023-08-31 have private data found under;
//!
//!   twitter-<DATE>-<UID>.zip:data/phone-number.js
//!
//! ## Example file reader
//!
//! ```no_build
//! use std::io::Read;
//! use std::{fs, path};
//! use zip::read::ZipArchive;
//!
//! use twitter_archive::structs::phone_number;
//!
//! fn main() {
//!     let input_file = "~/Downloads/twitter-archive.zip";
//!
//!     let file_descriptor = fs::File::open(input_file).expect("Unable to read --input-file");
//!     let mut zip_archive = ZipArchive::new(file_descriptor).unwrap();
//!     let mut zip_file = zip_archive.by_name("data/phone-number.js").unwrap();
//!     let mut buff = String::new();
//!     zip_file.read_to_string(&mut buff).unwrap();
//!
//!     let json = buff.replacen("window.YTD.phone_number.part0 = ", "", 1);
//!     let data: Vec<phone_number::DeviceObject> = serde_json::from_str(&json).expect("Unable to parse");
//!
//!     for (index, object) in data.iter().enumerate() {
//!         /* Do stuff with each phone number */
//!         println!("Phone index: {index}");
//!         println!("Phone number: {}", object.device.phone_number);
//!     }
//!     assert_eq!(true, false);
//! }
//! ```
//!
//! ## Example `twitter-<DATE>-<UID>.zip:data/phone-number.js` content
//!
//! ```javascript
//! window.YTD.phone_number.part0 = [
//!   {
//!     "device" : {
//!       "phoneNumber" : "+15551234567"
//!     }
//!   }
//! ]
//! ```

use derive_more::Display;
use serde::{Deserialize, Serialize};

/// ## Example
///
/// ```
/// use twitter_archive::structs::phone_number::DeviceObject;
///
/// let json = r#"{
///   "device": {
///     "phoneNumber": "+15551234567"
///   }
/// }"#;
///
/// let data: DeviceObject = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.device.phone_number, "+15551234567");
///
/// // Re-serialize is equivalent to original data
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct DeviceObject {
	/// Possibly unnecessary level of indirection created by upstream
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "device": {
	///     "phoneNumber": "+15551234567"
	///   }
	/// }
	/// ```
	pub device: Device,
}

/// ## Example
///
/// ```
/// use twitter_archive::structs::phone_number::Device;
///
/// let json = r#"{
///   "phoneNumber": "+15551234567"
/// }"#;
///
/// let data: Device = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.phone_number, "+15551234567");
///
/// // Re-serialize is equivalent to original data
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct Device {
	/// ## Example JSON data
	///
	/// ```json
	/// { "phoneNumber": "+15551234567" }
	/// ```
	pub phone_number: String,
}
