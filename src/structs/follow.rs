#!/usr/bin/env rust

//! Tweeter archives as of 2023-08-31 have private data found under;
//!
//!   twitter-<DATE>-<UID>.zip:data/following.js
//!   twitter-<DATE>-<UID>.zip:data/follower.js
//!
//! Check following source code files for example usage;
//!
//! - ./follower.rs
//! - ./following.rs
//!
//! ## Example `twitter-<DATE>-<UID>.zip:data/following.js` content
//!
//! ```javascript
//! window.YTD.following.part0 = [
//!   {
//!     "following" : {
//!       "accountId" : "1111111111111111111",
//!       "userLink" : "https://twitter.com/intent/user?user_id=1111111111111111111"
//!     }
//!   }
//! ]
//! ```
//!
//! ## Example `twitter-<DATE>-<UID>.zip:data/follower.js` content
//!
//! ```javascript
//! window.YTD.follower.part0 = [
//!   {
//!     "follower" : {
//!       "accountId" : "2222222222222222222",
//!       "userLink" : "https://twitter.com/intent/user?user_id=2222222222222222222"
//!     }
//!   }
//! ]
//! ```

use derive_more::Display;
use serde::{Deserialize, Serialize};

/// ## Example
///
/// ```
/// use twitter_archive::structs::follow::Follow;
///
/// let json = r#"{
///   "accountId": "2222222222222222222",
///   "userLink": "https://twitter.com/intent/user?user_id=2222222222222222222"
/// }"#;
///
/// let data: Follow = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.account_id, "2222222222222222222");
/// assert_eq!(data.user_link, "https://twitter.com/intent/user?user_id=2222222222222222222");
///
/// // Re-serialize is equivalent to original data
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct Follow {
	/// URL formats;
	///
	/// - Desktop: https://twitter.com/i/user/{account_id}
	///
	/// > Note; does **not** work if not logged-in.  Thanks be to Mr. Musk !-D
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "accountId": "2222222222222222222" }
	/// ```
	pub account_id: String,

	/// Alternate way of directly linking to account by ID, with added side effect of prompting
	/// client to follow profile regardless of following status
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "userLink": "https://twitter.com/intent/user?user_id=2222222222222222222" }
	/// ```
	pub user_link: String,
}
