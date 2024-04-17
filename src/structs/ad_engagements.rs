#!/usr/bin/env rust

//! Tweeter archives as of 2023-08-31 have private blocking data found under;
//!
//!   twitter-<DATE>-<UID>.zip:data/ad-engagements.js
//!
//! ## Example file reader for `twitter-<DATE>-<UID>.zip:data/ad-engagements.js`
//!
//! ```no_build
//! use std::io::Read;
//! use std::{fs, path};
//! use zip::read::ZipArchive;
//!
//! use twitter_archive::structs::ad_engagements;
//!
//! fn main() {
//!     let input_file = "~/Downloads/twitter-archive.zip";
//!
//!     let file_descriptor = fs::File::open(input_file).expect("Unable to read --input-file");
//!     let mut zip_archive = ZipArchive::new(file_descriptor).unwrap();
//!     let mut zip_file = zip_archive.by_name("data/ad-engagements.js").unwrap();
//!     let mut buff = String::new();
//!     zip_file.read_to_string(&mut buff).unwrap();
//!
//!     let json = buff.replacen("window.YTD.ad_engagements.part0 = ", "", 1);
//!     let data: Vec<ad_engagements::AdObject> = serde_json::from_str(&json).expect("Unable to parse");
//!
//!     for (index_ad, object_ad) in data.iter().enumerate() {
//!         /* Do stuff with each advertisement */
//!         println!("Advertisement index: {index_ad}");
//!         let engagements = &object_ad.ad.ads_user_data.ad_engagements.engagements;
//!         for (index_engagement, object_engagement) in engagements.iter().enumerate() {
//!             if let Some(promoted_tweet_info) = &object_engagement.impression_attributes.promoted_tweet_info {
//!                 println!("Promoted Tweet ID: {}", promoted_tweet_info.tweet_id);
//!                 println!("Promoted Tweet text: {}", promoted_tweet_info.tweet_text);
//!             }
//!             println!("Impression time: {}", object_engagement.impression_attributes.impression_time);
//!         }
//!     }
//! }
//! ```
//!
//! ## Example `twitter-<DATE>-<UID>.zip:data/ad-engagements.js` content
//!
//! ```javascript
//! window.YTD.ad_engagements.part0 = [
//!   {
//!     "ad" : {
//!       "adsUserData" : {
//!         "adEngagements" : {
//!           "engagements" : [
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

use chrono::{DateTime, Utc};
use derive_more::Display;
use serde::{Deserialize, Serialize};

use crate::convert;
use crate::structs::ad;

