#!/usr/bin/env rust

//! Tweeter archives as of 2023-08-31 have public tweets found under;
//!
//!   twitter-<DATE>-<UID>.zip:data/direct-messages.js
//!
//! ## Example file reader
//!
//! ```no_build
//! use std::io::Read;
//! use std::{fs, path};
//! use zip::read::ZipArchive;
//!
//! use twitter_archive::structs::direct_messages;
//!
//! fn main() {
//!     let input_file = "path/to/twitter.zip";
//!
//!     let file_descriptor = fs::File::open(input_file).expect("Unable to read --input-file");
//!     let mut zip_archive = ZipArchive::new(file_descriptor).unwrap();
//!     let mut zip_file = zip_archive.by_name("data/direct-messages.js").unwrap();
//!     let mut buff = String::new();
//!     zip_file.read_to_string(&mut buff).unwrap();
//!
//!     let json = buff.replacen("window.YTD.direct_messages.part0 = ", "", 1);
//!     let data: Vec<direct_messages::DmConversationObject> = serde_json::from_str(&json).expect("Unable to parse");
//!
//!     for (index_conversation, object_conversation) in data.iter().enumerate() {
//!         let messages = &object_conversation.dm_conversation.messages;
//!         /* Do stuff with each conversation and message */
//!         for (index_message, object_message) in messages.iter().enumerate() {
//!             let message = &object_message.message_create;
//!             println!("{index_conversation} -- {index_message}");
//!             println!("{} -> {}", message.sender_id, message.recipient_id);
//!             println!("Created at: {}", message.created_at);
//!             println!("vvv Content\n{}\n^^^ Content", message.text);
//!         }
//!     }
//! }
//! ```
//!
//! ## Example `twitter-<DATE>-<UID>.zip:data/direct_messages.js` content
//!
//! ```javascript
//! window.YTD.direct_messages.part0 = [
//! ]
//! ```

use chrono::{DateTime, Utc};
use derive_more::Display;
use serde::{Deserialize, Serialize};

use crate::convert;
use crate::structs::direct_message;

