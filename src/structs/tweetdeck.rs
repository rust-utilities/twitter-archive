#!/usr/bin/env rust

//! Tweeter archives as of 2023-08-31 have public tweetdeck found under;
//!
//!   twitter-<DATE>-<UID>.zip:data/tweetdeck.js
//!
//! ## Example file reader
//!
//! ```no_build
//! use std::io::Read;
//! use std::{fs, path};
//! use zip::read::ZipArchive;
//!
//! use twitter_archive::structs::tweetdeck;
//!
//! fn main() {
//!     let input_file = "~/Downloads/twitter-archive.zip";
//!
//!     let file_descriptor = fs::File::open(input_file).expect("Unable to read --input-file");
//!     let mut zip_archive = ZipArchive::new(file_descriptor).unwrap();
//!     let mut zip_file = zip_archive.by_name("data/tweetdeck.js").unwrap();
//!     let mut buff = String::new();
//!     zip_file.read_to_string(&mut buff).unwrap();
//!
//!     let json = buff.replacen("window.YTD.tweetdeck.part0 = ", "", 1);
//!     let data: Vec<tweetdeck::DeckObject> = serde_json::from_str(&json).expect("Unable to parse");
//!
//!     for (index_deck, object_deck) in data.iter().enumerate() {
//!         /* Do stuff with each Deck */
//!         println!("Deck index: {index_deck}");
//!         for (index_column, column) in object_deck.deck.columns.iter().enumerate() {
//!             /* Do stuff with each Deck's columns */
//!             println!("  Column index: {index_deck}");
//!             if let Some(title) = &column.title {
//!                 println!("  Title: {title}");
//!             }
//!
//!             if let Some(query) = &column.query {
//!                 println!("  Query: {query}");
//!             }
//!
//!             println!("  Path name: {}", column.pathname);
//!         }
//!     }
//! }
//! ```
//!
//! ## Example `twitter-<DATE>-<UID>.zip:data/tweetdeck.js` content
//!
//! ```javascript
//! window.YTD.tweetdeck.part0 = [
//!   {
//!     "deck" : {
//!       "title" : "Personal",
//!       "columns" : [
//!         {
//!           "pathname" : "/home",
//!           "title" : "Home"
//!         },
//!         {
//!           "pathname" : "/notifications",
//!           "title" : "🔔-Notifications"
//!         },
//!         {
//!           "pathname" : "/S0_And_S0",
//!           "query" : "from:S0_And_S0"
//!         }
//!       ]
//!     }
//!   },
//!   {
//!     "deck" : {
//!       "title" : "Timelines",
//!       "columns" : [
//!         {
//!           "pathname" : "/S0_And_S0"
//!         },
//!         {
//!           "pathname" : "/S0_And_S0/timelines/1161839635128967168"
//!         },
//!         {
//!           "pathname" : "/S0_And_S0/timelines/1161837773554212864?urtUrl="
//!         }
//!       ]
//!     }
//!   }
//! ]
//! ```

use derive_more::Display;
use serde::{Deserialize, Serialize};

/// ## Example
///
/// ```
/// use twitter_archive::structs::tweetdeck::DeckObject;
///
/// let json = r#"{
///   "deck": {
///     "title": "Personal",
///     "columns": [
///       {
///         "pathname": "/home",
///         "title": "Home"
///       },
///       {
///         "pathname": "/notifications",
///         "title": "🔔-Notifications"
///       },
///       {
///         "pathname": "/S0_And_S0",
///         "query": "from:S0_And_S0"
///       }
///     ]
///   }
/// }"#;
///
/// let data: DeckObject = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.deck.title, "Personal");
/// assert_eq!(data.deck.columns.len(), 3);
///
/// // Re-serialize is equivalent to original data
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
pub struct DeckObject {
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "deck": {
	///     "title": "Personal",
	///     "columns": [
	///       {
	///         "pathname": "/home",
	///         "title": "Home"
	///       },
	///       {
	///         "pathname": "/notifications",
	///         "title": "🔔-Notifications"
	///       },
	///       {
	///         "pathname": "/S0_And_S0",
	///         "query": "from:S0_And_S0"
	///       }
	///     ]
	///   }
	/// }
	/// ```
	pub deck: Deck,
}

/// ## Example
///
/// ```
/// use twitter_archive::structs::tweetdeck::Deck;
///
/// let json = r#"{
///   "title": "Personal",
///   "columns": [
///     {
///       "pathname": "/home",
///       "title": "Home"
///     },
///     {
///       "pathname": "/notifications",
///       "title": "🔔-Notifications"
///     },
///     {
///       "pathname": "/S0_And_S0",
///       "query": "from:S0_And_S0"
///     }
///   ]
/// }"#;
///
/// let data: Deck = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.title, "Personal");
/// assert_eq!(data.columns.len(), 3);
///
/// // Re-serialize is equivalent to original data
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
pub struct Deck {
	/// ## Example JSON data
	///
	/// ```json
	/// { "title": "Personal" }
	/// ```
	pub title: String,

	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "columns": [
	///     {
	///       "pathname": "/home",
	///       "title": "Home"
	///     },
	///     {
	///       "pathname": "/notifications",
	///       "title": "🔔-Notifications"
	///     },
	///     {
	///       "pathname": "/S0_And_S0",
	///       "query": "from:S0_And_S0"
	///     }
	///   ]
	/// }
	/// ```
	pub columns: Vec<DeckColumn>,
}

/// ## Example
///
/// ```
/// use twitter_archive::structs::tweetdeck::DeckColumn;
///
/// let json = r#"{
///   "pathname": "/home",
///   "title": "Home"
/// }"#;
///
/// let data: DeckColumn = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.pathname, "/home".to_string());
/// assert_eq!(data.title, Some("Home".to_string()));
/// assert_eq!(data.query, None);
///
/// // Re-serialize is equivalent to original data
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
pub struct DeckColumn {
	/// URL format
	///
	/// - `https://twitter.com/S0_And_S0/status{pathname}`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "pathname": "/home" }
	/// ```
	pub pathname: String,

	/// ## Example JSON data
	///
	/// ```json
	/// { "title": "Home" }
	/// ```
	#[serde(skip_serializing_if = "Option::is_none")]
	pub title: Option<String>,

	/// ## Example JSON data
	///
	/// ```json
	/// { "query": "from:S0_And_S0" }
	/// ```
	#[serde(skip_serializing_if = "Option::is_none")]
	pub query: Option<String>,
}
