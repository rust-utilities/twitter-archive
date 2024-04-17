#!/usr/bin/env rust

//! Tweeter archives as of 2023-08-31 have public connected_application found under;
//!
//!   twitter-<DATE>-<UID>.zip:data/connected-application.js
//!
//! ## Example file reader
//!
//! ```no_build
//! use std::io::Read;
//! use std::{fs, path};
//! use zip::read::ZipArchive;
//!
//! use twitter_archive::structs::connected_application;
//!
//! fn main() {
//!     let input_file = "~/Downloads/twitter-archive.zip";
//!
//!     let file_descriptor = fs::File::open(input_file).expect("Unable to read --input-file");
//!     let mut zip_archive = ZipArchive::new(file_descriptor).unwrap();
//!     let mut zip_file = zip_archive.by_name("data/connected-application.js").unwrap();
//!     let mut buff = String::new();
//!     zip_file.read_to_string(&mut buff).unwrap();
//!
//!     let json = buff.replacen("window.YTD.connected_application.part0 = ", "", 1);
//!     let data: Vec<connected_application::ConnectedApplicationObject> = serde_json::from_str(&json).expect("Unable to parse");
//!
//!     for (index, object) in data.iter().enumerate() {
//!         /* Do stuff with each deleted Tweet */
//!         println!("Connected application index: {index}");
//!         println!("Organization name: {}", object.connected_application.organization.name);
//!         println!("Organization URL: {}", object.connected_application.organization.url);
//!         println!("Organization privacy policy: {}", object.connected_application.organization.privacy_policy_url);
//!         println!("Organization terms and conditions: {}", object.connected_application.organization.terms_and_conditions_url);
//!         println!("Description: {}", object.connected_application.description);
//!         println!("Permissions: {:?}", object.connected_application.permissions);
//!         println!("Approved at: {}", object.connected_application.approved_at);
//!         println!("ID: {}", object.connected_application.id);
//!     }
//! }
//! ```
//!
//! ## Example `twitter-<DATE>-<UID>.zip:data/connected-application.js` content
//!
//! ```javascript
//! window.YTD.connected_application.part0 = [
//!   {
//!     "connectedApplication" : {
//!       "organization" : {
//!         "name" : "Medium",
//!         "url" : "https://medium.com",
//!         "privacyPolicyUrl" : "https://medium.com/policy/medium-privacy-policy-f03bf92035c9",
//!         "termsAndConditionsUrl" : "https://medium.com/policy/medium-terms-of-service-9db0094a1e0f"
//!       },
//!       "name" : "Medium",
//!       "description" : "Evolving publishing",
//!       "permissions" : [
//!         "read",
//!         "write",
//!         "emailaddress"
//!       ],
//!       "approvedAt" : "2020-01-20T21:42:09.068Z",
//!       "id" : "1111111"
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
/// use twitter_archive::structs::connected_application::ConnectedApplicationObject;
///
/// let approved_at_string = "2020-01-20T21:42:09.068Z";
/// let approved_at_native_time = NaiveDateTime::parse_from_str(&approved_at_string, FORMAT).unwrap();
/// let approved_at_date_time = DateTime::<Utc>::from_naive_utc_and_offset(approved_at_native_time, Utc);
///
/// let json = format!(r#"{{
///   "connectedApplication" : {{
///     "organization" : {{
///       "name" : "Example",
///       "url" : "https://example.com",
///       "privacyPolicyUrl" : "https://example.com/policy/example-privacy-policy",
///       "termsAndConditionsUrl" : "https://example.com/policy/medium-terms-of-service"
///     }},
///     "name" : "Example",
///     "description" : "Example-description",
///     "permissions" : [
///       "read",
///       "write",
///       "emailaddress"
///     ],
///     "approvedAt" : "{approved_at_string}",
///     "id" : "1111111"
///   }}
/// }}"#);
///
/// let data: ConnectedApplicationObject = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.connected_application.organization.name, "Example");
/// assert_eq!(data.connected_application.organization.url, "https://example.com");
/// assert_eq!(data.connected_application.organization.privacy_policy_url, "https://example.com/policy/example-privacy-policy");
/// assert_eq!(data.connected_application.organization.terms_and_conditions_url, "https://example.com/policy/medium-terms-of-service");
///
/// assert_eq!(data.connected_application.name, "Example");
/// assert_eq!(data.connected_application.description, "Example-description");
///
/// assert_eq!(data.connected_application.permissions.len(), 3);
/// assert_eq!(data.connected_application.permissions[0], "read");
/// assert_eq!(data.connected_application.permissions[1], "write");
/// assert_eq!(data.connected_application.permissions[2], "emailaddress");
///
/// assert_eq!(data.connected_application.approved_at, approved_at_date_time);
/// assert_eq!(data.connected_application.id, "1111111");
///
/// // Re-serialize is equivalent to original data without pretty printing
/// // assert_eq!(serde_json::to_string(&data).unwrap(), json.replace("\n", "").replace(" ", ""));
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct ConnectedApplicationObject {
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "connectedApplication" : {
	///     "organization" : {
	///       "name" : "Example",
	///       "url" : "https://example.com",
	///       "privacyPolicyUrl" : "https://example.com/policy/example-privacy-policy",
	///       "termsAndConditionsUrl" : "https://example.com/policy/medium-terms-of-service"
	///     },
	///     "name" : "Example",
	///     "description" : "Example-description",
	///     "permissions" : [
	///       "read",
	///       "write",
	///       "emailaddress"
	///     ],
	///     "approvedAt" : "2020-01-20T21:42:09.068Z",
	///     "id" : "1111111"
	///   }
	/// }
	/// ```
	pub connected_application: ConnectedApplication,
}

/// ## Example
///
/// ```
/// use chrono::{DateTime, NaiveDateTime, Utc};
///
/// use twitter_archive::convert::date_time_iso_8601::FORMAT;
///
/// use twitter_archive::structs::connected_application::ConnectedApplication;
///
/// let approved_at_string = "2020-01-20T21:42:09.068Z";
/// let approved_at_native_time = NaiveDateTime::parse_from_str(&approved_at_string, FORMAT).unwrap();
/// let approved_at_date_time = DateTime::<Utc>::from_naive_utc_and_offset(approved_at_native_time, Utc);
///
/// let json = format!(r#"{{
///   "organization" : {{
///     "name" : "Example",
///     "url" : "https://example.com",
///     "privacyPolicyUrl" : "https://example.com/policy/example-privacy-policy",
///     "termsAndConditionsUrl" : "https://example.com/policy/medium-terms-of-service"
///   }},
///   "name" : "Example",
///   "description" : "Example-description",
///   "permissions" : [
///     "read",
///     "write",
///     "emailaddress"
///   ],
///   "approvedAt" : "{approved_at_string}",
///   "id" : "1111111"
/// }}"#);
///
/// let data: ConnectedApplication = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.organization.name, "Example");
/// assert_eq!(data.organization.url, "https://example.com");
/// assert_eq!(data.organization.privacy_policy_url, "https://example.com/policy/example-privacy-policy");
/// assert_eq!(data.organization.terms_and_conditions_url, "https://example.com/policy/medium-terms-of-service");
///
/// assert_eq!(data.name, "Example");
/// assert_eq!(data.description, "Example-description");
///
/// assert_eq!(data.permissions.len(), 3);
/// assert_eq!(data.permissions[0], "read");
/// assert_eq!(data.permissions[1], "write");
/// assert_eq!(data.permissions[2], "emailaddress");
///
/// assert_eq!(data.approved_at, approved_at_date_time);
/// assert_eq!(data.id, "1111111");
///
/// // Re-serialize is equivalent to original data without pretty printing
/// // assert_eq!(serde_json::to_string(&data).unwrap(), json.replace("\n", "").replace(" ", ""));
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct ConnectedApplication {
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "organization" : {
	///     "name" : "Example",
	///     "url" : "https://example.com",
	///     "privacyPolicyUrl" : "https://example.com/policy/example-privacy-policy",
	///     "termsAndConditionsUrl" : "https://example.com/policy/medium-terms-of-service"
	///   }
	/// }
	/// ```
	pub organization: Organization,

	/// Human readable name of application
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "name" : "Example" }
	/// ```
	pub name: String,

	/// Human readable description of application
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "description" : "Example-description" }
	/// ```
	pub description: String,

	/// List of permissions provided to application
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "permissions" : [
	///     "read",
	///     "write",
	///     "emailaddress"
	///   ]
	/// }
	/// ```
	pub permissions: Vec<String>,

	/// ## Example JSON data
	///
	/// ```json
	/// { "approvedAt" : "2020-01-20T21:42:09.068Z" }
	/// ```
	#[serde(with = "convert::date_time_iso_8601")]
	pub approved_at: DateTime<Utc>,

	/// ID of application or maybe ID of account permitting application?
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "id" : "1111111" }
	/// ```
	pub id: String,
}

