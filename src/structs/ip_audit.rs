#!/usr/bin/env rust

//! Tweeter archives as of 2023-08-31 have private data found under;
//!
//!   twitter-<DATE>-<UID>.zip:data/ip-audit.js
//!
//! ## Example file reader
//!
//! ```no_build
//! use std::io::Read;
//! use std::{fs, path};
//! use zip::read::ZipArchive;
//!
//! use twitter_archive::structs::ip_audit;
//!
//! fn main() {
//!     let input_file = "~/Downloads/twitter-archive.zip";
//!
//!     let file_descriptor = fs::File::open(input_file).expect("Unable to read --input-file");
//!     let mut zip_archive = ZipArchive::new(file_descriptor).unwrap();
//!     let mut zip_file = zip_archive.by_name("data/ip-audit.js").unwrap();
//!     let mut buff = String::new();
//!     zip_file.read_to_string(&mut buff).unwrap();
//!
//!     let json = buff.replacen("window.YTD.ip_audit.part0 = ", "", 1);
//!     let data: Vec<ip_audit::IpAuditObject> = serde_json::from_str(&json).expect("Unable to parse");
//!
//!     for (index, object) in data.iter().enumerate() {
//!         /* Do stuff with each `RegisteredDevices` entry */
//!         println!("IP audit index: {index}");
//!         println!("Account ID: {}", object.ip_audit.account_id);
//!         println!("Created at: {}", object.ip_audit.created_at);
//!         println!("Login IP: {}", object.ip_audit.login_ip);
//!     }
//! }
//! ```
//!
//! ## Example `twitter-<DATE>-<UID>.zip:data/ip-audit.js` content
//!
//! ```javascript
//! window.YTD.ip_audit.part0 = [
//!   {
//!     "ipAudit" : {
//!       "accountId" : "111111111",
//!       "createdAt" : "2023-05-30T13:31:42.908Z",
//!       "loginIp" : "127.0.0.1"
//!     }
//!   },
//!   {
//!     "ipAudit" : {
//!       "accountId" : "111111111",
//!       "createdAt" : "2023-04-30T13:31:42.908Z",
//!       "loginIp" : "127.0.0.1"
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
/// use twitter_archive::structs::ip_audit::IpAuditObject;
///
/// let created_at_string = "2023-05-30T13:31:42.908Z";
/// let created_at_native_time = NaiveDateTime::parse_from_str(&created_at_string, date_time_iso_8601::FORMAT).unwrap();
/// let created_at_date_time = DateTime::<Utc>::from_naive_utc_and_offset(created_at_native_time, Utc);
///
/// let json = format!(r#"{{
///   "ipAudit": {{
///     "accountId": "111111111",
///     "createdAt": "{created_at_string}",
///     "loginIp": "127.0.0.1"
///   }}
/// }}"#);
///
/// let data: IpAuditObject = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.ip_audit.account_id, "111111111");
/// assert_eq!(data.ip_audit.created_at, created_at_date_time);
/// assert_eq!(data.ip_audit.login_ip, "127.0.0.1");
///
/// // Re-serialize is equivalent to original data
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct IpAuditObject {
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "ipAudit": {
	///     "accountId": "111111111",
	///     "createdAt": "2023-05-30T13:31:42.908Z",
	///     "loginIp": "127.0.0.1"
	///   }
	/// }
	/// ```
	pub ip_audit: IpAudit,
}

/// ## Example
///
/// ```
/// use chrono::{DateTime, NaiveDateTime, Utc};
///
/// use twitter_archive::convert::date_time_iso_8601;
///
/// use twitter_archive::structs::ip_audit::IpAudit;
///
/// let created_at_string = "2023-05-30T13:31:42.908Z";
/// let created_at_native_time = NaiveDateTime::parse_from_str(&created_at_string, date_time_iso_8601::FORMAT).unwrap();
/// let created_at_date_time = DateTime::<Utc>::from_naive_utc_and_offset(created_at_native_time, Utc);
///
/// let json = format!(r#"{{
///   "accountId": "111111111",
///   "createdAt": "{created_at_string}",
///   "loginIp": "127.0.0.1"
/// }}"#);
///
/// let data: IpAudit = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.account_id, "111111111");
/// assert_eq!(data.created_at, created_at_date_time);
/// assert_eq!(data.login_ip, "127.0.0.1");
///
/// // Re-serialize is equivalent to original data
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct IpAudit {
	/// ## Example JSON data
	///
	/// ```json
	/// { "accountId": "111111111" }
	/// ```
	pub account_id: String,

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
	/// { "loginIp": "127.0.0.1" }
	/// ```
	pub login_ip: String,
}
