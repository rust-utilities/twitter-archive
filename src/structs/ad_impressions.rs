#!/usr/bin/env rust

//! Tweeter archives as of 2023-08-31 have private blocking data found under;
//!
//!   twitter-<DATE>-<UID>.zip:data/ad-impressions.js
//!
//! ## Example file reader for `twitter-<DATE>-<UID>.zip:data/ad-impressions.js`
//!
//! ```no_build
//! use std::io::Read;
//! use std::{fs, path};
//! use zip::read::ZipArchive;
//!
//! use twitter_archive::structs::ad_impressions;
//!
//! fn main() {
//!     let input_file = "~/Downloads/twitter-archive.zip";
//!
//!     let file_descriptor = fs::File::open(input_file).expect("Unable to read --input-file");
//!     let mut zip_archive = ZipArchive::new(file_descriptor).unwrap();
//!     let mut zip_file = zip_archive.by_name("data/ad-impressions.js").unwrap();
//!     let mut buff = String::new();
//!     zip_file.read_to_string(&mut buff).unwrap();
//!
//!     let json = buff.replacen("window.YTD.ad_impressions.part0 = ", "", 1);
//!     let data: Vec<ad_impressions::AdObject> = serde_json::from_str(&json).expect("Unable to parse");
//!
//!     for (index_ad, object_ad) in data.iter().enumerate() {
//!         /* Do stuff with each advertisement */
//!         println!("Advertisement index: {index_ad}");
//!         let impressions = &object_ad.ad.ads_user_data.ad_impressions.impressions;
//!         for (index_impression, object_impression) in impressions.iter().enumerate() {
//!             if let Some(promoted_tweet_info) = &object_impression.promoted_tweet_info {
//!                 println!("Promoted Tweet ID: {}", promoted_tweet_info.tweet_id);
//!                 println!("Promoted Tweet text: {}", promoted_tweet_info.tweet_text);
//!             }
//!             println!("Impression time: {}", object_impression.impression_time);
//!         }
//!     }
//! }
//! ```
//!
//! ## Example `twitter-<DATE>-<UID>.zip:data/ad-impressions.js` content
//!
//! ```javascript
//! window.YTD.ad_impressions.part0 = [
//!   {
//!     "ad" : {
//!       "adsUserData" : {
//!         "adImpressions" : {
//!           "impressions" : [
//!             {
//!               "deviceInfo" : {
//!                 "osType" : "Desktop"
//!               },
//!               "displayLocation" : "TweetConversation",
//!               "promotedTweetInfo" : {
//!                 "tweetId" : "1111111111111111111",
//!                 "tweetText" : "Click bate",
//!                 "urls" : [ ],
//!                 "mediaUrls" : [
//!                   "https://t.co/AHAAAAAAAA"
//!                 ]
//!               },
//!               "advertiserInfo" : {
//!                 "advertiserName" : "EXAMPLE",
//!                 "screenName" : "@EXAMPLE"
//!               },
//!               "matchedTargetingCriteria" : [
//!                 {
//!                   "targetingType" : "Follower look-alikes",
//!                   "targetingValue" : "@EXAMPLE"
//!                 }
//!               ],
//!               "impressionTime" : "2023-06-05 17:00:52"
//!             }
//!           ]
//!         }
//!       }
//!     }
//!   }
//! ]
//! ```

use derive_more::Display;
use serde::{Deserialize, Serialize};

use crate::structs::ad;