/// ## Example
///
/// ```
/// use twitter_archive::structs::connected_application::Organization;
///
/// let json = r#"{
///   "name" : "Example",
///   "url" : "https://example.com",
///   "privacyPolicyUrl" : "https://example.com/policy/example-privacy-policy",
///   "termsAndConditionsUrl" : "https://example.com/policy/medium-terms-of-service"
/// }"#;
///
/// let data: Organization = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.name, "Example");
/// assert_eq!(data.url, "https://example.com");
/// assert_eq!(data.privacy_policy_url, "https://example.com/policy/example-privacy-policy");
/// assert_eq!(data.terms_and_conditions_url, "https://example.com/policy/medium-terms-of-service");
///
/// // Re-serialize is equivalent to original data without pretty printing
/// // assert_eq!(serde_json::to_string(&data).unwrap(), json.replace("\n", "").replace(" ", ""));
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct Organization {
	/// Human readable name of application
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "name" : "Example" }
	/// ```
	pub name: String,

	/// Web address of application, usually the "home" page
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "url" : "https://example.com" }
	/// ```
	pub url: String,

	/// Web address of privacy policy for application
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "privacyPolicyUrl" : "https://example.com/policy/example-privacy-policy" }
	/// ```
	pub privacy_policy_url: String,

	/// Web address of terms and conditions policy for application
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "termsAndConditionsUrl" : "https://example.com/policy/medium-terms-of-service" }
	/// ```
	pub terms_and_conditions_url: String,
}
