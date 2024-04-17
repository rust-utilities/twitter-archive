#!/usr/bin/env rust

//! Tweeter archives as of 2023-08-31 have private data found under;
//!
//!   twitter-<DATE>-<UID>.zip:data/direct-message-group-headers.js
//!
//! ## Example file reader
//!
//! ```no_build
//! use std::io::Read;
//! use std::{fs, path};
//! use zip::read::ZipArchive;
//!
//! use twitter_archive::structs::direct_message_group_headers;
//!
//! fn main() {
//!     let input_file = "~/Downloads/twitter-archive.zip";
//!
//!     let file_descriptor = fs::File::open(input_file).expect("Unable to read --input-file");
//!     let mut zip_archive = ZipArchive::new(file_descriptor).unwrap();
//!     let mut zip_file = zip_archive.by_name("data/direct-message-group-headers.js").unwrap();
//!     let mut buff = String::new();
//!     zip_file.read_to_string(&mut buff).unwrap();
//!
//!     let json = buff.replacen("window.YTD.direct_message_group_headers.part0 = ", "", 1);
//!     let data: Vec<direct_message_group_headers::DmConversationObject> = serde_json::from_str(&json).expect("Unable to parse");
//!
//!     for (index_header, object_header) in data.iter().enumerate() {
//!         /* Do stuff with each `DmConversationObject` entry */
//!         println!("Conversation header index: {index_header}");
//!         println!("Conversation ID: {}", object_header.dm_conversation.conversation_id);
//!         for (index_message, object_message) in object_header.dm_conversation.messages.iter().enumerate() {
//!             println!("Message event index: {index_message}");
//!             /* Do stuff with each `Message` variant */
//!             match object_message {
//!                 direct_message_group_headers::Message::MessageCreate(message) => {
//!                     println!("ID: {}", message.id);
//!                     println!("Sender ID: {}", message.sender_id);
//!                     println!("Created at: {}", message.created_at);
//!                 }
//!
//!                 direct_message_group_headers::Message::ParticipantsLeave(participants) => {
//!                     println!("Created at: {}", participants.created_at);
//!                     println!("Leaving user IDs: {:?}", participants.user_ids);
//!                 }
//!
//!                 direct_message_group_headers::Message::JoinConversation(join) => {
//!                     println!("Created at: {}", join.created_at);
//!                     println!("Initiating user ID: {}", join.initiating_user_id);
//!                 }
//!             }
//!         }
//!     }
//! }
//! ```
//!
//! ## Example `twitter-<DATE>-<UID>.zip:data/direct-message-group-headers.js` content
//!
//! ```javascript
//! window.YTD.direct_message_group_headers.part0 = [
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
/// use twitter_archive::convert::date_time_iso_8601;
///
/// use twitter_archive::structs::direct_message_group_headers::DmConversationObject;
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
///           "id": "4444444444444444444",
///           "senderId": "222222222",
///           "createdAt": "{created_at_string}"
///         }}
///       }},
///       {{
///         "messageCreate": {{
///           "id": "3333333333333333333",
///           "senderId": "111111111",
///           "createdAt": "{created_at_string}"
///         }}
///       }},
///       {{
///         "participantsLeave": {{
///           "userIds": [
///             "1234",
///             "9876"
///           ],
///           "createdAt": "{created_at_string}"
///         }}
///       }},
///       {{
///         "joinConversation": {{
///           "initiatingUserId": "111111111",
///           "participantsSnapshot": [
///             "222222222",
///             "111111111"
///           ],
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
/// assert_eq!(data.dm_conversation.messages.len(), 4);
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
	///   "dmConversation" : {
	///     "conversationId" : "1111-2222",
	///     "messages" : [
	///        {
	///          "messageCreate" : {
	///            "id" : "4444444444444444444",
	///            "senderId" : "222222222",
	///            "createdAt" : "2023-08-12T17:10:37.000Z"
	///          }
	///        },
	///        {
	///          "messageCreate" : {
	///            "id" : "3333333333333333333",
	///            "senderId" : "111111111",
	///            "createdAt" : "2023-08-12T17:10:37.000Z"
	///          }
	///        },
	///        {
	///          "participantsLeave" : {
	///            "userIds" : [
	///              "1234",
	///              "9876"
	///            ],
	///            "createdAt" : "2023-08-12T17:10:37.000Z"
	///          }
	///        },
	///        {
	///          "joinConversation" : {
	///            "initiatingUserId" : "111111111",
	///            "participantsSnapshot" : [
	///              "222222222",
	///              "111111111"
	///            ],
	///            "createdAt" : "2023-08-12T17:10:37.000Z"
	///          }
	///        }
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
/// use twitter_archive::structs::direct_message_group_headers::DmConversation;
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
///         "id": "4444444444444444444",
///         "senderId": "222222222",
///         "createdAt": "{created_at_string}"
///       }}
///     }},
///     {{
///       "messageCreate": {{
///         "id": "3333333333333333333",
///         "senderId": "111111111",
///         "createdAt": "{created_at_string}"
///       }}
///     }},
///     {{
///       "participantsLeave": {{
///         "userIds": [
///           "1234",
///           "9876"
///         ],
///         "createdAt": "{created_at_string}"
///       }}
///     }},
///     {{
///       "joinConversation": {{
///         "initiatingUserId": "111111111",
///         "participantsSnapshot": [
///           "222222222",
///           "111111111"
///         ],
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
/// assert_eq!(data.messages.len(), 4);
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
	///      {
	///        "messageCreate": {
	///          "id": "4444444444444444444",
	///          "senderId": "222222222",
	///          "createdAt": "2023-08-12T17:10:37.000Z"
	///        }
	///      },
	///      {
	///        "messageCreate": {
	///          "id": "3333333333333333333",
	///          "senderId": "111111111",
	///          "createdAt": "2023-08-12T17:10:37.000Z"
	///        }
	///      },
	///      {
	///        "participantsLeave": {
	///          "userIds": [
	///            "1234",
	///            "9876"
	///          ],
	///          "createdAt": "2023-08-12T17:10:37.000Z"
	///        }
	///      },
	///      {
	///        "joinConversation": {
	///          "initiatingUserId": "111111111",
	///          "participantsSnapshot": [
	///            "222222222",
	///            "111111111"
	///          ],
	///          "createdAt": "2023-08-12T17:10:37.000Z"
	///        }
	///      }
	///   ]
	/// }"#);
	/// ```
	pub messages: Vec<Message>,
}