/// ## Example
///
/// ```
/// use chrono::{DateTime, NaiveDateTime, Utc};
///
/// use twitter_archive::convert::date_year_month_day_hour_minute_second::FORMAT;
///
/// use twitter_archive::structs::ad_impressions::AdObject;
///
/// let impression_time_string = "2023-06-05 17:00:52";
/// let impression_time_native_time = NaiveDateTime::parse_from_str(&impression_time_string, FORMAT).unwrap();
/// let impression_time_date_time = DateTime::<Utc>::from_naive_utc_and_offset(impression_time_native_time, Utc);
///
/// let json = format!(r#"{{
///   "ad": {{
///     "adsUserData": {{
///       "adImpressions": {{
///         "impressions": [
///           {{
///             "deviceInfo": {{
///               "osType": "Desktop"
///             }},
///             "displayLocation": "TweetConversation",
///             "promotedTweetInfo": {{
///               "tweetId": "1111111111111111111",
///               "tweetText": "Click bate",
///               "urls": [],
///               "mediaUrls": [
///                 "https://t.co/AHAAAAAAAA"
///               ]
///             }},
///             "advertiserInfo": {{
///               "advertiserName": "EXAMPLE",
///               "screenName": "@EXAMPLE"
///             }},
///             "matchedTargetingCriteria": [
///               {{
///                 "targetingType": "Follower look-alikes",
///                 "targetingValue": "@EXAMPLE"
///               }}
///             ],
///             "impressionTime": "{impression_time_string}"
///           }}
///         ]
///       }}
///     }}
///   }}
/// }}"#);
///
/// let data: AdObject = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.ad.ads_user_data.ad_impressions.impressions.len(), 1);
/// assert_eq!(data.ad.ads_user_data.ad_impressions.impressions[0].device_info.os_type, "Desktop");
/// assert_eq!(data.ad.ads_user_data.ad_impressions.impressions[0].display_location, "TweetConversation");
///
/// if let Some(promoted_tweet_info) = &data.ad.ads_user_data.ad_impressions.impressions[0].promoted_tweet_info {
///     assert_eq!(promoted_tweet_info.tweet_id, "1111111111111111111");
///     assert_eq!(promoted_tweet_info.tweet_text, "Click bate");
///     assert_eq!(promoted_tweet_info.urls.len(), 0);
///     assert_eq!(promoted_tweet_info.media_urls.len(), 1);
///     assert_eq!(promoted_tweet_info.media_urls[0], "https://t.co/AHAAAAAAAA");
/// }
///
/// if let Some(advertiser_name) = &data.ad.ads_user_data.ad_impressions.impressions[0].advertiser_info.advertiser_name {
///     assert_eq!(advertiser_name, "EXAMPLE");
/// }
/// if let Some(screen_name) = &data.ad.ads_user_data.ad_impressions.impressions[0].advertiser_info.screen_name {
///     assert_eq!(screen_name, "@EXAMPLE");
/// }
///
/// if let Some(matched_targeting_criteria) = &data.ad.ads_user_data.ad_impressions.impressions[0].matched_targeting_criteria {
///     assert_eq!(matched_targeting_criteria.len(), 1);
///     assert_eq!(matched_targeting_criteria[0].targeting_type, "Follower look-alikes");
///     if let Some(targeting_value) = &matched_targeting_criteria[0].targeting_value {
///         assert_eq!(targeting_value, "@EXAMPLE");
///     }
/// }
///
/// assert_eq!(data.ad.ads_user_data.ad_impressions.impressions[0].impression_time, impression_time_date_time);
///
/// // Re-serialize is equivalent to original data without pretty printing
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct AdObject {
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "ad": {
	///     "adsUserData": {
	///       "adImpressions": {
	///         "impressions": [
	///           {
	///             "deviceInfo": {
	///               "osType": "Desktop"
	///             },
	///             "displayLocation": "TweetConversation",
	///             "promotedTweetInfo": {
	///               "tweetId": "1111111111111111111",
	///               "tweetText": "Click bate",
	///               "urls": [],
	///               "mediaUrls": [
	///                 "https://t.co/AHAAAAAAAA"
	///               ]
	///             },
	///             "advertiserInfo": {
	///               "advertiserName": "EXAMPLE",
	///               "screenName": "@EXAMPLE"
	///             },
	///             "matchedTargetingCriteria": [
	///               {
	///                 "targetingType": "Follower look-alikes",
	///                 "targetingValue": "@EXAMPLE"
	///               }
	///             ],
	///             "impressionTime": "2023-06-05 17:00:52"
	///           }
	///         ]
	///       }
	///     }
	///   }
	/// }
	/// ```
	pub ad: Ad,
}

/// ## Example
///
/// ```
/// use chrono::{DateTime, NaiveDateTime, Utc};
///
/// use twitter_archive::convert::date_year_month_day_hour_minute_second::FORMAT;
///
/// use twitter_archive::structs::ad_impressions::Ad;
///
/// let impression_time_string = "2023-06-05 17:00:52";
/// let impression_time_native_time = NaiveDateTime::parse_from_str(&impression_time_string, FORMAT).unwrap();
/// let impression_time_date_time = DateTime::<Utc>::from_naive_utc_and_offset(impression_time_native_time, Utc);
///
/// let json = format!(r#"{{
///   "adsUserData": {{
///     "adImpressions": {{
///       "impressions": [
///         {{
///           "deviceInfo": {{
///             "osType": "Desktop"
///           }},
///           "displayLocation": "TweetConversation",
///           "promotedTweetInfo": {{
///             "tweetId": "1111111111111111111",
///             "tweetText": "Click bate",
///             "urls": [],
///             "mediaUrls": [
///               "https://t.co/AHAAAAAAAA"
///             ]
///           }},
///           "advertiserInfo": {{
///             "advertiserName": "EXAMPLE",
///             "screenName": "@EXAMPLE"
///           }},
///           "matchedTargetingCriteria": [
///             {{
///               "targetingType": "Follower look-alikes",
///               "targetingValue": "@EXAMPLE"
///             }}
///           ],
///           "impressionTime": "{impression_time_string}"
///         }}
///       ]
///     }}
///   }}
/// }}"#);
///
/// let data: Ad = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.ads_user_data.ad_impressions.impressions.len(), 1);
/// assert_eq!(data.ads_user_data.ad_impressions.impressions[0].device_info.os_type, "Desktop");
/// assert_eq!(data.ads_user_data.ad_impressions.impressions[0].display_location, "TweetConversation");
///
/// if let Some(promoted_tweet_info) = &data.ads_user_data.ad_impressions.impressions[0].promoted_tweet_info {
///     assert_eq!(promoted_tweet_info.tweet_id, "1111111111111111111");
///     assert_eq!(promoted_tweet_info.tweet_text, "Click bate");
///     assert_eq!(promoted_tweet_info.urls.len(), 0);
///     assert_eq!(promoted_tweet_info.media_urls.len(), 1);
///     assert_eq!(promoted_tweet_info.media_urls[0], "https://t.co/AHAAAAAAAA");
/// }
///
/// if let Some(advertiser_name) = &data.ads_user_data.ad_impressions.impressions[0].advertiser_info.advertiser_name {
///     assert_eq!(advertiser_name, "EXAMPLE");
/// }
/// if let Some(screen_name) = &data.ads_user_data.ad_impressions.impressions[0].advertiser_info.screen_name {
///     assert_eq!(screen_name, "@EXAMPLE");
/// }
///
/// if let Some(matched_targeting_criteria) = &data.ads_user_data.ad_impressions.impressions[0].matched_targeting_criteria {
///     assert_eq!(matched_targeting_criteria.len(), 1);
///     assert_eq!(matched_targeting_criteria[0].targeting_type, "Follower look-alikes");
///     if let Some(targeting_value) = &matched_targeting_criteria[0].targeting_value {
///         assert_eq!(targeting_value, "@EXAMPLE");
///     }
/// }
///
/// assert_eq!(data.ads_user_data.ad_impressions.impressions[0].impression_time, impression_time_date_time);
///
/// // Re-serialize is equivalent to original data without pretty printing
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct Ad {
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "adsUserData": {
	///     "adImpressions": {
	///       "impressions": [
	///         {
	///           "deviceInfo": {
	///             "osType": "Desktop"
	///           },
	///           "displayLocation": "TweetConversation",
	///           "promotedTweetInfo": {
	///             "tweetId": "1111111111111111111",
	///             "tweetText": "Click bate",
	///             "urls": [],
	///             "mediaUrls": [
	///               "https://t.co/AHAAAAAAAA"
	///             ]
	///           },
	///           "advertiserInfo": {
	///             "advertiserName": "EXAMPLE",
	///             "screenName": "@EXAMPLE"
	///           },
	///           "matchedTargetingCriteria": [
	///             {
	///               "targetingType": "Follower look-alikes",
	///               "targetingValue": "@EXAMPLE"
	///             }
	///           ],
	///           "impressionTime": "2023-06-05 17:00:52"
	///         }
	///       ]
	///     }
	///   }
	/// }
	/// ```
	pub ads_user_data: AdsUserData,
}

/// ## Example
///
/// ```
/// use chrono::{DateTime, NaiveDateTime, Utc};
///
/// use twitter_archive::convert::date_year_month_day_hour_minute_second::FORMAT;
///
/// use twitter_archive::structs::ad_impressions::AdsUserData;
///
/// let impression_time_string = "2023-06-05 17:00:52";
/// let impression_time_native_time = NaiveDateTime::parse_from_str(&impression_time_string, FORMAT).unwrap();
/// let impression_time_date_time = DateTime::<Utc>::from_naive_utc_and_offset(impression_time_native_time, Utc);
///
/// let json = format!(r#"{{
///   "adImpressions": {{
///     "impressions": [
///       {{
///         "deviceInfo": {{
///           "osType": "Desktop"
///         }},
///         "displayLocation": "TweetConversation",
///         "promotedTweetInfo": {{
///           "tweetId": "1111111111111111111",
///           "tweetText": "Click bate",
///           "urls": [],
///           "mediaUrls": [
///             "https://t.co/AHAAAAAAAA"
///           ]
///         }},
///         "advertiserInfo": {{
///           "advertiserName": "EXAMPLE",
///           "screenName": "@EXAMPLE"
///         }},
///         "matchedTargetingCriteria": [
///           {{
///             "targetingType": "Follower look-alikes",
///             "targetingValue": "@EXAMPLE"
///           }}
///         ],
///         "impressionTime": "{impression_time_string}"
///       }}
///     ]
///   }}
/// }}"#);
///
/// let data: AdsUserData = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.ad_impressions.impressions.len(), 1);
///
/// assert_eq!(data.ad_impressions.impressions[0].device_info.os_type, "Desktop");
///
/// assert_eq!(data.ad_impressions.impressions[0].display_location, "TweetConversation");
///
/// if let Some(promoted_tweet_info) = &data.ad_impressions.impressions[0].promoted_tweet_info {
///     assert_eq!(promoted_tweet_info.tweet_id, "1111111111111111111");
///     assert_eq!(promoted_tweet_info.tweet_text, "Click bate");
///     assert_eq!(promoted_tweet_info.urls.len(), 0);
///     assert_eq!(promoted_tweet_info.media_urls.len(), 1);
///     assert_eq!(promoted_tweet_info.media_urls[0], "https://t.co/AHAAAAAAAA");
/// }
///
/// if let Some(advertiser_name) = &data.ad_impressions.impressions[0].advertiser_info.advertiser_name {
///     assert_eq!(advertiser_name, "EXAMPLE");
/// }
/// if let Some(screen_name) = &data.ad_impressions.impressions[0].advertiser_info.screen_name {
///     assert_eq!(screen_name, "@EXAMPLE");
/// }
///
/// if let Some(matched_targeting_criteria) = &data.ad_impressions.impressions[0].matched_targeting_criteria {
///     assert_eq!(matched_targeting_criteria.len(), 1);
///     assert_eq!(matched_targeting_criteria[0].targeting_type, "Follower look-alikes");
///     if let Some(targeting_value) = &matched_targeting_criteria[0].targeting_value {
///         assert_eq!(targeting_value, "@EXAMPLE");
///     }
/// }
///
/// assert_eq!(data.ad_impressions.impressions[0].impression_time, impression_time_date_time);
///
/// // Re-serialize is equivalent to original data without pretty printing
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct AdsUserData {
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "adImpressions": {
	///     "impressions": [
	///       {
	///         "deviceInfo": {
	///           "osType": "Desktop"
	///         },
	///         "displayLocation": "TweetConversation",
	///         "promotedTweetInfo": {
	///           "tweetId": "1111111111111111111",
	///           "tweetText": "Click bate",
	///           "urls": [],
	///           "mediaUrls": [
	///             "https://t.co/AHAAAAAAAA"
	///           ]
	///         },
	///         "advertiserInfo": {
	///           "advertiserName": "EXAMPLE",
	///           "screenName": "@EXAMPLE"
	///         },
	///         "matchedTargetingCriteria": [
	///           {
	///             "targetingType": "Follower look-alikes",
	///             "targetingValue": "@EXAMPLE"
	///           }
	///         ],
	///         "impressionTime": "2023-06-05 17:00:52"
	///       }
	///     ]
	///   }
	/// }
	/// ```
	pub ad_impressions: AdImpressions,
}

/// ## Example
///
/// ```
/// use chrono::{DateTime, NaiveDateTime, Utc};
///
/// use twitter_archive::convert::date_year_month_day_hour_minute_second::FORMAT;
///
/// use twitter_archive::structs::ad_impressions::AdImpressions;
///
/// let impression_time_string = "2023-06-05 17:00:52";
/// let impression_time_native_time = NaiveDateTime::parse_from_str(&impression_time_string, FORMAT).unwrap();
/// let impression_time_date_time = DateTime::<Utc>::from_naive_utc_and_offset(impression_time_native_time, Utc);
///
/// let json = format!(r#"{{
///   "impressions": [
///     {{
///       "deviceInfo": {{
///         "osType": "Desktop"
///       }},
///       "displayLocation": "TweetConversation",
///       "promotedTweetInfo": {{
///         "tweetId": "1111111111111111111",
///         "tweetText": "Click bate",
///         "urls": [],
///         "mediaUrls": [
///           "https://t.co/AHAAAAAAAA"
///         ]
///       }},
///       "advertiserInfo": {{
///         "advertiserName": "EXAMPLE",
///         "screenName": "@EXAMPLE"
///       }},
///       "matchedTargetingCriteria": [
///         {{
///           "targetingType": "Follower look-alikes",
///           "targetingValue": "@EXAMPLE"
///         }}
///       ],
///       "impressionTime": "{impression_time_string}"
///     }}
///   ]
/// }}"#);
///
/// let data: AdImpressions = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.impressions.len(), 1);
///
/// assert_eq!(data.impressions[0].device_info.os_type, "Desktop");
///
/// assert_eq!(data.impressions[0].display_location, "TweetConversation");
///
/// if let Some(promoted_tweet_info) = &data.impressions[0].promoted_tweet_info {
///     assert_eq!(promoted_tweet_info.tweet_id, "1111111111111111111");
///     assert_eq!(promoted_tweet_info.tweet_text, "Click bate");
///     assert_eq!(promoted_tweet_info.urls.len(), 0);
///     assert_eq!(promoted_tweet_info.media_urls.len(), 1);
///     assert_eq!(promoted_tweet_info.media_urls[0], "https://t.co/AHAAAAAAAA");
/// }
///
/// if let Some(advertiser_name) = &data.impressions[0].advertiser_info.advertiser_name {
///     assert_eq!(advertiser_name, "EXAMPLE");
/// }
/// if let Some(screen_name) = &data.impressions[0].advertiser_info.screen_name {
///     assert_eq!(screen_name, "@EXAMPLE");
/// }
///
/// if let Some(matched_targeting_criteria) = &data.impressions[0].matched_targeting_criteria {
///     assert_eq!(matched_targeting_criteria.len(), 1);
///     assert_eq!(matched_targeting_criteria[0].targeting_type, "Follower look-alikes");
///     if let Some(targeting_value) = &matched_targeting_criteria[0].targeting_value {
///         assert_eq!(targeting_value, "@EXAMPLE");
///     }
/// }
///
/// assert_eq!(data.impressions[0].impression_time, impression_time_date_time);
///
/// // Re-serialize is equivalent to original data without pretty printing
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
pub struct AdImpressions {
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "impressions": [
	///     {
	///       "deviceInfo": {
	///         "osType": "Desktop"
	///       },
	///       "displayLocation": "TweetConversation",
	///       "promotedTweetInfo": {
	///         "tweetId": "1111111111111111111",
	///         "tweetText": "Click bate",
	///         "urls": [],
	///         "mediaUrls": [
	///           "https://t.co/AHAAAAAAAA"
	///         ]
	///       },
	///       "advertiserInfo": {
	///         "advertiserName": "EXAMPLE",
	///         "screenName": "@EXAMPLE"
	///       },
	///       "matchedTargetingCriteria": [
	///         {
	///           "targetingType": "Follower look-alikes",
	///           "targetingValue": "@EXAMPLE"
	///         }
	///       ],
	///       "impressionTime": "2023-06-05 17:00:52"
	///     }
	///   ]
	/// }
	/// ```
	pub impressions: Vec<ad::Impression>,
}