/// ## Example
///
/// ```
/// use chrono::{DateTime, NaiveDateTime, Utc};
///
/// use twitter_archive::convert::date_year_month_day_hour_minute_second::FORMAT;
///
/// use twitter_archive::structs::ad_engagements::AdObject;
///
/// let impression_time_string = "2023-06-05 17:00:52";
/// let impression_time_native_time = NaiveDateTime::parse_from_str(&impression_time_string, FORMAT).unwrap();
/// let impression_time_date_time = DateTime::<Utc>::from_naive_utc_and_offset(impression_time_native_time, Utc);
///
/// let engagement_time_string = "2023-06-05 17:00:52";
/// let engagement_time_native_time = NaiveDateTime::parse_from_str(&engagement_time_string, FORMAT).unwrap();
/// let engagement_time_date_time = DateTime::<Utc>::from_naive_utc_and_offset(engagement_time_native_time, Utc);
///
/// let json = format!(r#"{{
///   "ad": {{
///     "adsUserData": {{
///       "adEngagements": {{
///         "engagements": [
///           {{
///             "impressionAttributes": {{
///               "deviceInfo": {{
///                 "osType": "Desktop"
///               }},
///               "displayLocation": "TweetConversation",
///               "promotedTweetInfo": {{
///                 "tweetId": "1111111111111111111",
///                 "tweetText": "Click bate",
///                 "urls": [],
///                 "mediaUrls": [
///                   "https://t.co/AHAAAAAAAA"
///                 ]
///               }},
///               "advertiserInfo": {{
///                 "advertiserName": "EXAMPLE",
///                 "screenName": "@EXAMPLE"
///               }},
///               "matchedTargetingCriteria": [
///                 {{
///                   "targetingType": "Follower look-alikes",
///                   "targetingValue": "@EXAMPLE"
///                 }}
///               ],
///               "impressionTime": "{impression_time_string}"
///             }},
///             "engagementAttributes": [
///               {{
///                 "engagementTime": "{engagement_time_string}",
///                 "engagementType": "ChargeableImpression"
///               }},
///               {{
///                 "engagementTime": "{engagement_time_string}",
///                 "engagementType": "Mute"
///               }}
///             ]
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
/// assert_eq!(data.ad.ads_user_data.ad_engagements.engagements.len(), 1);
///
/// assert_eq!(data.ad.ads_user_data.ad_engagements.engagements[0].impression_attributes.device_info.os_type, "Desktop");
///
/// assert_eq!(data.ad.ads_user_data.ad_engagements.engagements[0].impression_attributes.display_location, "TweetConversation");
///
/// if let Some(promoted_tweet_info) = &data.ad.ads_user_data.ad_engagements.engagements[0].impression_attributes.promoted_tweet_info {
///     assert_eq!(promoted_tweet_info.tweet_id, "1111111111111111111");
///     assert_eq!(promoted_tweet_info.tweet_text, "Click bate");
///     assert_eq!(promoted_tweet_info.urls.len(), 0);
///     assert_eq!(promoted_tweet_info.media_urls.len(), 1);
///     assert_eq!(promoted_tweet_info.media_urls[0], "https://t.co/AHAAAAAAAA");
/// }
///
/// if let Some(advertiser_name) = &data.ad.ads_user_data.ad_engagements.engagements[0].impression_attributes.advertiser_info.advertiser_name {
///     assert_eq!(advertiser_name, "EXAMPLE");
/// }
/// if let Some(screen_name) = &data.ad.ads_user_data.ad_engagements.engagements[0].impression_attributes.advertiser_info.screen_name {
///     assert_eq!(screen_name, "@EXAMPLE");
/// }
///
/// if let Some(matched_targeting_criteria) = &data.ad.ads_user_data.ad_engagements.engagements[0].impression_attributes.matched_targeting_criteria {
///     assert_eq!(matched_targeting_criteria.len(), 1);
///     assert_eq!(matched_targeting_criteria[0].targeting_type, "Follower look-alikes");
///     if let Some(targeting_value) = &matched_targeting_criteria[0].targeting_value {
///         assert_eq!(targeting_value, "@EXAMPLE");
///     }
/// }
///
/// assert_eq!(data.ad.ads_user_data.ad_engagements.engagements[0].impression_attributes.impression_time, impression_time_date_time);
///
/// assert_eq!(data.ad.ads_user_data.ad_engagements.engagements[0].engagement_attributes[0].engagement_time, engagement_time_date_time);
/// assert_eq!(data.ad.ads_user_data.ad_engagements.engagements[0].engagement_attributes[0].engagement_type, "ChargeableImpression");
/// assert_eq!(data.ad.ads_user_data.ad_engagements.engagements[0].engagement_attributes[1].engagement_time, engagement_time_date_time);
/// assert_eq!(data.ad.ads_user_data.ad_engagements.engagements[0].engagement_attributes[1].engagement_type, "Mute");
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
	///       "adEngagements": {
	///         "engagements": [
	///           {
	///             "impressionAttributes": {
	///               "deviceInfo": {
	///                 "osType": "Desktop"
	///               },
	///               "displayLocation": "TweetConversation",
	///               "promotedTweetInfo": {
	///                 "tweetId": "1111111111111111111",
	///                 "tweetText": "Click bate",
	///                 "urls": [],
	///                 "mediaUrls": [
	///                   "https://t.co/AHAAAAAAAA"
	///                 ]
	///               },
	///               "advertiserInfo": {
	///                 "advertiserName": "EXAMPLE",
	///                 "screenName": "@EXAMPLE"
	///               },
	///               "matchedTargetingCriteria": [
	///                 {
	///                   "targetingType": "Follower look-alikes",
	///                   "targetingValue": "@EXAMPLE"
	///                 }
	///               ],
	///               "impressionTime": "2023-06-05 17:00:52"
	///             },
	///             "engagementAttributes": [
	///               {
	///                 "engagementTime": "2023-06-05 17:00:52",
	///                 "engagementType": "ChargeableImpression"
	///               },
	///               {
	///                 "engagementTime": "2023-06-05 17:00:52",
	///                 "engagementType": "Mute"
	///               }
	///             ]
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
/// use twitter_archive::structs::ad_engagements::Ad;
///
/// let impression_time_string = "2023-06-05 17:00:52";
/// let impression_time_native_time = NaiveDateTime::parse_from_str(&impression_time_string, FORMAT).unwrap();
/// let impression_time_date_time = DateTime::<Utc>::from_naive_utc_and_offset(impression_time_native_time, Utc);
///
/// let engagement_time_string = "2023-06-05 17:00:52";
/// let engagement_time_native_time = NaiveDateTime::parse_from_str(&engagement_time_string, FORMAT).unwrap();
/// let engagement_time_date_time = DateTime::<Utc>::from_naive_utc_and_offset(engagement_time_native_time, Utc);
///
/// let json = format!(r#"{{
///   "adsUserData": {{
///     "adEngagements": {{
///       "engagements": [
///         {{
///           "impressionAttributes": {{
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
///           }},
///           "engagementAttributes": [
///             {{
///               "engagementTime": "{engagement_time_string}",
///               "engagementType": "ChargeableImpression"
///             }},
///             {{
///               "engagementTime": "{engagement_time_string}",
///               "engagementType": "Mute"
///             }}
///           ]
///         }}
///       ]
///     }}
///   }}
/// }}"#);
///
/// let data: Ad = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.ads_user_data.ad_engagements.engagements.len(), 1);
///
/// assert_eq!(data.ads_user_data.ad_engagements.engagements[0].impression_attributes.device_info.os_type, "Desktop");
///
/// assert_eq!(data.ads_user_data.ad_engagements.engagements[0].impression_attributes.display_location, "TweetConversation");
///
/// if let Some(promoted_tweet_info) = &data.ads_user_data.ad_engagements.engagements[0].impression_attributes.promoted_tweet_info {
///     assert_eq!(promoted_tweet_info.tweet_id, "1111111111111111111");
///     assert_eq!(promoted_tweet_info.tweet_text, "Click bate");
///     assert_eq!(promoted_tweet_info.urls.len(), 0);
///     assert_eq!(promoted_tweet_info.media_urls.len(), 1);
///     assert_eq!(promoted_tweet_info.media_urls[0], "https://t.co/AHAAAAAAAA");
/// }
///
/// if let Some(advertiser_name) = &data.ads_user_data.ad_engagements.engagements[0].impression_attributes.advertiser_info.advertiser_name {
///     assert_eq!(advertiser_name, "EXAMPLE");
/// }
/// if let Some(screen_name) = &data.ads_user_data.ad_engagements.engagements[0].impression_attributes.advertiser_info.screen_name {
///     assert_eq!(screen_name, "@EXAMPLE");
/// }
///
/// if let Some(matched_targeting_criteria) = &data.ads_user_data.ad_engagements.engagements[0].impression_attributes.matched_targeting_criteria {
///     assert_eq!(matched_targeting_criteria.len(), 1);
///     assert_eq!(matched_targeting_criteria[0].targeting_type, "Follower look-alikes");
///     if let Some(targeting_value) = &matched_targeting_criteria[0].targeting_value {
///         assert_eq!(targeting_value, "@EXAMPLE");
///     }
/// }
///
/// assert_eq!(data.ads_user_data.ad_engagements.engagements[0].impression_attributes.impression_time, impression_time_date_time);
///
/// assert_eq!(data.ads_user_data.ad_engagements.engagements[0].engagement_attributes[0].engagement_time, engagement_time_date_time);
/// assert_eq!(data.ads_user_data.ad_engagements.engagements[0].engagement_attributes[0].engagement_type, "ChargeableImpression");
/// assert_eq!(data.ads_user_data.ad_engagements.engagements[0].engagement_attributes[1].engagement_time, engagement_time_date_time);
/// assert_eq!(data.ads_user_data.ad_engagements.engagements[0].engagement_attributes[1].engagement_type, "Mute");
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
	///     "adEngagements": {
	///       "engagements": [
	///         {
	///           "impressionAttributes": {
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
	///           },
	///           "engagementAttributes": [
	///             {
	///               "engagementTime": "2023-06-05 17:00:52",
	///               "engagementType": "ChargeableImpression"
	///             },
	///             {
	///               "engagementTime": "2023-06-05 17:00:52",
	///               "engagementType": "Mute"
	///             }
	///           ]
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
/// use twitter_archive::structs::ad_engagements::AdsUserData;
///
/// let impression_time_string = "2023-06-05 17:00:52";
/// let impression_time_native_time = NaiveDateTime::parse_from_str(&impression_time_string, FORMAT).unwrap();
/// let impression_time_date_time = DateTime::<Utc>::from_naive_utc_and_offset(impression_time_native_time, Utc);
///
/// let engagement_time_string = "2023-06-05 17:00:52";
/// let engagement_time_native_time = NaiveDateTime::parse_from_str(&engagement_time_string, FORMAT).unwrap();
/// let engagement_time_date_time = DateTime::<Utc>::from_naive_utc_and_offset(engagement_time_native_time, Utc);
///
/// let json = format!(r#"{{
///   "adEngagements": {{
///     "engagements": [
///       {{
///         "impressionAttributes": {{
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
///         }},
///         "engagementAttributes": [
///           {{
///             "engagementTime": "{engagement_time_string}",
///             "engagementType": "ChargeableImpression"
///           }},
///           {{
///             "engagementTime": "{engagement_time_string}",
///             "engagementType": "Mute"
///           }}
///         ]
///       }}
///     ]
///   }}
/// }}"#);
///
/// let data: AdsUserData = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.ad_engagements.engagements.len(), 1);
///
/// assert_eq!(data.ad_engagements.engagements[0].impression_attributes.device_info.os_type, "Desktop");
///
/// assert_eq!(data.ad_engagements.engagements[0].impression_attributes.display_location, "TweetConversation");
///
/// if let Some(promoted_tweet_info) = &data.ad_engagements.engagements[0].impression_attributes.promoted_tweet_info {
///     assert_eq!(promoted_tweet_info.tweet_id, "1111111111111111111");
///     assert_eq!(promoted_tweet_info.tweet_text, "Click bate");
///     assert_eq!(promoted_tweet_info.urls.len(), 0);
///     assert_eq!(promoted_tweet_info.media_urls.len(), 1);
///     assert_eq!(promoted_tweet_info.media_urls[0], "https://t.co/AHAAAAAAAA");
/// }
///
/// if let Some(advertiser_name) = &data.ad_engagements.engagements[0].impression_attributes.advertiser_info.advertiser_name {
///     assert_eq!(advertiser_name, "EXAMPLE");
/// }
/// if let Some(screen_name) = &data.ad_engagements.engagements[0].impression_attributes.advertiser_info.screen_name {
///     assert_eq!(screen_name, "@EXAMPLE");
/// }
///
/// if let Some(matched_targeting_criteria) = &data.ad_engagements.engagements[0].impression_attributes.matched_targeting_criteria {
///     assert_eq!(matched_targeting_criteria.len(), 1);
///     assert_eq!(matched_targeting_criteria[0].targeting_type, "Follower look-alikes");
///     if let Some(targeting_value) = &matched_targeting_criteria[0].targeting_value {
///         assert_eq!(targeting_value, "@EXAMPLE");
///     }
/// }
///
/// assert_eq!(data.ad_engagements.engagements[0].impression_attributes.impression_time, impression_time_date_time);
///
/// assert_eq!(data.ad_engagements.engagements[0].engagement_attributes[0].engagement_time, engagement_time_date_time);
/// assert_eq!(data.ad_engagements.engagements[0].engagement_attributes[0].engagement_type, "ChargeableImpression");
/// assert_eq!(data.ad_engagements.engagements[0].engagement_attributes[1].engagement_time, engagement_time_date_time);
/// assert_eq!(data.ad_engagements.engagements[0].engagement_attributes[1].engagement_type, "Mute");
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
	///   "adEngagements": {
	///     "engagements": [
	///       {
	///         "impressionAttributes": {
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
	///         },
	///         "engagementAttributes": [
	///           {
	///             "engagementTime": "2023-06-05 17:00:52",
	///             "engagementType": "ChargeableImpression"
	///           },
	///           {
	///             "engagementTime": "2023-06-05 17:00:52",
	///             "engagementType": "Mute"
	///           }
	///         ]
	///       }
	///     ]
	///   }
	/// }
	/// ```
	pub ad_engagements: AdEngagements,
}

