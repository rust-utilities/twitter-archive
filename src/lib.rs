#!/usr/bin/env rust

//! Serde structs, deserialize, and serialize definitions for Twitter archived data

#![forbid(unsafe_code)]
#![deny(clippy::all)]
#![deny(missing_docs)]

/// Various functions for facilitating conversion between JSON and Rust values
pub mod convert {
	/// Convert Rust `DateTime` type to/from `tweets[].tweet.created_at` string
	pub mod created_at;

	/// Convert Rust `DateTime` type to/from strings found in;
	///
	/// - `tweets[].tweet.edit_info.initial.editableUntil`
	/// - `direct_messages[].dmConversation.messages[].messageCreate.createdAt`
	pub mod date_time_iso_8601;

	/// Convert Rust `DateTime` type to/from strings found in;
	///
	/// - `ni_devices[].niDeviceResponse.messagingDevice.updatedDate`
	/// - `ni_devices[].niDeviceResponse.messagingDevice.createdDate`
	pub mod date_year_month_day;

	/// Convert Rust `DateTime` type to/from strings found in;
	///
	/// - `ad_impressions[].ad.adsUserData.adImpressions.impressions[].impressionTime`
	pub mod date_year_month_day_hour_minute_second;

	/// Convert Rust `[usize; 2]` type to/from array of strings found mostly within;
	///
	/// - `tweets[].tweet.entities.hashtags[].indices`
	/// - `tweets[].tweet.entities.symbols[].indices`
	/// - `tweets[].tweet.entities.user_mentions[].indices`
	/// - `tweets[].tweet.entities.urls[].indices`
	pub mod indices;

	/// Convert Rust `usize` type to/from strings unlikely to overflow `usize::MAX`
	pub mod number_like_string;
}

/// Data structures that allow `serde` to better understand Mr. Musk's vision
pub mod structs {
	/// Describe data within `twitter-<uuid>.zip:data/manifest.js` file
	pub mod manifest;

	/// Describe data within `twitter-<uuid>.zip:data/account-timezone.js` file
	pub mod account_timezone;

	/// Describe data within `twitter-<uuid>.zip:data/account.js` file
	pub mod account;

	/// Describe entries common between;
	///
	/// - `twitter-<uuid>.zip:data/ad-engagements.js`
	/// - `twitter-<uuid>.zip:data/ad-impressions.js`
	pub mod ad;

	/// Describe data within `twitter-<uuid>.zip:data/ad-engagements.js` file
	pub mod ad_engagements;

	/// Describe data within `twitter-<uuid>.zip:data/ad-impressions.js` file
	pub mod ad_impressions;

	/// Describe data within `twitter-<uuid>.zip:data/block.js` file
	pub mod block;

	/// Describe data within `twitter-<uuid>.zip:data/community-note-rating.js` file
	pub mod community_note_rating;

	/// Describe data within `twitter-<uuid>.zip:data/connected-application.js` file
	pub mod connected_application;

	/// Describe data within `twitter-<uuid>.zip:data/deleted-tweet-headers.js` file
	pub mod deleted_tweet_headers;

	/// Describe data within `twitter-<uuid>.zip:data/device-token.js` file
	pub mod device_token;

	/// Describe data within `twitter-<uuid>.zip:data/direct-message-group-headers.js` file
	pub mod direct_message_group_headers;

	/// Describe data within `twitter-<uuid>.zip:data/direct-message-headers.js` file
	pub mod direct_message_headers;

	/// Describe entries common between;
	///
	/// - `twitter-<uuid>.zip:data/direct-messages.js`
	/// - `twitter-<uuid>.zip:data/direct-messages-group.js`
	pub mod direct_message;

	/// Describe data within `twitter-<uuid>.zip:data/direct-messages.js` file
	pub mod direct_messages;

	/// Describe data within `twitter-<uuid>.zip:data/direct-messages.js` file
	pub mod direct_messages_group;

	/// Describe data within `twitter-<uuid>.zip:data/email-address-change.js` file
	pub mod email_address_change;

	/// Describe entries common between;
	///
	/// - `twitter-<uuid>.zip:data/following.js`
	/// - `twitter-<uuid>.zip:data/follower.js`
	pub mod follow;

	/// Describe data within `twitter-<uuid>.zip:data/follower.js` file
	pub mod follower;

	/// Describe data within `twitter-<uuid>.zip:data/following.js` file
	pub mod following;

	/// Describe data within `twitter-<uuid>.zip:data/ip-audit.js` file
	pub mod ip_audit;

	/// Describe data within `twitter-<uuid>.zip:data/key-registry.js` file
	pub mod key_registry;

	/// Describe data within `twitter-<uuid>.zip:data/like.js` file
	pub mod like;

	/// Describe data within `twitter-<uuid>.zip:data/lists-member.js` file
	pub mod lists_member;

	/// Describe data within `twitter-<uuid>.zip:data/mute.js` file
	pub mod mute;

	/// Describe data within `twitter-<uuid>.zip:data/ni-devices.js` file
	pub mod ni_devices;

	/// Describe data within `twitter-<uuid>.zip:data/personalization.js` file
	pub mod personalization;

	/// Describe data within `twitter-<uuid>.zip:data/phone-number.js` file
	pub mod phone_number;

	/// Describe data within `twitter-<uuid>.zip:data/profile.js` file
	pub mod profile;

	/// Describe data within `twitter-<uuid>.zip:data/screen-name-change.js` file
	pub mod screen_name_change;

	/// Describe data within `twitter-<uuid>.zip:data/tweets.js` file
	pub mod tweets;

	/// Describe data within `twitter-<uuid>.zip:data/twitter-headers.js` file
	pub mod tweet_headers;

	/// Describe data within `twitter-<uuid>.zip:data/tweetdeck.js` file
	pub mod tweetdeck;

	/// Describe data within `twitter-<uuid>.zip:data/twitter-circle.js` file
	pub mod twitter_circle;

	/// Describe data within `twitter-<uuid>.zip:data/verified.js` file
	pub mod verified;
}
