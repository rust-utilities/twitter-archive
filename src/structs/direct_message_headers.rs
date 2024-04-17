#!/usr/bin/env rust

//! Tweeter archives as of 2023-08-31 have private data found under;
//!
//!   twitter-<DATE>-<UID>.zip:data/direct-message-headers.js
//!
//! ## Example file reader
//!
//! ```no_build
//! use std::io::Read;
//! use std::{fs, path};
//! use zip::read::ZipArchive;
//!
//! use twitter_archive::structs::direct_message_headers;
//!
//! fn main() {
//!     let input_file = "~/Downloads/twitter-archive.zip";
//!
//!     let file_descriptor = fs::File::open(input_file).expect("Unable to read --input-file");
//!     let mut zip_archive = ZipArchive::new(file_descriptor).unwrap();
//!     let mut zip_file = zip_archive.by_name("data/direct-message-headers.js").unwrap();
//!     let mut buff = String::new();
//!     zip_file.read_to_string(&mut buff).unwrap();
//!
//!     let json = buff.replacen("window.YTD.direct_message_headers.part0 = ", "", 1);
//!     let data: Vec<direct_message_headers::DmConversationObject> = serde_json::from_str(&json).expect("Unable to parse");
//!
//!     for (index_header, object_header) in data.iter().enumerate() {
//!         /* Do stuff with each `DmConversationObject` entry */
//!         println!("Conversation header index: {index_header}");
//!         println!("Conversation ID: {}", object_header.dm_conversation.conversation_id);
//!         for (index_message, object_message) in object_header.dm_conversation.messages.iter().enumerate() {
//!             /* Do stuff with each `object_message` entry */
//!             println!("Message header index: {index_message}");
//!             println!("Created at: {}", object_message.message_create.created_at);
//!             println!("Message ID: {}", object_message.message_create.id);
//!             println!("Sender ID: {}", object_message.message_create.sender_id);
//!             println!("Recipient ID: {}", object_message.message_create.recipient_id);
//!         }
//!     }
//! }
//! ```
//!
//! ## Example `twitter-<DATE>-<UID>.zip:data/direct-message-headers.js` content
//!
//! ```javascript
//! window.YTD.direct_message_headers.part0 = [
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
/// use twitter_archive::structs::direct_message_headers::DmConversationObject;
///
/// let created_at_string = "2023-08-12T17:10:37.000Z";
/// let created_at_native_time = NaiveDateTime::parse_from_str(&created_at_string, date_time_iso_8601::FORMAT).unwrap();
/// let created_at_date_time = DateTime::<Utc>::from_naive_utc_and_offset(created_at_native_time, Utc);
///
/// let json = format!(r#"{{
///   "dmConversation": {{
///     "conversationId": "1111-2222",
///     "messages": [
///       {{
///         "messageCreate": {{
///           "id": "1111111111111111111",
///           "senderId": "2222",
///           "recipientId": "1111",
///           "createdAt": "{created_at_string}"
///         }}
///       }}
///     ]
///   }}
/// }}"#);
///
/// let data: DmConversationObject = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.dm_conversation.conversation_id, "1111-2222");
///
/// assert_eq!(data.dm_conversation.messages.len(), 1);
/// assert_eq!(data.dm_conversation.messages[0].message_create.id, "1111111111111111111");
/// assert_eq!(data.dm_conversation.messages[0].message_create.sender_id, "2222");
/// assert_eq!(data.dm_conversation.messages[0].message_create.recipient_id, "1111");
/// assert_eq!(data.dm_conversation.messages[0].message_create.created_at, created_at_date_time);
///
/// // Re-serialize is equivalent to original data without pretty printing
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct DmConversationObject {
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "dmConversation": {
	///     "conversationId": "1111-2222",
	///     "messages": [
	///       {
	///         "messageCreate": {
	///           "id": "1111111111111111111",
	///           "senderId": "2222",
	///           "recipientId": "1111",
	///           "createdAt": "2023-08-12T17:10:37.000Z"
	///         }
	///       }
	///     ]
	///   }
	/// }
	/// ```
	pub dm_conversation: DmConversation,
}

/// ## Example
///
/// ```
/// use chrono::{DateTime, NaiveDateTime, Utc};
///
/// use twitter_archive::convert::date_time_iso_8601;
///
/// use twitter_archive::structs::direct_message_headers::DmConversation;
///
/// let created_at_string = "2023-08-12T17:10:37.000Z";
/// let created_at_native_time = NaiveDateTime::parse_from_str(&created_at_string, date_time_iso_8601::FORMAT).unwrap();
/// let created_at_date_time = DateTime::<Utc>::from_naive_utc_and_offset(created_at_native_time, Utc);
///
/// let json = format!(r#"{{
///   "conversationId": "1111-2222",
///   "messages": [
///     {{
///       "messageCreate": {{
///         "id": "1111111111111111111",
///         "senderId": "2222",
///         "recipientId": "1111",
///         "createdAt": "{created_at_string}"
///       }}
///     }}
///   ]
/// }}"#);
///
/// let data: DmConversation = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.conversation_id, "1111-2222");
///
/// assert_eq!(data.messages.len(), 1);
/// assert_eq!(data.messages[0].message_create.id, "1111111111111111111");
/// assert_eq!(data.messages[0].message_create.sender_id, "2222");
/// assert_eq!(data.messages[0].message_create.recipient_id, "1111");
/// assert_eq!(data.messages[0].message_create.created_at, created_at_date_time);
///
/// // Re-serialize is equivalent to original data without pretty printing
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct DmConversation {
	/// ## Example JSON data
	///
	/// ```json
	/// { "conversationId": "1111-2222" }
	/// ```
	pub conversation_id: String,

	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "messages": [
	///     {
	///       "messageCreate": {
	///         "id": "1111111111111111111",
	///         "senderId": "2222",
	///         "recipientId": "1111",
	///         "createdAt": "2023-08-12T17:10:37.000Z"
	///       }
	///     }
	///   ]
	/// }
	/// ```
	pub messages: Vec<MessageCreateObject>,
}

/// ## Example
///
/// ```
/// use chrono::{DateTime, NaiveDateTime, Utc};
///
/// use twitter_archive::convert::date_time_iso_8601;
///
/// use twitter_archive::structs::direct_message_headers::MessageCreateObject;
///
/// let created_at_string = "2023-08-12T17:10:37.000Z";
/// let created_at_native_time = NaiveDateTime::parse_from_str(&created_at_string, date_time_iso_8601::FORMAT).unwrap();
/// let created_at_date_time = DateTime::<Utc>::from_naive_utc_and_offset(created_at_native_time, Utc);
///
/// let json = format!(r#"{{
///   "messageCreate": {{
///     "id": "1111111111111111111",
///     "senderId": "2222",
///     "recipientId": "1111",
///     "createdAt": "{created_at_string}"
///   }}
/// }}"#);
///
/// let data: MessageCreateObject = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.message_create.id, "1111111111111111111");
/// assert_eq!(data.message_create.sender_id, "2222");
/// assert_eq!(data.message_create.recipient_id, "1111");
/// assert_eq!(data.message_create.created_at, created_at_date_time);
///
/// // Re-serialize is equivalent to original data without pretty printing
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct MessageCreateObject {
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "messageCreate": {
	///     "id": "1111111111111111111",
	///     "senderId": "2222",
	///     "recipientId": "1111",
	///     "createdAt": "2023-08-12T17:10:37.000Z"
	///   }
	/// }
	/// ```
	pub message_create: MessageCreate,
}

/// ## Example
///
/// ```
/// use chrono::{DateTime, NaiveDateTime, Utc};
///
/// use twitter_archive::convert::date_time_iso_8601;
///
/// use twitter_archive::structs::direct_message_headers::MessageCreate;
///
/// let created_at_string = "2023-08-12T17:10:37.000Z";
/// let created_at_native_time = NaiveDateTime::parse_from_str(&created_at_string, date_time_iso_8601::FORMAT).unwrap();
/// let created_at_date_time = DateTime::<Utc>::from_naive_utc_and_offset(created_at_native_time, Utc);
///
/// let json = format!(r#"{{
///   "id": "1111111111111111111",
///   "senderId": "2222",
///   "recipientId": "1111",
///   "createdAt": "{created_at_string}"
/// }}"#);
///
/// let data: MessageCreate = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.id, "1111111111111111111");
/// assert_eq!(data.sender_id, "2222");
/// assert_eq!(data.recipient_id, "1111");
/// assert_eq!(data.created_at, created_at_date_time);
///
/// // Re-serialize is equivalent to original data without pretty printing
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct MessageCreate {
	/// ## Example JSON data
	///
	/// ```json
	/// { "id": "1111111111111111111" }
	/// ```
	pub id: String,

	/// ID of user sending message
	///
	/// URL formats;
	///
	/// - Desktop: `https://twitter.com/i/user/{sender_id}`
	///
	/// > Note; does **not** work if not logged-in.  Thanks be to Mr. Musk !-D
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "senderId": "2222" }
	/// ```
	pub sender_id: String,

	/// ID of user receiving message
	///
	/// URL formats;
	///
	/// - Desktop: `https://twitter.com/i/user/{recipient_id}`
	///
	/// > Note; does **not** work if not logged-in.  Thanks be to Mr. Musk !-D
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "recipientId": "1111" }
	/// ```
	pub recipient_id: String,

	/// Date time stamp when DM was created
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "createdAt": "2023-08-12T17:10:37.000Z" }
	/// ```
	#[serde(with = "convert::date_time_iso_8601")]
	pub created_at: DateTime<Utc>,
}
