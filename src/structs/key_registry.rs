#!/usr/bin/env rust

//! Tweeter archives as of 2023-08-31 have private data found under;
//!
//!   twitter-<DATE>-<UID>.zip:data/key-registry.js
//!
//! ## Example file reader
//!
//! ```no_build
//! use std::io::Read;
//! use std::{fs, path};
//! use zip::read::ZipArchive;
//!
//! use twitter_archive::structs::key_registry;
//!
//! fn main() {
//!     let input_file = "~/Downloads/twitter-archive.zip";
//!
//!     let file_descriptor = fs::File::open(input_file).expect("Unable to read --input-file");
//!     let mut zip_archive = ZipArchive::new(file_descriptor).unwrap();
//!     let mut zip_file = zip_archive.by_name("data/key-registry.js").unwrap();
//!     let mut buff = String::new();
//!     zip_file.read_to_string(&mut buff).unwrap();
//!
//!     let json = buff.replacen("window.YTD.key_registry.part0 = ", "", 1);
//!     let data: Vec<key_registry::RegisteredDevicesObject> = serde_json::from_str(&json).expect("Unable to parse");
//!
//!     for (index_registered_devices, object_registered_devices) in data.iter().enumerate() {
//!         /* Do stuff with each `RegisteredDevices` entry */
//!         println!("Registered devices index: {index_registered_devices}");
//!         let device_metadata_list = &object_registered_devices.registered_devices.device_metadata_list;
//!         for (index_device_metadata, object_device_metadata) in device_metadata_list.iter().enumerate() {
//!             println!("Device Metadata index: {index_device_metadata}");
//!             println!("User agent: {}", object_device_metadata.user_agent);
//!             println!("Registration token: {}", object_device_metadata.registration_token);
//!             println!("Identity key: {}", object_device_metadata.identity_key);
//!             println!("Created at: {}", object_device_metadata.created_at);
//!             println!("Device ID: {}", object_device_metadata.device_id);
//!         }
//!     }
//! }
//! ```
//!
//! ## Example `twitter-<DATE>-<UID>.zip:data/key-registry.js` content
//!
//! ```javascript
//! window.YTD.key_registry.part0 = [
//!   {
//!     "registeredDevices" : {
//!       "deviceMetadataList" : [
//!         {
//!           "userAgent" : "Mozilla/5.0 (Windows NT 10.0; rv:102.0) Gecko/20100101 Firefox/102.0",
//!           "registrationToken" : "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
//!           "identityKey" : "DEADBEEF",
//!           "createdAt" : "2023-05-30T13:31:42.908Z",
//!           "deviceId" : "xxxxxxxx-111a-0000-abcd-333333333333"
//!         }
//!       ]
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
/// use twitter_archive::convert::date_time_iso_8601;
///
/// use twitter_archive::structs::key_registry::RegisteredDevicesObject;
///
/// let created_at_string = "2023-05-30T13:31:42.908Z";
/// let created_at_native_time = NaiveDateTime::parse_from_str(&created_at_string, date_time_iso_8601::FORMAT).unwrap();
/// let created_at_date_time = DateTime::<Utc>::from_naive_utc_and_offset(created_at_native_time, Utc);
///
/// let json = format!(r#"{{
///   "registeredDevices": {{
///     "deviceMetadataList": [
///       {{
///         "userAgent": "Mozilla/5.0 (Windows NT 10.0; rv:102.0) Gecko/20100101 Firefox/102.0",
///         "registrationToken": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
///         "identityKey": "DEADBEEF",
///         "createdAt": "{created_at_string}",
///         "deviceId": "xxxxxxxx-111a-0000-abcd-333333333333"
///       }}
///     ]
///   }}
/// }}"#);
///
/// let data: RegisteredDevicesObject = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.registered_devices.device_metadata_list[0].user_agent, "Mozilla/5.0 (Windows NT 10.0; rv:102.0) Gecko/20100101 Firefox/102.0");
/// assert_eq!(data.registered_devices.device_metadata_list[0].registration_token, "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
/// assert_eq!(data.registered_devices.device_metadata_list[0].identity_key, "DEADBEEF");
/// assert_eq!(data.registered_devices.device_metadata_list[0].created_at, created_at_date_time);
/// assert_eq!(data.registered_devices.device_metadata_list[0].device_id, "xxxxxxxx-111a-0000-abcd-333333333333");
///
/// // Re-serialize is equivalent to original data
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct RegisteredDevicesObject {
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "registeredDevices": {
	///     "deviceMetadataList": [
	///       {
	///         "userAgent": "Mozilla/5.0 (Windows NT 10.0; rv:102.0) Gecko/20100101 Firefox/102.0",
	///         "registrationToken": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
	///         "identityKey": "DEADBEEF",
	///         "createdAt": "2023-05-30T13:31:42.908Z",
	///         "deviceId": "xxxxxxxx-111a-0000-abcd-333333333333"
	///       }
	///     ]
	///   }
	/// }
	/// ```
	pub registered_devices: RegisteredDevices,
}

/// ## Example
///
/// ```
/// use chrono::{DateTime, NaiveDateTime, Utc};
///
/// use twitter_archive::convert::date_time_iso_8601;
///
/// use twitter_archive::structs::key_registry::RegisteredDevices;
///
/// let created_at_string = "2023-05-30T13:31:42.908Z";
/// let created_at_native_time = NaiveDateTime::parse_from_str(&created_at_string, date_time_iso_8601::FORMAT).unwrap();
/// let created_at_date_time = DateTime::<Utc>::from_naive_utc_and_offset(created_at_native_time, Utc);
///
/// let json = format!(r#"{{
///   "deviceMetadataList": [
///     {{
///       "userAgent": "Mozilla/5.0 (Windows NT 10.0; rv:102.0) Gecko/20100101 Firefox/102.0",
///       "registrationToken": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
///       "identityKey": "DEADBEEF",
///       "createdAt": "{created_at_string}",
///       "deviceId": "xxxxxxxx-111a-0000-abcd-333333333333"
///     }}
///   ]
/// }}"#);
///
/// let data: RegisteredDevices = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.device_metadata_list[0].user_agent, "Mozilla/5.0 (Windows NT 10.0; rv:102.0) Gecko/20100101 Firefox/102.0");
/// assert_eq!(data.device_metadata_list[0].registration_token, "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
/// assert_eq!(data.device_metadata_list[0].identity_key, "DEADBEEF");
/// assert_eq!(data.device_metadata_list[0].created_at, created_at_date_time);
/// assert_eq!(data.device_metadata_list[0].device_id, "xxxxxxxx-111a-0000-abcd-333333333333");
///
/// // Re-serialize is equivalent to original data
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct RegisteredDevices {
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "deviceMetadataList": [
	///     {
	///       "userAgent": "Mozilla/5.0 (Windows NT 10.0; rv:102.0) Gecko/20100101 Firefox/102.0",
	///       "registrationToken": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
	///       "identityKey": "DEADBEEF",
	///       "createdAt": "2023-05-30T13:31:42.908Z",
	///       "deviceId": "xxxxxxxx-111a-0000-abcd-333333333333"
	///     }
	///   ]
	/// }
	/// ```
	pub device_metadata_list: Vec<DeviceMetadata>,
}

/// ## Example
///
/// ```
/// use chrono::{DateTime, NaiveDateTime, Utc};
///
/// use twitter_archive::convert::date_time_iso_8601;
///
/// use twitter_archive::structs::key_registry::DeviceMetadata;
///
/// let created_at_string = "2023-05-30T13:31:42.908Z";
/// let created_at_native_time = NaiveDateTime::parse_from_str(&created_at_string, date_time_iso_8601::FORMAT).unwrap();
/// let created_at_date_time = DateTime::<Utc>::from_naive_utc_and_offset(created_at_native_time, Utc);
///
/// let json = format!(r#"{{
///   "userAgent": "Mozilla/5.0 (Windows NT 10.0; rv:102.0) Gecko/20100101 Firefox/102.0",
///   "registrationToken": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
///   "identityKey": "DEADBEEF",
///   "createdAt": "{created_at_string}",
///   "deviceId": "xxxxxxxx-111a-0000-abcd-333333333333"
/// }}"#);
///
/// let data: DeviceMetadata = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.user_agent, "Mozilla/5.0 (Windows NT 10.0; rv:102.0) Gecko/20100101 Firefox/102.0");
/// assert_eq!(data.registration_token, "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
/// assert_eq!(data.identity_key, "DEADBEEF");
/// assert_eq!(data.created_at, created_at_date_time);
/// assert_eq!(data.device_id, "xxxxxxxx-111a-0000-abcd-333333333333");
///
/// // Re-serialize is equivalent to original data
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct DeviceMetadata {
	/// ## Example JSON data
	///
	/// ```json
	/// { "userAgent": "Mozilla/5.0 (Windows NT 10.0; rv:102.0) Gecko/20100101 Firefox/102.0" }
	/// ```
	pub user_agent: String,

	/// ## Example JSON data
	///
	/// ```json
	/// { "registrationToken": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa" }
	/// ```
	pub registration_token: String,

	/// ## Example JSON data
	///
	/// ```json
	/// { "identityKey": "DEADBEEF" }
	/// ```
	pub identity_key: String,

	/// ## Example JSON data
	///
	/// ```json
	/// { "createdAt": "2023-05-30T13:31:42.908Z" }
	/// ```
	#[serde(with = "convert::date_time_iso_8601")]
	pub created_at: DateTime<Utc>,

	/// ## Example JSON data
	///
	/// ```json
	/// { "deviceId": "xxxxxxxx-111a-0000-abcd-333333333333" }
	/// ```
	pub device_id: String,
}
