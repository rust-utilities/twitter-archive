#!/usr/bin/env rust

//! Tweeter archives as of 2023-08-31 have private data found under;
//!
//!   twitter-<DATE>-<UID>.zip:data/ad-engagements.js
//!   twitter-<DATE>-<UID>.zip:data/ad-impressions.js
//!
//! Check following source code files for example usage;
//!
//! - ./direct_messages.rs
//! - ./direct_messages_group.rs
//!
//! ## Example `twitter-<DATE>-<UID>.zip:data/ad-engagements.js` content
//!
//! ```javascript
//! ```
//!
//! ## Example `twitter-<DATE>-<UID>.zip:data/ad-impressions.js` content
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
/// use twitter_archive::convert::date_year_month_day_hour_minute_second::FORMAT;
///
/// use twitter_archive::structs::ad::Impression;
///
/// let impression_time_string = "2023-06-05 17:00:52";
/// let impression_time_native_time = NaiveDateTime::parse_from_str(&impression_time_string, FORMAT).unwrap();
/// let impression_time_date_time = DateTime::<Utc>::from_naive_utc_and_offset(impression_time_native_time, Utc);
///
/// let json = format!(r#"{{
///   "deviceInfo": {{
///     "osType": "Desktop"
///   }},
///   "displayLocation": "TweetConversation",
///   "promotedTweetInfo": {{
///     "tweetId": "1111111111111111111",
///     "tweetText": "Click bate",
///     "urls": [],
///     "mediaUrls": [
///       "https://t.co/AHAAAAAAAA"
///     ]
///   }},
///   "advertiserInfo": {{
///     "advertiserName": "EXAMPLE",
///     "screenName": "@EXAMPLE"
///   }},
///   "matchedTargetingCriteria": [
///     {{
///       "targetingType": "Follower look-alikes",
///       "targetingValue": "@EXAMPLE"
///     }}
///   ],
///   "impressionTime": "{impression_time_string}"
/// }}"#);
///
/// let data: Impression = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.device_info.os_type, "Desktop");
///
/// assert_eq!(data.display_location, "TweetConversation");
///
/// if let Some(promoted_tweet_info) = &data.promoted_tweet_info {
///     assert_eq!(promoted_tweet_info.tweet_id, "1111111111111111111");
///     assert_eq!(promoted_tweet_info.tweet_text, "Click bate");
///     assert_eq!(promoted_tweet_info.urls.len(), 0);
///     assert_eq!(promoted_tweet_info.media_urls.len(), 1);
///     assert_eq!(promoted_tweet_info.media_urls[0], "https://t.co/AHAAAAAAAA");
/// }
///
/// if let Some(advertiser_name) = &data.advertiser_info.advertiser_name {
///     assert_eq!(advertiser_name, "EXAMPLE");
/// }
/// if let Some(screen_name) = &data.advertiser_info.screen_name {
///     assert_eq!(screen_name, "@EXAMPLE");
/// }
///
/// if let Some(matched_targeting_criteria) = &data.matched_targeting_criteria {
///     assert_eq!(matched_targeting_criteria.len(), 1);
///     assert_eq!(matched_targeting_criteria[0].targeting_type, "Follower look-alikes");
///     if let Some(targeting_value) = &matched_targeting_criteria[0].targeting_value {
///         assert_eq!(targeting_value, "@EXAMPLE");
///     }
/// }
///
/// assert_eq!(data.impression_time, impression_time_date_time);
///
/// // Re-serialize is equivalent to original data without pretty printing
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct Impression {
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "deviceInfo": {
	///     "osType": "Desktop"
	///   }
	/// }
	/// ```
	pub device_info: DeviceInfo,

	/// ## Example JSON data
	///
	/// ```json
	/// { "displayLocation": "TweetConversation" }
	/// ```
	pub display_location: String,

	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "promotedTweetInfo": {
	///     "tweetId": "1111111111111111111",
	///     "tweetText": "Click bate",
	///     "urls": [],
	///     "mediaUrls": [
	///       "https://t.co/AHAAAAAAAA"
	///     ]
	///   }
	/// }
	/// ```
	#[serde(skip_serializing_if = "Option::is_none")]
	pub promoted_tweet_info: Option<PromotedTweetInfo>,

	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "advertiserInfo": {
	///     "advertiserName": "EXAMPLE",
	///     "screenName": "@EXAMPLE"
	///   }
	/// }
	/// ```
	pub advertiser_info: AdvertiserInfo,

	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "matchedTargetingCriteria": [
	///     {
	///       "targetingType": "Follower look-alikes",
	///       "targetingValue": "@EXAMPLE"
	///     }
	///   ]
	/// }
	/// ```
	#[serde(skip_serializing_if = "Option::is_none")]
	pub matched_targeting_criteria: Option<Vec<TargetingCriteria>>,

	/// ## Example JSON data
	///
	/// ```json
	/// { "impressionTime": "2023-06-05 17:00:52" }
	/// ```
	#[serde(with = "convert::date_year_month_day_hour_minute_second")]
	pub impression_time: DateTime<Utc>,
}

/// ## Example
///
/// ```
/// use twitter_archive::structs::ad::DeviceInfo;
///
/// let json = format!(r#"{{
///   "osType": "Desktop"
/// }}"#);
///
/// let data: DeviceInfo = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.os_type, "Desktop");
///
/// // Re-serialize is equivalent to original data without pretty printing
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct DeviceInfo {
	/// ## Example JSON data
	///
	/// ```json
	/// { "osType": "Desktop" }
	/// ```
	pub os_type: String,
}

/// ## Example
///
/// ```
/// use twitter_archive::structs::ad::PromotedTweetInfo;
///
/// let json = format!(r#"{{
///   "tweetId": "1111111111111111111",
///   "tweetText": "Click bate",
///   "urls": [],
///   "mediaUrls": [
///     "https://t.co/AHAAAAAAAA"
///   ]
/// }}"#);
///
/// let data: PromotedTweetInfo = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.tweet_id, "1111111111111111111");
/// assert_eq!(data.tweet_text, "Click bate");
/// assert_eq!(data.urls.len(), 0);
/// assert_eq!(data.media_urls.len(), 1);
/// assert_eq!(data.media_urls[0], "https://t.co/AHAAAAAAAA");
///
/// // Re-serialize is equivalent to original data without pretty printing
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct PromotedTweetInfo {
	/// URL formats;
	///
	/// - Desktop: `https://twitter.com/i/web/status/{tweet_id}`
	/// - Mobile: `https://mobile.twitter.com/i/web/status/{tweet_id}`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "tweetId": "1111111111111111111" }
	/// ```
	pub tweet_id: String,

	/// ## Example JSON data
	///
	/// ```json
	/// { "tweetText": "Click bate" }
	/// ```
	pub tweet_text: String,

	/// ## Example JSON data
	///
	/// ```json
	/// { "urls": [] }
	/// ```
	pub urls: Vec<String>,

	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "mediaUrls": [
	///     "https://t.co/AHAAAAAAAA"
	///   ]
	/// }
	/// ```
	pub media_urls: Vec<String>,
}

/// ## Example
///
/// ```
/// use twitter_archive::structs::ad::AdvertiserInfo;
///
/// let json = format!(r#"{{
///   "advertiserName": "EXAMPLE",
///   "screenName": "@EXAMPLE"
/// }}"#);
///
/// let data: AdvertiserInfo = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// if let Some(advertiser_name) = &data.advertiser_name {
///     assert_eq!(advertiser_name, "EXAMPLE");
/// }
/// if let Some(screen_name) = &data.screen_name {
///     assert_eq!(screen_name, "@EXAMPLE");
/// }
///
/// // Re-serialize is equivalent to original data without pretty printing
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct AdvertiserInfo {
	/// ## Example JSON data
	///
	/// ```json
	/// { "advertiserName": "EXAMPLE" }
	/// ```
	#[serde(skip_serializing_if = "Option::is_none")]
	pub advertiser_name: Option<String>,

	/// ## Example JSON data
	///
	/// ```json
	/// { "screenName": "@EXAMPLE" }
	/// ```
	#[serde(skip_serializing_if = "Option::is_none")]
	pub screen_name: Option<String>,
}

/// ## Example
///
/// ```
/// use twitter_archive::structs::ad::TargetingCriteria;
///
/// let json = format!(r#"{{
///   "targetingType": "Follower look-alikes",
///   "targetingValue": "@EXAMPLE"
/// }}"#);
///
/// let data: TargetingCriteria = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.targeting_type, "Follower look-alikes");
/// if let Some(targeting_value) = &data.targeting_value {
///     assert_eq!(targeting_value, "@EXAMPLE");
/// }
///
/// // Re-serialize is equivalent to original data without pretty printing
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct TargetingCriteria {
	/// ## Example JSON data
	///
	/// ```json
	/// { "targetingType": "Follower look-alikes" }
	/// ```
	///
	/// TODO: Maybe convert to `enum` in future major version realise
	pub targeting_type: String,

	/// ## Example JSON data
	///
	/// ```json
	/// { "targetingValue": "@EXAMPLE" }
	/// ```
	#[serde(skip_serializing_if = "Option::is_none")]
	pub targeting_value: Option<String>,
}
