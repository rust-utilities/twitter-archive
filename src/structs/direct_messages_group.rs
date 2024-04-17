#!/usr/bin/env rust

//! Tweeter archives as of 2023-08-31 have private data found under;
//!
//!   twitter-<DATE>-<UID>.zip:data/direct-messages-group.js
//!
//! ## Example file reader
//!
//! ```no_build
//! use std::io::Read;
//! use std::{fs, path};
//! use zip::read::ZipArchive;
//!
//! use twitter_archive::structs::direct_messages_group;
//!
//! fn main() {
//!     let input_file = "~/Downloads/twitter-archive.zip";
//!
//!     let file_descriptor = fs::File::open(input_file).expect("Unable to read --input-file");
//!     let mut zip_archive = ZipArchive::new(file_descriptor).unwrap();
//!     let mut zip_file = zip_archive.by_name("data/direct-messages-group.js").unwrap();
//!     let mut buff = String::new();
//!     zip_file.read_to_string(&mut buff).unwrap();
//!
//!     let json = buff.replacen("window.YTD.direct_messages_group.part0 = ", "", 1);
//!     let data: Vec<direct_messages_group::DmConversationObject> = serde_json::from_str(&json).expect("Unable to parse");
//!
//!     for (index_conversation, object_conversation) in data.iter().enumerate() {
//!         /* Do stuff with each `DmConversationObject` entry */
//!         println!("Group conversation index: {index_conversation}");
//!         println!("Group conversation ID: {}", object_conversation.dm_conversation.conversation_id);
//!         for (index_message, object_message) in object_conversation.dm_conversation.messages.iter().enumerate() {
//!             println!("Message event index: {index_message}");
//!             /* Do stuff with each `Message` variant */
//!             match object_message {
//!                 direct_messages_group::Message::MessageCreate(message) => {
//!                     println!("Created at: {}", message.created_at);
//!                     println!("Sender ID: {}", message.sender_id);
//!                     println!("vvv Message\n{}\n^^^ Message", message.text);
//!                 }
//!
//!                 direct_messages_group::Message::ParticipantsLeave(participants) => {
//!                     println!("Created at: {}", participants.created_at);
//!                     println!("Leaving user IDs: {:?}", participants.user_ids);
//!                 }
//!
//!                 direct_messages_group::Message::JoinConversation(join) => {
//!                     println!("Created at: {}", join.created_at);
//!                     println!("Initiating user ID: {}", join.initiating_user_id);
//!                 }
//!             }
//!         }
//!     }
//! }
//! ```
//!
//! ## Example `twitter-<DATE>-<UID>.zip:data/direct-messages-group.js` content
//!
//! ```javascript
//! window.YTD.direct_messages_group.part0 = [
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
/// use twitter_archive::structs::direct_messages_group::DmConversationObject;
///
/// let created_at_string = "2023-08-12T17:10:37.000Z";
/// let created_at_native_time = NaiveDateTime::parse_from_str(&created_at_string, date_time_iso_8601::FORMAT).unwrap();
/// let created_at_date_time = DateTime::<Utc>::from_naive_utc_and_offset(created_at_native_time, Utc);
///
/// let json = format!(r#"{{
///   "dmConversation": {{
///     "conversationId": "6666666666666666666",
///     "messages": [
///       {{
///         "messageCreate": {{
///           "reactions": [],
///           "urls": [],
///           "text": "Sup!?",
///           "mediaUrls": [],
///           "senderId": "222222222",
///           "id": "4444444444444444444",
///           "createdAt": "{created_at_string}"
///         }}
///       }},
///       {{
///         "messageCreate": {{
///           "reactions": [],
///           "urls": [],
///           "text": "Salutations!",
///           "mediaUrls": [],
///           "senderId": "111111111",
///           "id": "3333333333333333333",
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
/// assert_eq!(data.dm_conversation.conversation_id, "6666666666666666666");
/// assert_eq!(data.dm_conversation.messages.len(), 4);
///
/// // Re-serialize is equivalent to original data
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
	///     "conversationId": "6666666666666666666",
	///     "messages": [
	///       {
	///         "messageCreate": {
	///           "reactions": [ ],
	///           "urls": [ ],
	///           "text": "Sup!?",
	///           "mediaUrls": [ ],
	///           "senderId": "222222222",
	///           "id": "4444444444444444444",
	///           "createdAt": "2023-08-12T17:10:37.000Z"
	///         }
	///       },
	///       {
	///         "messageCreate": {
	///           "reactions": [ ],
	///           "urls": [ ],
	///           "text": "Salutations!",
	///           "mediaUrls": [ ],
	///           "senderId": "111111111",
	///           "id": "3333333333333333333",
	///           "createdAt": "2023-08-12T17:10:37.000Z"
	///         }
	///       },
	///       {
	///         "participantsLeave": {
	///           "userIds": [
	///             "1234",
	///             "9876"
	///           ],
	///           "createdAt": "2023-08-12T17:10:37.000Z"
	///         }
	///       },
	///       {
	///         "joinConversation": {
	///           "initiatingUserId": "111111111",
	///           "participantsSnapshot": [
	///             "222222222",
	///             "111111111"
	///           ],
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
/// use twitter_archive::structs::direct_messages_group::DmConversation;
///
/// let created_at_string = "2023-08-12T17:10:37.000Z";
/// let created_at_native_time = NaiveDateTime::parse_from_str(&created_at_string, date_time_iso_8601::FORMAT).unwrap();
/// let created_at_date_time = DateTime::<Utc>::from_naive_utc_and_offset(created_at_native_time, Utc);
///
/// let json = format!(r#"{{
///   "conversationId": "6666666666666666666",
///   "messages": [
///     {{
///       "messageCreate": {{
///         "reactions": [],
///         "urls": [],
///         "text": "Sup!?",
///         "mediaUrls": [],
///         "senderId": "222222222",
///         "id": "4444444444444444444",
///         "createdAt": "{created_at_string}"
///       }}
///     }},
///     {{
///       "messageCreate": {{
///         "reactions": [],
///         "urls": [],
///         "text": "Salutations!",
///         "mediaUrls": [],
///         "senderId": "111111111",
///         "id": "3333333333333333333",
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
/// assert_eq!(data.conversation_id, "6666666666666666666");
/// assert_eq!(data.messages.len(), 4);
///
/// // Re-serialize is equivalent to original data
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct DmConversation {
	/// ## Example JSON data
	///
	/// ```json
	/// { "conversationId": "6666666666666666666" }
	/// ```
	pub conversation_id: String,

	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "messages": [
	///     {
	///       "messageCreate": {
	///         "reactions": [],
	///         "urls": [],
	///         "text": "Sup!?",
	///         "mediaUrls": [],
	///         "senderId": "222222222",
	///         "id": "4444444444444444444",
	///         "createdAt": "2023-08-12T17:10:37.000Z"
	///       }
	///     },
	///     {
	///       "messageCreate": {
	///         "reactions": [],
	///         "urls": [],
	///         "text": "Salutations!",
	///         "mediaUrls": [],
	///         "senderId": "111111111",
	///         "id": "3333333333333333333",
	///         "createdAt": "2023-08-12T17:10:37.000Z"
	///       }
	///     },
	///     {
	///       "participantsLeave": {
	///         "userIds": [
	///           "1234",
	///           "9876"
	///         ],
	///         "createdAt": "2023-08-12T17:10:37.000Z"
	///       }
	///     },
	///     {
	///       "joinConversation": {
	///         "initiatingUserId": "111111111",
	///         "participantsSnapshot": [
	///           "222222222",
	///           "111111111"
	///         ],
	///         "createdAt": "2023-08-12T17:10:37.000Z"
	///       }
	///     }
	///   ]
	/// }
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
/// use twitter_archive::structs::direct_messages_group::Message;
///
/// let created_at_string = "2023-08-12T17:10:37.000Z";
/// let created_at_native_time = NaiveDateTime::parse_from_str(&created_at_string, date_time_iso_8601::FORMAT).unwrap();
/// let created_at_date_time = DateTime::<Utc>::from_naive_utc_and_offset(created_at_native_time, Utc);
///
/// let json = format!(r#"[
///   {{
///     "messageCreate": {{
///       "reactions": [],
///       "urls": [],
///       "text": "Sup!?",
///       "mediaUrls": [],
///       "senderId": "222222222",
///       "id": "4444444444444444444",
///       "createdAt": "{created_at_string}"
///     }}
///   }},
///   {{
///     "messageCreate": {{
///       "reactions": [],
///       "urls": [],
///       "text": "Salutations!",
///       "mediaUrls": [],
///       "senderId": "111111111",
///       "id": "3333333333333333333",
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
///     assert_eq!(message_create.reactions.len(), 0);
///     assert_eq!(message_create.urls.len(), 0);
///     assert_eq!(message_create.text, "Sup!?");
///     assert_eq!(message_create.media_urls.len(), 0);
///     assert_eq!(message_create.sender_id, "222222222");
///     assert_eq!(message_create.id, "4444444444444444444");
///     assert_eq!(message_create.created_at, created_at_date_time);
/// }
///
/// if let Message::MessageCreate(message_create) = &data.get(1).unwrap() {
///     assert_eq!(message_create.reactions.len(), 0);
///     assert_eq!(message_create.urls.len(), 0);
///     assert_eq!(message_create.text, "Salutations!");
///     assert_eq!(message_create.media_urls.len(), 0);
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
/// // Re-serialize is equivalent to original data
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[serde(rename_all = "camelCase")]
pub enum Message {
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "reactions": [],
	///   "urls": [],
	///   "text": "Salutations!",
	///   "mediaUrls": [],
	///   "senderId": "111111111",
	///   "id": "3333333333333333333",
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

/// Note, other than the lack of a `recipient_id` (`recipientId`) key, this is identical to
/// `MessageCreate` from `twitter_archive::structs::direct_messages`
///
/// ## Example
///
/// ```
/// use chrono::{DateTime, NaiveDateTime, Utc};
///
/// use twitter_archive::convert::date_time_iso_8601::FORMAT;
/// use twitter_archive::structs::direct_messages_group::MessageCreate;
///
/// let created_at_string = "2020-01-20T21:42:09.068Z";
/// let created_at_native_time = NaiveDateTime::parse_from_str(&created_at_string, FORMAT).unwrap();
/// let created_at_date_time = DateTime::<Utc>::from_naive_utc_and_offset(created_at_native_time, Utc);
///
/// let json = format!(r#"{{
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
