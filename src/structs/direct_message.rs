#!/usr/bin/env rust

//! Tweeter archives as of 2023-08-31 have private data found under;
//!
//!   twitter-<DATE>-<UID>.zip:data/direct-messages.js
//!   twitter-<DATE>-<UID>.zip:data/direct-messages-group.js
//!
//! Check following source code files for example usage;
//!
//! - ./direct_messages.rs
//! - ./direct_messages_group.rs
//!
//! ## Example `twitter-<DATE>-<UID>.zip:data/direct-messages.js` content
//!
//! ```javascript
//! ```
//!
//! ## Example `twitter-<DATE>-<UID>.zip:data/direct-messages-group.js` content
//!
//! ```javascript
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
/// use twitter_archive::structs::direct_message::ParticipantsLeave;
/// use twitter_archive::convert::date_time_iso_8601::FORMAT;
///
/// let created_at_string = "2020-01-20T21:42:09.068Z";
/// let created_at_native_time = NaiveDateTime::parse_from_str(&created_at_string, FORMAT).unwrap();
/// let created_at_date_time = DateTime::<Utc>::from_naive_utc_and_offset(created_at_native_time, Utc);
///
/// let json = format!(r#"{{
///   "userIds": [
///     "1234",
///     "9876"
///   ],
///   "createdAt": "{created_at_string}"
/// }}"#);
///
/// let data: ParticipantsLeave = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.user_ids.len(), 2);
/// assert_eq!(data.user_ids[0], "1234");
/// assert_eq!(data.user_ids[1], "9876");
/// assert_eq!(data.created_at, created_at_date_time);
///
/// // Re-serialize is equivalent to original data
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct ParticipantsLeave {
	/// List of user IDs that chose to leave group
	///
	/// URL formats;
	///
	/// - Desktop: https://twitter.com/i/user/{participants_snapshot[0]}
	///
	/// > Note; does **not** work if not logged-in.  Thanks be to Mr. Musk !-D
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "userIds": [
	///     "1234",
	///     "9876"
	///   ]
	/// }
	/// ```
	pub user_ids: Vec<String>,

	/// Date time stamp when listed participants left
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "createdAt": "2023-08-12T17:10:37.000Z" }
	/// ```
	#[serde(with = "convert::date_time_iso_8601")]
	pub created_at: DateTime<Utc>,
}

/// ## Example
///
/// ```
/// use chrono::{DateTime, NaiveDateTime, Utc};
///
/// use twitter_archive::convert::date_time_iso_8601;
///
/// use twitter_archive::structs::direct_message::JoinConversation;
///
/// let created_at_string = "2023-08-12T17:10:37.000Z";
/// let created_at_native_time = NaiveDateTime::parse_from_str(&created_at_string, date_time_iso_8601::FORMAT).unwrap();
/// let created_at_date_time = DateTime::<Utc>::from_naive_utc_and_offset(created_at_native_time, Utc);
///
/// let json = format!(r#"{{
///   "initiatingUserId": "1111111111111111111",
///   "participantsSnapshot": [
///     "2222",
///     "3333",
///     "4444"
///   ],
///   "createdAt": "{created_at_string}"
/// }}"#);
///
/// let data: JoinConversation = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.initiating_user_id, "1111111111111111111");
/// assert_eq!(data.participants_snapshot[0], "2222");
/// assert_eq!(data.participants_snapshot[1], "3333");
/// assert_eq!(data.participants_snapshot[2], "4444");
/// assert_eq!(data.created_at, created_at_date_time);
///
/// // Re-serialize is equivalent to original data
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct JoinConversation {
	/// ID of user responsible for initializing DM group
	///
	/// URL formats;
	///
	/// - Desktop: https://twitter.com/i/user/{initiating_user_id}
	///
	/// > Note; does **not** work if not logged-in.  Thanks be to Mr. Musk !-D
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "initiatingUserId": "1111111111111111111" }
	/// ```
	pub initiating_user_id: String,

	/// List of user IDs at one-point invited, if not involved, with DM group
	///
	/// URL formats;
	///
	/// - Desktop: `https://twitter.com/i/user/{participants_snapshot[0]}`
	///
	/// > Note; does **not** work if not logged-in.  Thanks be to Mr. Musk !-D
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "participantsSnapshot": [
	///     "2222",
	///     "3333",
	///     "4444"
	///   ]
	/// }
	/// ```
	pub participants_snapshot: Vec<String>,

	/// Date time stamp when DM group was created
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "createdAt": "2020-01-20T21:42:09.068Z" }
	/// ```
	#[serde(with = "convert::date_time_iso_8601")]
	pub created_at: DateTime<Utc>,
}

/// ## Example
///
/// ```
/// use chrono::{DateTime, NaiveDateTime, Utc};
///
/// use twitter_archive::convert::date_time_iso_8601::FORMAT;
///
/// use twitter_archive::structs::direct_message::MessageCreateReaction;
///
/// let created_at_string = "2020-01-20T21:42:09.068Z";
/// let created_at_native_time = NaiveDateTime::parse_from_str(&created_at_string, FORMAT).unwrap();
/// let created_at_date_time = DateTime::<Utc>::from_naive_utc_and_offset(created_at_native_time, Utc);
///
/// let json = format!(r#"{{
///   "senderId": "222222222",
///   "reactionKey": "excited",
///   "eventId": "1020304050607080901",
///   "createdAt": "{created_at_string}"
/// }}"#);
///
/// let data: MessageCreateReaction = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.sender_id, "222222222");
/// assert_eq!(data.reaction_key, "excited");
/// assert_eq!(data.event_id, "1020304050607080901");
/// assert_eq!(data.created_at, created_at_date_time);
///
/// // Re-serialize is equivalent to original data
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct MessageCreateReaction {
	/// User ID of who set the reaction
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "senderId": "222222222" }
	/// ```
	pub sender_id: String,

	/// Word representation of emoji displayed to clients
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "reactionKey": "excited" }
	/// ```
	pub reaction_key: String,

	/// Possibly unique ID across all conversations and messages
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "eventId": "1020304050607080901" }
	/// ```
	pub event_id: String,

	/// When reaction was published
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "createdAt": "2020-01-20T21:42:09.068Z" }
	/// ```
	#[serde(with = "convert::date_time_iso_8601")]
	pub created_at: DateTime<Utc>,
}

/// ## Example
///
/// ```
/// use twitter_archive::structs::direct_message::MessageCreateUrl;
///
/// let json = r#"{
///   "url": "https://t.co/Yot7Ijm9vG",
///   "expanded": "https://github.com/S0AndS0/",
///   "display": "github.com/S0AndS0/"
/// }"#;
///
/// let data: MessageCreateUrl = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.url, "https://t.co/Yot7Ijm9vG");
/// assert_eq!(data.expanded, "https://github.com/S0AndS0/");
/// assert_eq!(data.display, "github.com/S0AndS0/");
///
/// // Re-serialize is equivalent to original data
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
pub struct MessageCreateUrl {
	/// Twitter shortened, and tracking, URL
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "url": "https://t.co/Yot7Ijm9vG" }
	/// ```
	pub url: String,

	/// The _real_ URL
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "expanded": "https://github.com/S0AndS0/" }
	/// ```
	pub expanded: String,

	/// What clients are able to view of URL within text
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "display": "github.com/S0AndS0/" }
	/// ```
	pub display: String,
}