/// Because, for reasons, the Twitter devs decided to create a list of messages that contains one
/// data structure, at the very end, that be not like the others we must leverage a Rust `enum`
///
/// ## Example
///
/// ```
/// use chrono::{DateTime, NaiveDateTime, Utc};
///
/// use twitter_archive::convert::date_time_iso_8601;
///
/// use twitter_archive::structs::direct_message_group_headers::Message;
///
/// let created_at_string = "2023-08-12T17:10:37.000Z";
/// let created_at_native_time = NaiveDateTime::parse_from_str(&created_at_string, date_time_iso_8601::FORMAT).unwrap();
/// let created_at_date_time = DateTime::<Utc>::from_naive_utc_and_offset(created_at_native_time, Utc);
///
/// let json = format!(r#"[
///   {{
///     "messageCreate": {{
///       "id": "4444444444444444444",
///       "senderId": "222222222",
///       "createdAt": "{created_at_string}"
///     }}
///   }},
///   {{
///     "messageCreate": {{
///       "id": "3333333333333333333",
///       "senderId": "111111111",
///       "createdAt": "{created_at_string}"
///     }}
///   }},
///   {{
///     "participantsLeave": {{
///       "userIds": [
///         "1234",
///         "9876"
///       ],
///       "createdAt": "{created_at_string}"
///     }}
///   }},
///   {{
///     "joinConversation": {{
///       "initiatingUserId": "111111111",
///       "participantsSnapshot": [
///         "222222222",
///         "111111111"
///       ],
///       "createdAt": "{created_at_string}"
///     }}
///   }}
/// ]"#);
///
/// let data: Vec<Message> = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.len(), 4);
///
/// if let Message::MessageCreate(message_create) = &data[0] {
///     assert_eq!(message_create.sender_id, "222222222");
///     assert_eq!(message_create.id, "4444444444444444444");
///     assert_eq!(message_create.created_at, created_at_date_time);
/// }
///
/// if let Message::MessageCreate(message_create) = &data.get(1).unwrap() {
///     assert_eq!(message_create.sender_id, "111111111");
///     assert_eq!(message_create.id, "3333333333333333333");
///     assert_eq!(message_create.created_at, created_at_date_time);
/// }
///
/// if let Message::ParticipantsLeave(message_create) = &data.get(2).unwrap() {
///     assert_eq!(message_create.user_ids[0], "1234");
///     assert_eq!(message_create.user_ids[1], "9876");
///     assert_eq!(message_create.created_at, created_at_date_time);
/// }
///
/// if let Some(Message::JoinConversation(join_conversation)) = &data.last() {
///     assert_eq!(join_conversation.initiating_user_id, "111111111");
///     assert_eq!(join_conversation.participants_snapshot.len(), 2);
///     assert_eq!(join_conversation.participants_snapshot[0], "222222222");
///     assert_eq!(join_conversation.participants_snapshot[1], "111111111");
///     assert_eq!(join_conversation.created_at, created_at_date_time);
/// }
///
/// // Re-serialize is equivalent to original data without pretty printing
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[serde(rename_all = "camelCase")]
pub enum Message {
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "id": "1111111111111111111",
	///   "senderId": "2222",
	///   "createdAt": "2020-01-20T21:42:09.068Z"
	/// }
	/// ```
	MessageCreate(MessageCreate),

	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "userIds": [
	///     "1234",
	///     "9876"
	///   ],
	///   "createdAt": "2020-01-20T21:42:09.068Z"
	/// }
	/// ```
	ParticipantsLeave(direct_message::ParticipantsLeave),

	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "initiatingUserId": "1111111111111111111",
	///   "participantsSnapshot": [
	///     "2222",
	///     "3333",
	///     "4444"
	///   ],
	///   "createdAt": "2023-08-12T17:10:37.000Z"
	/// }
	/// ```
	JoinConversation(direct_message::JoinConversation),
}

/// ## Example
///
/// ```
/// use chrono::{DateTime, NaiveDateTime, Utc};
///
/// use twitter_archive::convert::date_time_iso_8601;
///
/// use twitter_archive::structs::direct_message_group_headers::MessageCreate;
///
/// let created_at_string = "2023-08-12T17:10:37.000Z";
/// let created_at_native_time = NaiveDateTime::parse_from_str(&created_at_string, date_time_iso_8601::FORMAT).unwrap();
/// let created_at_date_time = DateTime::<Utc>::from_naive_utc_and_offset(created_at_native_time, Utc);
///
/// let json = format!(r#"{{
///   "id": "1111111111111111111",
///   "senderId": "2222",
///   "createdAt": "{created_at_string}"
/// }}"#);
///
/// let data: MessageCreate = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.id, "1111111111111111111");
/// assert_eq!(data.sender_id, "2222");
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
	/// - Desktop: https://twitter.com/i/user/{sender_id}
	///
	/// > Note; does **not** work if not logged-in.  Thanks be to Mr. Musk !-D
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "senderId": "2222" }
	/// ```
	pub sender_id: String,

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