/// ## Example
///
/// ```
/// use chrono::{DateTime, NaiveDateTime, Utc};
///
/// use twitter_archive::convert::date_year_month_day_hour_minute_second::FORMAT;
///
/// use twitter_archive::structs::ad_engagements::AdEngagements;
///
/// let impression_time_string = "2023-06-05 17:00:52";
/// let impression_time_native_time = NaiveDateTime::parse_from_str(&impression_time_string, FORMAT).unwrap();
/// let impression_time_date_time = DateTime::<Utc>::from_naive_utc_and_offset(impression_time_native_time, Utc);
///
/// let engagement_time_string = "2023-06-05 17:00:52";
/// let engagement_time_native_time = NaiveDateTime::parse_from_str(&engagement_time_string, FORMAT).unwrap();
/// let engagement_time_date_time = DateTime::<Utc>::from_naive_utc_and_offset(engagement_time_native_time, Utc);
///
/// let json = format!(r#"{{
///   "engagements": [
///     {{
///       "impressionAttributes": {{
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
///       }},
///       "engagementAttributes": [
///         {{
///           "engagementTime": "{engagement_time_string}",
///           "engagementType": "ChargeableImpression"
///         }},
///         {{
///           "engagementTime": "{engagement_time_string}",
///           "engagementType": "Mute"
///         }}
///       ]
///     }}
///   ]
/// }}"#);
///
/// let data: AdEngagements = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.engagements.len(), 1);
///
/// assert_eq!(data.engagements[0].impression_attributes.device_info.os_type, "Desktop");
///
/// assert_eq!(data.engagements[0].impression_attributes.display_location, "TweetConversation");
///
/// if let Some(promoted_tweet_info) = &data.engagements[0].impression_attributes.promoted_tweet_info {
///     assert_eq!(promoted_tweet_info.tweet_id, "1111111111111111111");
///     assert_eq!(promoted_tweet_info.tweet_text, "Click bate");
///     assert_eq!(promoted_tweet_info.urls.len(), 0);
///     assert_eq!(promoted_tweet_info.media_urls.len(), 1);
///     assert_eq!(promoted_tweet_info.media_urls[0], "https://t.co/AHAAAAAAAA");
/// }
///
/// if let Some(advertiser_name) = &data.engagements[0].impression_attributes.advertiser_info.advertiser_name {
///     assert_eq!(advertiser_name, "EXAMPLE");
/// }
/// if let Some(screen_name) = &data.engagements[0].impression_attributes.advertiser_info.screen_name {
///     assert_eq!(screen_name, "@EXAMPLE");
/// }
///
/// if let Some(matched_targeting_criteria) = &data.engagements[0].impression_attributes.matched_targeting_criteria {
///     assert_eq!(matched_targeting_criteria.len(), 1);
///     assert_eq!(matched_targeting_criteria[0].targeting_type, "Follower look-alikes");
///     if let Some(targeting_value) = &matched_targeting_criteria[0].targeting_value {
///         assert_eq!(targeting_value, "@EXAMPLE");
///     }
/// }
///
/// assert_eq!(data.engagements[0].impression_attributes.impression_time, impression_time_date_time);
///
/// assert_eq!(data.engagements[0].engagement_attributes[0].engagement_time, engagement_time_date_time);
/// assert_eq!(data.engagements[0].engagement_attributes[0].engagement_type, "ChargeableImpression");
/// assert_eq!(data.engagements[0].engagement_attributes[1].engagement_time, engagement_time_date_time);
/// assert_eq!(data.engagements[0].engagement_attributes[1].engagement_type, "Mute");
///
/// // Re-serialize is equivalent to original data without pretty printing
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
pub struct AdEngagements {
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "engagements": [
	///     {
	///       "impressionAttributes": {
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
	///       },
	///       "engagementAttributes": [
	///         {
	///           "engagementTime": "2023-06-05 17:00:52",
	///           "engagementType": "ChargeableImpression"
	///         },
	///         {
	///           "engagementTime": "2023-06-05 17:00:52",
	///           "engagementType": "Mute"
	///         }
	///       ]
	///     }
	///   ]
	/// }
	/// ```
	pub engagements: Vec<Engagement>,
}

/// ## Example
///
/// ```
/// use chrono::{DateTime, NaiveDateTime, Utc};
///
/// use twitter_archive::convert::date_year_month_day_hour_minute_second::FORMAT;
///
/// use twitter_archive::structs::ad_engagements::Engagement;
///
/// let impression_time_string = "2023-06-05 17:00:52";
/// let impression_time_native_time = NaiveDateTime::parse_from_str(&impression_time_string, FORMAT).unwrap();
/// let impression_time_date_time = DateTime::<Utc>::from_naive_utc_and_offset(impression_time_native_time, Utc);
///
/// let engagement_time_string = "2023-06-05 17:00:52";
/// let engagement_time_native_time = NaiveDateTime::parse_from_str(&engagement_time_string, FORMAT).unwrap();
/// let engagement_time_date_time = DateTime::<Utc>::from_naive_utc_and_offset(engagement_time_native_time, Utc);
///
/// let json = format!(r#"{{
///   "impressionAttributes": {{
///     "deviceInfo": {{
///       "osType": "Desktop"
///     }},
///     "displayLocation": "TweetConversation",
///     "promotedTweetInfo": {{
///       "tweetId": "1111111111111111111",
///       "tweetText": "Click bate",
///       "urls": [],
///       "mediaUrls": [
///         "https://t.co/AHAAAAAAAA"
///       ]
///     }},
///     "advertiserInfo": {{
///       "advertiserName": "EXAMPLE",
///       "screenName": "@EXAMPLE"
///     }},
///     "matchedTargetingCriteria": [
///       {{
///         "targetingType": "Follower look-alikes",
///         "targetingValue": "@EXAMPLE"
///       }}
///     ],
///     "impressionTime": "{impression_time_string}"
///   }},
///   "engagementAttributes": [
///     {{
///       "engagementTime": "{engagement_time_string}",
///       "engagementType": "ChargeableImpression"
///     }},
///     {{
///       "engagementTime": "{engagement_time_string}",
///       "engagementType": "Mute"
///     }}
///   ]
/// }}"#);
///
/// let data: Engagement = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.impression_attributes.device_info.os_type, "Desktop");
///
/// assert_eq!(data.impression_attributes.display_location, "TweetConversation");
///
/// if let Some(promoted_tweet_info) = &data.impression_attributes.promoted_tweet_info {
///     assert_eq!(promoted_tweet_info.tweet_id, "1111111111111111111");
///     assert_eq!(promoted_tweet_info.tweet_text, "Click bate");
///     assert_eq!(promoted_tweet_info.urls.len(), 0);
///     assert_eq!(promoted_tweet_info.media_urls.len(), 1);
///     assert_eq!(promoted_tweet_info.media_urls[0], "https://t.co/AHAAAAAAAA");
/// }
///
/// if let Some(advertiser_name) = &data.impression_attributes.advertiser_info.advertiser_name {
///     assert_eq!(advertiser_name, "EXAMPLE");
/// }
/// if let Some(screen_name) = &data.impression_attributes.advertiser_info.screen_name {
///     assert_eq!(screen_name, "@EXAMPLE");
/// }
///
/// if let Some(matched_targeting_criteria) = &data.impression_attributes.matched_targeting_criteria {
///     assert_eq!(matched_targeting_criteria.len(), 1);
///     assert_eq!(matched_targeting_criteria[0].targeting_type, "Follower look-alikes");
///     if let Some(targeting_value) = &matched_targeting_criteria[0].targeting_value {
///         assert_eq!(targeting_value, "@EXAMPLE");
///     }
/// }
///
/// assert_eq!(data.impression_attributes.impression_time, impression_time_date_time);
///
/// assert_eq!(data.engagement_attributes[0].engagement_time, engagement_time_date_time);
/// assert_eq!(data.engagement_attributes[0].engagement_type, "ChargeableImpression");
/// assert_eq!(data.engagement_attributes[1].engagement_time, engagement_time_date_time);
/// assert_eq!(data.engagement_attributes[1].engagement_type, "Mute");
///
/// // Re-serialize is equivalent to original data without pretty printing
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct Engagement {
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "impressionAttributes": {
	///     "deviceInfo": {
	///       "osType": "Desktop"
	///     },
	///     "displayLocation": "TweetConversation",
	///     "promotedTweetInfo": {
	///       "tweetId": "1111111111111111111",
	///       "tweetText": "Click bate",
	///       "urls": [],
	///       "mediaUrls": [
	///         "https://t.co/AHAAAAAAAA"
	///       ]
	///     },
	///     "advertiserInfo": {
	///       "advertiserName": "EXAMPLE",
	///       "screenName": "@EXAMPLE"
	///     },
	///     "matchedTargetingCriteria": [
	///       {
	///         "targetingType": "Follower look-alikes",
	///         "targetingValue": "@EXAMPLE"
	///       }
	///     ],
	///     "impressionTime": "2023-06-05 17:00:52"
	///   }
	/// }
	/// ```
	pub impression_attributes: ad::Impression,

	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "engagementAttributes": [
	///     {
	///       "engagementTime": "2023-06-05 17:00:52",
	///       "engagementType": "ChargeableImpression"
	///     },
	///     {
	///       "engagementTime": "2023-06-05 17:00:52",
	///       "engagementType": "Mute"
	///     }
	///   ]
	/// }
	/// ```
	pub engagement_attributes: Vec<EngagementAttributes>,
}

/// ## Example
///
/// ```
/// use chrono::{DateTime, NaiveDateTime, Utc};
///
/// use twitter_archive::convert::date_year_month_day_hour_minute_second::FORMAT;
///
/// use twitter_archive::structs::ad_engagements::EngagementAttributes;
///
/// let engagement_time_string = "2023-06-05 17:00:52";
/// let engagement_time_native_time = NaiveDateTime::parse_from_str(&engagement_time_string, FORMAT).unwrap();
/// let engagement_time_date_time = DateTime::<Utc>::from_naive_utc_and_offset(engagement_time_native_time, Utc);
///
/// let json = format!(r#"{{
///   "engagementTime": "{engagement_time_string}",
///   "engagementType": "ChargeableImpression"
/// }}"#);
///
/// let data: EngagementAttributes = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.engagement_time, engagement_time_date_time);
/// assert_eq!(data.engagement_type, "ChargeableImpression");
///
/// // Re-serialize is equivalent to original data without pretty printing
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct EngagementAttributes {
	/// ## Example JSON data
	///
	/// ```json
	/// { "engagementTime": "{engagement_time_string}" }
	/// ```
	#[serde(with = "convert::date_year_month_day_hour_minute_second")]
	pub engagement_time: DateTime<Utc>,

	/// ## Example JSON data
	///
	/// ```json
	/// { "engagementType": "ChargeableImpression" }
	/// ```
	pub engagement_type: String,
}