/// ## Example
///
/// ```
/// use chrono::{DateTime, NaiveDateTime, Utc};
///
/// use twitter_archive::structs::direct_messages::DmConversationObject;
/// use twitter_archive::convert::date_time_iso_8601::FORMAT;
///
/// let created_at_string = "2020-01-20T21:42:09.068Z";
/// let created_at_native_time = NaiveDateTime::parse_from_str(&created_at_string, FORMAT).unwrap();
/// let created_at_date_time = DateTime::<Utc>::from_naive_utc_and_offset(created_at_native_time, Utc);
///
/// let json = format!(r#"{{
///   "dmConversation": {{
///     "conversationId": "111111111-222222222",
///     "messages": [
///       {{
///         "messageCreate": {{
///           "recipientId": "222222222",
///           "reactions": [],
///           "urls": [],
///           "text": "Salutations!",
///           "mediaUrls": [],
///           "senderId": "111111111",
///           "id": "3333333333333333333",
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
/// assert_eq!(data.dm_conversation.conversation_id, "111111111-222222222");
/// assert_eq!(data.dm_conversation.messages.len(), 1);
///
/// // Re-serialize is equivalent to original data
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct DmConversationObject {
	/// Why they wrapped a list of conversations within unnecessary object label is anyone's guess
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "dmConversation": {
	///     "conversationId": "111111111-222222222",
	///     "messages": [
	///       {
	///         "messageCreate": {
	///           "recipientId": "222222222",
	///           "reactions": [],
	///           "urls": [],
	///           "text": "Salutations!",
	///           "mediaUrls": [],
	///           "senderId": "111111111",
	///           "id": "3333333333333333333",
	///           "createdAt": "2020-01-20T21:42:09.068Z"
	///         }
	///       }
	///     ]
	///   }
	/// }
	/// ```
	pub dm_conversation: DMConversation,
}

/// ## Example
///
/// ```
/// use chrono::{DateTime, NaiveDateTime, Utc};
///
/// use twitter_archive::structs::direct_messages::DMConversation;
/// use twitter_archive::convert::date_time_iso_8601::FORMAT;
///
/// let created_at_string = "2020-01-20T21:42:09.068Z";
/// let created_at_native_time = NaiveDateTime::parse_from_str(&created_at_string, FORMAT).unwrap();
/// let created_at_date_time = DateTime::<Utc>::from_naive_utc_and_offset(created_at_native_time, Utc);
///
/// let json = format!(r#"{{
///   "conversationId": "111111111-222222222",
///   "messages": [
///     {{
///       "messageCreate": {{
///         "recipientId": "222222222",
///         "reactions": [],
///         "urls": [],
///         "text": "Salutations!",
///         "mediaUrls": [],
///         "senderId": "111111111",
///         "id": "3333333333333333333",
///         "createdAt": "{created_at_string}"
///       }}
///     }}
///   ]
/// }}"#);
///
/// let data: DMConversation = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.conversation_id, "111111111-222222222");
/// assert_eq!(data.messages.len(), 1);
///
/// // Re-serialize is equivalent to original data
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct DMConversation {
	/// This seems to be built by concatenating, with hyphen (`-`) separator, from the following values;
	///
	/// - `direct_messages[].dmConversation.messages[].messageCreate.recipientId`
	/// - `direct_messages[].dmConversation.messages[].messageCreate.senderId`
	///
	/// Direct URL maybe linked directly via; `https://twitter.com/messages/{conversation_id}`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "conversationId": "111111111-222222222" }
	/// ```
	pub conversation_id: String,

	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "messages": [
	///     {
	///       "messageCreate": {
	///         "recipientId": "222222222",
	///         "reactions": [],
	///         "urls": [],
	///         "text": "Salutations!",
	///         "mediaUrls": [],
	///         "senderId": "111111111",
	///         "id": "3333333333333333333",
	///         "createdAt": "2020-01-20T21:42:09.068Z"
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
/// use twitter_archive::structs::direct_messages::MessageCreateObject;
/// use twitter_archive::convert::date_time_iso_8601::FORMAT;
///
/// let created_at_string = "2020-01-20T21:42:09.068Z";
/// let created_at_native_time = NaiveDateTime::parse_from_str(&created_at_string, FORMAT).unwrap();
/// let created_at_date_time = DateTime::<Utc>::from_naive_utc_and_offset(created_at_native_time, Utc);
///
/// let json = format!(r#"{{
///   "messageCreate": {{
///     "recipientId": "222222222",
///     "reactions": [],
///     "urls": [],
///     "text": "Salutations!",
///     "mediaUrls": [],
///     "senderId": "111111111",
///     "id": "3333333333333333333",
///     "createdAt": "{created_at_string}"
///   }}
/// }}"#);
///
/// let data: MessageCreateObject = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.message_create.recipient_id, "222222222");
/// assert_eq!(data.message_create.reactions.len(), 0);
/// assert_eq!(data.message_create.urls.len(), 0);
/// assert_eq!(data.message_create.media_urls.len(), 0);
/// assert_eq!(data.message_create.sender_id, "111111111");
/// assert_eq!(data.message_create.id, "3333333333333333333");
/// assert_eq!(data.message_create.created_at, created_at_date_time);
///
/// // Re-serialize is equivalent to original data
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct MessageCreateObject {
	/// Similar to Tweets list the list of messages are wrapped by an additional layer indirection
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "messageCreate": {
	///     "recipientId": "222222222",
	///     "reactions": [],
	///     "urls": [],
	///     "text": "Salutations!",
	///     "mediaUrls": [],
	///     "senderId": "111111111",
	///     "id": "3333333333333333333",
	///     "createdAt": "2020-01-20T21:42:09.068Z"
	///   }
	/// }
	/// ```
	pub message_create: MessageCreate,
}

/// Note, other than the addition of a `recipient_id` (`recipientId`) key, this is identical to
/// `MessageCreate` from `twitter_archive::structs::direct_messages_group`
///
/// ## Example
///
/// ```
/// use chrono::{DateTime, NaiveDateTime, Utc};
///
/// use twitter_archive::structs::direct_messages::MessageCreate;
/// use twitter_archive::convert::date_time_iso_8601::FORMAT;
///
/// let created_at_string = "2020-01-20T21:42:09.068Z";
/// let created_at_native_time = NaiveDateTime::parse_from_str(&created_at_string, FORMAT).unwrap();
/// let created_at_date_time = DateTime::<Utc>::from_naive_utc_and_offset(created_at_native_time, Utc);
///
/// let json = format!(r#"{{
///   "recipientId": "222222222",
///   "reactions": [],
///   "urls": [],
///   "text": "Salutations!",
///   "mediaUrls": [],
///   "senderId": "111111111",
///   "id": "3333333333333333333",
///   "createdAt": "{created_at_string}"
/// }}"#);
///
/// let data: MessageCreate = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.recipient_id, "222222222");
/// assert_eq!(data.reactions.len(), 0);
/// assert_eq!(data.urls.len(), 0);
/// assert_eq!(data.media_urls.len(), 0);
/// assert_eq!(data.sender_id, "111111111");
/// assert_eq!(data.id, "3333333333333333333");
/// assert_eq!(data.created_at, created_at_date_time);
///
/// // Re-serialize is equivalent to original data
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct MessageCreate {
	/// URL formats;
	///
	/// - Desktop: https://twitter.com/i/user/{recipient_id}
	///
	/// > Note; does **not** work if not logged-in.  Thanks be to Mr. Musk !-D
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "recipientId": "222222222" }
	/// ```
	pub recipient_id: String,

	/// List data about who, when, and what reactions were had about a given message
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "reactions": [] }
	/// ```
	pub reactions: Vec<direct_message::MessageCreateReaction>,

	/// List of mangled/tracking URL data that includes originally written link too
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "urls": [] }
	/// ```
	pub urls: Vec<direct_message::MessageCreateUrl>,

	/// Content of message with embedded newlines `\n` where applicable
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "text": "Salutations!" }
	/// ```
	pub text: String,

	/// Finally a simple array of strings without unnecessary modifications or parsing required!
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "mediaUrls": [] }
	/// ```
	pub media_urls: Vec<String>,

	/// URL formats;
	///
	/// - Desktop: https://twitter.com/i/user/{sender_id}
	///
	/// > Note; does **not** work if not logged-in.  Thanks be to Mr. Musk !-D
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "senderId": "111111111" }
	/// ```
	pub sender_id: String,

	/// Possibly unique ID across all conversations and messages
	///
	/// Report url maybe built via; `https://twitter.com/i/report/dm_message/{id}`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "id": "3333333333333333333" }
	/// ```
	pub id: String,

	/// Date time-stamp of when message was originally sent
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "createdAt": "2020-01-20T21:42:09.068Z" }
	/// ```
	#[serde(with = "convert::date_time_iso_8601")]
	pub created_at: DateTime<Utc>,
}
