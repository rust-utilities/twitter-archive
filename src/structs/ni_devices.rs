#!/usr/bin/env rust

//! Tweeter archives as of 2023-08-31 have private data found under;
//!
//!   twitter-<DATE>-<UID>.zip:data/ni_devices.js
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
//! use twitter_archive::structs::ni_devices;
//!
//! fn main() {
//!     let input_file = "~/Downloads/twitter-archive.zip";
//!
//!     let file_descriptor = fs::File::open(input_file).expect("Unable to read --input-file");
//!     let mut zip_archive = ZipArchive::new(file_descriptor).unwrap();
//!     let mut zip_file = zip_archive.by_name("data/ni-devices.js").unwrap();
//!     let mut buff = String::new();
//!     zip_file.read_to_string(&mut buff).unwrap();
//!
//!     let json = buff.replacen("window.YTD.ni_devices.part0 = ", "", 1);
//!     let data: Vec<ni_devices::NiDeviceResponseObject> = serde_json::from_str(&json).expect("Unable to parse");
//!
//!     for (index, object) in data.iter().enumerate() {
//!         /* Do stuff with each `niDeviceResponse` entry */
//!         println!("Ni Devices index: {index}");
//!         println!("Messaging device phone number: {}", object.ni_device_response.messaging_device.phone_number);
//!         println!("Messaging device carrier: {}", object.ni_device_response.messaging_device.carrier);
//!         println!("Messaging device updated: {}", object.ni_device_response.messaging_device.updated_date);
//!         println!("Messaging device created: {}", object.ni_device_response.messaging_device.created_date);
//!     }
//! }
//! ```
//!
//! ## Example `twitter-<DATE>-<UID>.zip:data/ni-devices.js` content
//!
//! ```javascript
//! window.YTD.ni_devices.part0 = [
//!   {
//!     "niDeviceResponse" : {
//!       "messagingDevice" : {
//!         "phoneNumber" : "+15551234567",
//!         "carrier" : "us.carriername",
//!         "deviceType" : "Auth",
//!         "updatedDate" : "2021.10.20",
//!         "createdDate" : "2020.02.01"
//!       }
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
/// use chrono::{DateTime, NaiveDate, Utc};
///
/// use twitter_archive::convert::date_year_month_day;
///
/// use twitter_archive::structs::ni_devices::NiDeviceResponseObject;
///
/// let date_string_updated_date = "2021.10.20";
/// let date_native_updated_date = NaiveDate::parse_from_str(&date_string_updated_date, date_year_month_day::FORMAT).unwrap();
/// let date_time_updated_date = DateTime::<Utc>::from_naive_utc_and_offset(date_native_updated_date.into(), Utc);
///
/// let date_string_created_date = "2021.10.20";
/// let date_native_created_date = NaiveDate::parse_from_str(&date_string_created_date, date_year_month_day::FORMAT).unwrap();
/// let date_time_created_date = DateTime::<Utc>::from_naive_utc_and_offset(date_native_created_date.into(), Utc);
///
/// let json = format!(r#"{{
///   "niDeviceResponse": {{
///     "messagingDevice": {{
///       "phoneNumber": "+15551234567",
///       "carrier": "us.carriername",
///       "deviceType": "Auth",
///       "updatedDate": "{date_string_updated_date}",
///       "createdDate": "{date_string_created_date}"
///     }}
///   }}
/// }}"#);
///
/// let data: NiDeviceResponseObject = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.ni_device_response.messaging_device.phone_number, "+15551234567");
/// assert_eq!(data.ni_device_response.messaging_device.carrier, "us.carriername");
/// assert_eq!(data.ni_device_response.messaging_device.device_type, "Auth");
/// assert_eq!(data.ni_device_response.messaging_device.updated_date, date_time_updated_date);
/// assert_eq!(data.ni_device_response.messaging_device.created_date, date_time_created_date);
///
/// // Re-serialize is equivalent to original data
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct NiDeviceResponseObject {
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "niDeviceResponse": {
	///     "messagingDevice": {
	///       "phoneNumber": "+15551234567",
	///       "carrier": "us.carriername",
	///       "deviceType": "Auth",
	///       "updatedDate": "2021.10.20",
	///       "createdDate": "2021.10.20"
	///     }
	///   }
	/// }
	/// ```
	pub ni_device_response: NiDeviceResponse,
}

/// ## Example
///
/// ```
/// use chrono::{DateTime, NaiveDate, Utc};
///
/// use twitter_archive::convert::date_year_month_day;
///
/// use twitter_archive::structs::ni_devices::NiDeviceResponse;
///
/// let date_string_updated_date = "2021.10.20";
/// let date_native_updated_date = NaiveDate::parse_from_str(&date_string_updated_date, date_year_month_day::FORMAT).unwrap();
/// let date_time_updated_date = DateTime::<Utc>::from_naive_utc_and_offset(date_native_updated_date.into(), Utc);
///
/// let date_string_created_date = "2021.10.20";
/// let date_native_created_date = NaiveDate::parse_from_str(&date_string_created_date, date_year_month_day::FORMAT).unwrap();
/// let date_time_created_date = DateTime::<Utc>::from_naive_utc_and_offset(date_native_created_date.into(), Utc);
///
/// let json = format!(r#"{{
///   "messagingDevice": {{
///     "phoneNumber": "+15551234567",
///     "carrier": "us.carriername",
///     "deviceType": "Auth",
///     "updatedDate": "{date_string_updated_date}",
///     "createdDate": "{date_string_created_date}"
///   }}
/// }}"#);
///
/// let data: NiDeviceResponse = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.messaging_device.phone_number, "+15551234567");
/// assert_eq!(data.messaging_device.carrier, "us.carriername");
/// assert_eq!(data.messaging_device.device_type, "Auth");
/// assert_eq!(data.messaging_device.updated_date, date_time_updated_date);
/// assert_eq!(data.messaging_device.created_date, date_time_created_date);
///
/// // Re-serialize is equivalent to original data
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct NiDeviceResponse {
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "messagingDevice": {
	///     "phoneNumber": "+15551234567",
	///     "carrier": "us.carriername",
	///     "deviceType": "Auth",
	///     "updatedDate": "2021.10.20",
	///     "createdDate": "2021.10.20"
	///   }
	/// }
	/// ```
	pub messaging_device: MessagingDevice,
}

/// ## Example
///
/// ```
/// use chrono::{DateTime, NaiveDate, Utc};
///
/// use twitter_archive::convert::date_year_month_day;
///
/// use twitter_archive::structs::ni_devices::MessagingDevice;
///
/// let date_string_updated_date = "2021.10.20";
/// let date_native_updated_date = NaiveDate::parse_from_str(&date_string_updated_date, date_year_month_day::FORMAT).unwrap();
/// let date_time_updated_date = DateTime::<Utc>::from_naive_utc_and_offset(date_native_updated_date.into(), Utc);
///
/// let date_string_created_date = "2021.10.20";
/// let date_native_created_date = NaiveDate::parse_from_str(&date_string_created_date, date_year_month_day::FORMAT).unwrap();
/// let date_time_created_date = DateTime::<Utc>::from_naive_utc_and_offset(date_native_created_date.into(), Utc);
///
/// let json = format!(r#"{{
///   "phoneNumber": "+15551234567",
///   "carrier": "us.carriername",
///   "deviceType": "Auth",
///   "updatedDate": "{date_string_updated_date}",
///   "createdDate": "{date_string_created_date}"
/// }}"#);
///
/// let data: MessagingDevice = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.phone_number, "+15551234567");
/// assert_eq!(data.carrier, "us.carriername");
/// assert_eq!(data.device_type, "Auth");
/// assert_eq!(data.updated_date, date_time_updated_date);
/// assert_eq!(data.created_date, date_time_created_date);
///
/// // Re-serialize is equivalent to original data
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct MessagingDevice {
	/// ## Example JSON data
	///
	/// ```json
	/// { "phoneNumber": "+15551234567" }
	/// ```
	pub phone_number: String,

	/// ## Example JSON data
	///
	/// ```json
	/// { "carrier": "us.carriername" }
	/// ```
	pub carrier: String,

	/// ## Example JSON data
	///
	/// ```json
	/// { "deviceType": "Auth" }
	/// ```
	pub device_type: String,

	/// ## Example JSON data
	///
	/// ```json
	/// { "updatedDate": "2021.10.20" }
	/// ```
	#[serde(with = "convert::date_year_month_day")]
	pub updated_date: DateTime<Utc>,

	/// ## Example JSON data
	///
	/// ```json
	/// { "createdDate": "2021.10.20" }
	/// ```
	#[serde(with = "convert::date_year_month_day")]
	pub created_date: DateTime<Utc>,
}
