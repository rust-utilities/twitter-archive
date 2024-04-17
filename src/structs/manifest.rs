#!/usr/bin/env rust

//! Tweeter archives as of 2023-08-31 have private blocking data found under;
//!
//!   twitter-<DATE>-<UID>.zip:data/manifest.js
//!
//! ## Example file reader for `twitter-<DATE>-<UID>.zip:data/manifest.js`
//!
//! ```no_build
//! use std::io::Read;
//! use std::{fs, path};
//! use zip::read::ZipArchive;
//!
//! use twitter_archive::structs::manifest;
//!
//! fn main() {
//!     let input_file = "~/Downloads/twitter-archive.zip";
//!
//!     let file_descriptor = fs::File::open(input_file).expect("Unable to read --input-file");
//!     let mut zip_archive = ZipArchive::new(file_descriptor).unwrap();
//!     let mut zip_file = zip_archive.by_name("data/manifest.js").unwrap();
//!     let mut buff = String::new();
//!     zip_file.read_to_string(&mut buff).unwrap();
//!
//!     let json = buff.replacen("window.__THAR_CONFIG = ", "", 1);
//!     let data: manifest::Manifest = serde_json::from_str(&json).expect("Unable to parse");
//!
//!     // Extract specific data and format output to be similar to YAML syntax
//!     println!("User info:");
//!     println!("  Account ID: {}", data.user_info.account_id);
//!     println!("  User name: {}", data.user_info.user_name);
//!     println!("  Display name: {}", data.user_info.display_name);
//!
//!     println!("Archive info:");
//!     println!("  Generation date: {}", data.archive_info.generation_date);
//!     println!("  Is partial archive: {}", data.archive_info.is_partial_archive);
//!
//!     println!("Data types:");
//!     println!("  Tweets:");
//!     println!("    - Media directory: {}", data.data_types.tweets.media_directory);
//!     println!("      Files:");
//!     for file in &data.data_types.tweets.files {
//!         println!("        - File name: {}", file.file_name);
//!         println!("          Global name: {}", file.global_name);
//!         println!("          Count: {}", file.count);
//!     }
//! }
//! ```
//!
//! ## Example `twitter-<DATE>-<UID>.zip:data/account.js` content
//!
//! ```javascript
//! window.__THAR_CONFIG = [
//!   "userInfo": {
//!     "accountId": "111111111",
//!     "userName": "S0_And_S0",
//!     "displayName": "S0AndS0.eth"
//!   },
//!   "archiveInfo": {
//!     "sizeBytes": "44546997",
//!     "generationDate": "{generation_date_string}",
//!     "isPartialArchive": false,
//!     "maxPartSizeBytes": "53687091200"
//!   },
//!   "readmeInfo": {
//!     "fileName": "data/README.txt",
//!     "directory": "data/",
//!     "name": "README.txt"
//!   },
//!   "dataTypes": {
//!     "account": {
//!       "files": [
//!         {
//!           "fileName": "data/account.js",
//!           "globalName": "YTD.account.part0",
//!           "count": "1"
//!         }
//!       ]
//!     },
//!     "accountCreationIp": {
//!       "files": [
//!         {
//!           "fileName": "data/account-creation-ip.js",
//!           "globalName": "YTD.account_creation_ip.part0",
//!           "count": "1"
//!         }
//!       ]
//!     },
//!     "accountLabel": {
//!       "files": [
//!         {
//!           "fileName": "data/account-label.js",
//!           "globalName": "YTD.account_label.part0",
//!           "count": "1"
//!         }
//!       ]
//!     },
//!     "accountSuspension": {
//!       "files": [
//!         {
//!           "fileName": "data/account-suspension.js",
//!           "globalName": "YTD.account_suspension.part0",
//!           "count": "0"
//!         }
//!       ]
//!     },
//!     "accountTimezone": {
//!       "files": [
//!         {
//!           "fileName": "data/account-timezone.js",
//!           "globalName": "YTD.account_timezone.part0",
//!           "count": "1"
//!         }
//!       ]
//!     },
//!     "adEngagements": {
//!       "files": [
//!         {
//!           "fileName": "data/ad-engagements.js",
//!           "globalName": "YTD.ad_engagements.part0",
//!           "count": "122"
//!         }
//!       ]
//!     },
//!     "adImpressions": {
//!       "files": [
//!         {
//!           "fileName": "data/ad-impressions.js",
//!           "globalName": "YTD.ad_impressions.part0",
//!           "count": "125"
//!         }
//!       ]
//!     },
//!     "adMobileConversionsAttributed": {
//!       "files": [
//!         {
//!           "fileName": "data/ad-mobile-conversions-attributed.js",
//!           "globalName": "YTD.ad_mobile_conversions_attributed.part0",
//!           "count": "0"
//!         }
//!       ]
//!     },
//!     "adMobileConversionsUnattributed": {
//!       "files": [
//!         {
//!           "fileName": "data/ad-mobile-conversions-unattributed.js",
//!           "globalName": "YTD.ad_mobile_conversions_unattributed.part0",
//!           "count": "0"
//!         }
//!       ]
//!     },
//!     "adOnlineConversionsAttributed": {
//!       "files": [
//!         {
//!           "fileName": "data/ad-online-conversions-attributed.js",
//!           "globalName": "YTD.ad_online_conversions_attributed.part0",
//!           "count": "0"
//!         }
//!       ]
//!     },
//!     "adOnlineConversionsUnattributed": {
//!       "files": [
//!         {
//!           "fileName": "data/ad-online-conversions-unattributed.js",
//!           "globalName": "YTD.ad_online_conversions_unattributed.part0",
//!           "count": "0"
//!         }
//!       ]
//!     },
//!     "ageinfo": {
//!       "files": [
//!         {
//!           "fileName": "data/ageinfo.js",
//!           "globalName": "YTD.ageinfo.part0",
//!           "count": "1"
//!         }
//!       ]
//!     },
//!     "app": {
//!       "files": [
//!         {
//!           "fileName": "data/app.js",
//!           "globalName": "YTD.app.part0",
//!           "count": "0"
//!         }
//!       ]
//!     },
//!     "block": {
//!       "files": [
//!         {
//!           "fileName": "data/block.js",
//!           "globalName": "YTD.block.part0",
//!           "count": "10"
//!         }
//!       ]
//!     },
//!     "branchLinks": {
//!       "files": [
//!         {
//!           "fileName": "data/branch-links.js",
//!           "globalName": "YTD.branch_links.part0",
//!           "count": "0"
//!         }
//!       ]
//!     },
//!     "catalogItem": {
//!       "files": [
//!         {
//!           "fileName": "data/catalog-item.js",
//!           "globalName": "YTD.catalog_item.part0",
//!           "count": "0"
//!         }
//!       ]
//!     },
//!     "commerceCatalog": {
//!       "files": [
//!         {
//!           "fileName": "data/commerce-catalog.js",
//!           "globalName": "YTD.commerce_catalog.part0",
//!           "count": "0"
//!         }
//!       ]
//!     },
//!     "communityNote": {
//!       "files": [
//!         {
//!           "fileName": "data/community-note.js",
//!           "globalName": "YTD.community_note.part0",
//!           "count": "0"
//!         }
//!       ]
//!     },
//!     "communityNoteRating": {
//!       "files": [
//!         {
//!           "fileName": "data/community-note-rating.js",
//!           "globalName": "YTD.community_note_rating.part0",
//!           "count": "1"
//!         }
//!       ]
//!     },
//!     "communityNoteTombstone": {
//!       "files": [
//!         {
//!           "fileName": "data/community-note-tombstone.js",
//!           "globalName": "YTD.community_note_tombstone.part0",
//!           "count": "0"
//!         }
//!       ]
//!     },
//!     "communityTweet": {
//!       "mediaDirectory": "data/community_tweet_media",
//!       "files": [
//!         {
//!           "fileName": "data/community-tweet.js",
//!           "globalName": "YTD.community_tweet.part0",
//!           "count": "0"
//!         }
//!       ]
//!     },
//!     "communityTweetMedia": {
//!       "mediaDirectory": "data/community_tweet_media"
//!     },
//!     "connectedApplication": {
//!       "files": [
//!         {
//!           "fileName": "data/connected-application.js",
//!           "globalName": "YTD.connected_application.part0",
//!           "count": "1"
//!         }
//!       ]
//!     },
//!     "contact": {
//!       "files": [
//!         {
//!           "fileName": "data/contact.js",
//!           "globalName": "YTD.contact.part0",
//!           "count": "0"
//!         }
//!       ]
//!     },
//!     "deletedNoteTweet": {
//!       "files": [
//!         {
//!           "fileName": "data/deleted-note-tweet.js",
//!           "globalName": "YTD.deleted_note_tweet.part0",
//!           "count": "0"
//!         }
//!       ]
//!     },
//!     "deletedTweetHeaders": {
//!       "files": [
//!         {
//!           "fileName": "data/deleted-tweet-headers.js",
//!           "globalName": "YTD.deleted_tweet_headers.part0",
//!           "count": "3"
//!         }
//!       ]
//!     },
//!     "deletedTweets": {
//!       "mediaDirectory": "data/deleted_tweets_media",
//!       "files": [
//!         {
//!           "fileName": "data/deleted-tweets.js",
//!           "globalName": "YTD.deleted_tweets.part0",
//!           "count": "3"
//!         }
//!       ]
//!     },
//!     "deletedTweetsMedia": {
//!       "mediaDirectory": "data/deleted_tweets_media"
//!     },
//!     "deviceToken": {
//!       "files": [
//!         {
//!           "fileName": "data/device-token.js",
//!           "globalName": "YTD.device_token.part0",
//!           "count": "600"
//!         }
//!       ]
//!     },
//!     "directMessageGroupHeaders": {
//!       "files": [
//!         {
//!           "fileName": "data/direct-message-group-headers.js",
//!           "globalName": "YTD.direct_message_group_headers.part0",
//!           "count": "2"
//!         }
//!       ]
//!     },
//!     "directMessageHeaders": {
//!       "files": [
//!         {
//!           "fileName": "data/direct-message-headers.js",
//!           "globalName": "YTD.direct_message_headers.part0",
//!           "count": "84"
//!         }
//!       ]
//!     },
//!     "directMessageMute": {
//!       "files": [
//!         {
//!           "fileName": "data/direct-message-mute.js",
//!           "globalName": "YTD.direct_message_mute.part0",
//!           "count": "0"
//!         }
//!       ]
//!     },
//!     "directMessages": {
//!       "mediaDirectory": "data/direct_messages_media",
//!       "files": [
//!         {
//!           "fileName": "data/direct-messages.js",
//!           "globalName": "YTD.direct_messages.part0",
//!           "count": "84"
//!         }
//!       ]
//!     },
//!     "directMessagesGroup": {
//!       "mediaDirectory": "data/direct_messages_group_media",
//!       "files": [
//!         {
//!           "fileName": "data/direct-messages-group.js",
//!           "globalName": "YTD.direct_messages_group.part0",
//!           "count": "2"
//!         }
//!       ]
//!     },
//!     "directMessagesGroupMedia": {
//!       "mediaDirectory": "data/direct_messages_group_media"
//!     },
//!     "directMessagesMedia": {
//!       "mediaDirectory": "data/direct_messages_media"
//!     },
//!     "emailAddressChange": {
//!       "files": [
//!         {
//!           "fileName": "data/email-address-change.js",
//!           "globalName": "YTD.email_address_change.part0",
//!           "count": "2"
//!         }
//!       ]
//!     },
//!     "follower": {
//!       "files": [
//!         {
//!           "fileName": "data/follower.js",
//!           "globalName": "YTD.follower.part0",
//!           "count": "289"
//!         }
//!       ]
//!     },
//!     "following": {
//!       "files": [
//!         {
//!           "fileName": "data/following.js",
//!           "globalName": "YTD.following.part0",
//!           "count": "283"
//!         }
//!       ]
//!     },
//!     "ipAudit": {
//!       "files": [
//!         {
//!           "fileName": "data/ip-audit.js",
//!           "globalName": "YTD.ip_audit.part0",
//!           "count": "542"
//!         }
//!       ]
//!     },
//!     "keyRegistry": {
//!       "files": [
//!         {
//!           "fileName": "data/key-registry.js",
//!           "globalName": "YTD.key_registry.part0",
//!           "count": "1"
//!         }
//!       ]
//!     },
//!     "like": {
//!       "files": [
//!         {
//!           "fileName": "data/like.js",
//!           "globalName": "YTD.like.part0",
//!           "count": "25548"
//!         }
//!       ]
//!     },
//!     "listsCreated": {
//!       "files": [
//!         {
//!           "fileName": "data/lists-created.js",
//!           "globalName": "YTD.lists_created.part0",
//!           "count": "0"
//!         }
//!       ]
//!     },
//!     "listsMember": {
//!       "files": [
//!         {
//!           "fileName": "data/lists-member.js",
//!           "globalName": "YTD.lists_member.part0",
//!           "count": "11"
//!         }
//!       ]
//!     },
//!     "listsSubscribed": {
//!       "files": [
//!         {
//!           "fileName": "data/lists-subscribed.js",
//!           "globalName": "YTD.lists_subscribed.part0",
//!           "count": "0"
//!         }
//!       ]
//!     },
//!     "moment": {
//!       "mediaDirectory": "data/moments_media",
//!       "files": [
//!         {
//!           "fileName": "data/moment.js",
//!           "globalName": "YTD.moment.part0",
//!           "count": "0"
//!         }
//!       ]
//!     },
//!     "momentsMedia": {
//!       "mediaDirectory": "data/moments_media"
//!     },
//!     "momentsTweetsMedia": {
//!       "mediaDirectory": "data/moments_tweets_media"
//!     },
//!     "mute": {
//!       "files": [
//!         {
//!           "fileName": "data/mute.js",
//!           "globalName": "YTD.mute.part0",
//!           "count": "4086"
//!         }
//!       ]
//!     },
//!     "niDevices": {
//!       "files": [
//!         {
//!           "fileName": "data/ni-devices.js",
//!           "globalName": "YTD.ni_devices.part0",
//!           "count": "1"
//!         }
//!       ]
//!     },
//!     "noteTweet": {
//!       "files": [
//!         {
//!           "fileName": "data/note-tweet.js",
//!           "globalName": "YTD.note_tweet.part0",
//!           "count": "0"
//!         }
//!       ]
//!     },
//!     "periscopeAccountInformation": {
//!       "files": [
//!         {
//!           "fileName": "data/periscope-account-information.js",
//!           "globalName": "YTD.periscope_account_information.part0",
//!           "count": "0"
//!         }
//!       ]
//!     },
//!     "periscopeBanInformation": {
//!       "files": [
//!         {
//!           "fileName": "data/periscope-ban-information.js",
//!           "globalName": "YTD.periscope_ban_information.part0",
//!           "count": "0"
//!         }
//!       ]
//!     },
//!     "periscopeBroadcastMetadata": {
//!       "files": [
//!         {
//!           "fileName": "data/periscope-broadcast-metadata.js",
//!           "globalName": "YTD.periscope_broadcast_metadata.part0",
//!           "count": "0"
//!         }
//!       ]
//!     },
//!     "periscopeCommentsMadeByUser": {
//!       "files": [
//!         {
//!           "fileName": "data/periscope-comments-made-by-user.js",
//!           "globalName": "YTD.periscope_comments_made_by_user.part0",
//!           "count": "0"
//!         }
//!       ]
//!     },
//!     "periscopeExpiredBroadcasts": {
//!       "files": [
//!         {
//!           "fileName": "data/periscope-expired-broadcasts.js",
//!           "globalName": "YTD.periscope_expired_broadcasts.part0",
//!           "count": "0"
//!         }
//!       ]
//!     },
//!     "periscopeFollowers": {
//!       "files": [
//!         {
//!           "fileName": "data/periscope-followers.js",
//!           "globalName": "YTD.periscope_followers.part0",
//!           "count": "0"
//!         }
//!       ]
//!     },
//!     "periscopeProfileDescription": {
//!       "files": [
//!         {
//!           "fileName": "data/periscope-profile-description.js",
//!           "globalName": "YTD.periscope_profile_description.part0",
//!           "count": "0"
//!         }
//!       ]
//!     },
//!     "personalization": {
//!       "files": [
//!         {
//!           "fileName": "data/personalization.js",
//!           "globalName": "YTD.personalization.part0",
//!           "count": "1"
//!         }
//!       ]
//!     },
//!     "phoneNumber": {
//!       "files": [
//!         {
//!           "fileName": "data/phone-number.js",
//!           "globalName": "YTD.phone_number.part0",
//!           "count": "1"
//!         }
//!       ]
//!     },
//!     "productDrop": {
//!       "files": [
//!         {
//!           "fileName": "data/product-drop.js",
//!           "globalName": "YTD.product_drop.part0",
//!           "count": "0"
//!         }
//!       ]
//!     },
//!     "productSet": {
//!       "files": [
//!         {
//!           "fileName": "data/product-set.js",
//!           "globalName": "YTD.product_set.part0",
//!           "count": "0"
//!         }
//!       ]
//!     },
//!     "professionalData": {
//!       "files": [
//!         {
//!           "fileName": "data/professional-data.js",
//!           "globalName": "YTD.professional_data.part0",
//!           "count": "0"
//!         }
//!       ]
//!     },
//!     "profile": {
//!       "mediaDirectory": "data/profile_media",
//!       "files": [
//!         {
//!           "fileName": "data/profile.js",
//!           "globalName": "YTD.profile.part0",
//!           "count": "1"
//!         }
//!       ]
//!     },
//!     "profileMedia": {
//!       "mediaDirectory": "data/profile_media"
//!     },
//!     "protectedHistory": {
//!       "files": [
//!         {
//!           "fileName": "data/protected-history.js",
//!           "globalName": "YTD.protected_history.part0",
//!           "count": "0"
//!         }
//!       ]
//!     },
//!     "replyPrompt": {
//!       "files": [
//!         {
//!           "fileName": "data/reply-prompt.js",
//!           "globalName": "YTD.reply_prompt.part0",
//!           "count": "0"
//!         }
//!       ]
//!     },
//!     "savedSearch": {
//!       "files": [
//!         {
//!           "fileName": "data/saved-search.js",
//!           "globalName": "YTD.saved_search.part0",
//!           "count": "0"
//!         }
//!       ]
//!     },
//!     "screenNameChange": {
//!       "files": [
//!         {
//!           "fileName": "data/screen-name-change.js",
//!           "globalName": "YTD.screen_name_change.part0",
//!           "count": "1"
//!         }
//!       ]
//!     },
//!     "shopModule": {
//!       "files": [
//!         {
//!           "fileName": "data/shop-module.js",
//!           "globalName": "YTD.shop_module.part0",
//!           "count": "0"
//!         }
//!       ]
//!     },
//!     "shopifyAccount": {
//!       "files": [
//!         {
//!           "fileName": "data/shopify-account.js",
//!           "globalName": "YTD.shopify_account.part0",
//!           "count": "0"
//!         }
//!       ]
//!     },
//!     "smartblock": {
//!       "files": [
//!         {
//!           "fileName": "data/smartblock.js",
//!           "globalName": "YTD.smartblock.part0",
//!           "count": "0"
//!         }
//!       ]
//!     },
//!     "spacesMetadata": {
//!       "files": [
//!         {
//!           "fileName": "data/spaces-metadata.js",
//!           "globalName": "YTD.spaces_metadata.part0",
//!           "count": "0"
//!         }
//!       ]
//!     },
//!     "sso": {
//!       "files": [
//!         {
//!           "fileName": "data/sso.js",
//!           "globalName": "YTD.sso.part0",
//!           "count": "0"
//!         }
//!       ]
//!     },
//!     "tweetHeaders": {
//!       "files": [
//!         {
//!           "fileName": "data/tweet-headers.js",
//!           "globalName": "YTD.tweet_headers.part0",
//!           "count": "3601"
//!         }
//!       ]
//!     },
//!     "tweetdeck": {
//!       "files": [
//!         {
//!           "fileName": "data/tweetdeck.js",
//!           "globalName": "YTD.tweetdeck.part0",
//!           "count": "2"
//!         }
//!       ]
//!     },
//!     "tweets": {
//!       "mediaDirectory": "data/tweets_media",
//!       "files": [
//!         {
//!           "fileName": "data/tweets.js",
//!           "globalName": "YTD.tweets.part0",
//!           "count": "3601"
//!         }
//!       ]
//!     },
//!     "tweetsMedia": {
//!       "mediaDirectory": "data/tweets_media"
//!     },
//!     "twitterArticle": {
//!       "mediaDirectory": "data/twitter_article_media",
//!       "files": [
//!         {
//!           "fileName": "data/twitter-article.js",
//!           "globalName": "YTD.twitter_article.part0",
//!           "count": "0"
//!         }
//!       ]
//!     },
//!     "twitterArticleMedia": {
//!       "mediaDirectory": "data/twitter_article_media"
//!     },
//!     "twitterArticleMetadata": {
//!       "files": [
//!         {
//!           "fileName": "data/twitter-article-metadata.js",
//!           "globalName": "YTD.twitter_article_metadata.part0",
//!           "count": "0"
//!         }
//!       ]
//!     },
//!     "twitterCircle": {
//!       "mediaDirectory": "data/twitter_circle_tweet_media",
//!       "files": [
//!         {
//!           "fileName": "data/twitter-circle.js",
//!           "globalName": "YTD.twitter_circle.part0",
//!           "count": "1"
//!         }
//!       ]
//!     },
//!     "twitterCircleMember": {
//!       "files": [
//!         {
//!           "fileName": "data/twitter-circle-member.js",
//!           "globalName": "YTD.twitter_circle_member.part0",
//!           "count": "0"
//!         }
//!       ]
//!     },
//!     "twitterCircleTweet": {
//!       "mediaDirectory": "data/twitter_circle_tweet_media",
//!       "files": [
//!         {
//!           "fileName": "data/twitter-circle-tweet.js",
//!           "globalName": "YTD.twitter_circle_tweet.part0",
//!           "count": "0"
//!         }
//!       ]
//!     },
//!     "twitterCircleTweetMedia": {
//!       "mediaDirectory": "data/twitter_circle_tweet_media"
//!     },
//!     "twitterShop": {
//!       "files": [
//!         {
//!           "fileName": "data/twitter-shop.js",
//!           "globalName": "YTD.twitter_shop.part0",
//!           "count": "0"
//!         }
//!       ]
//!     },
//!     "userLinkClicks": {
//!       "files": [
//!         {
//!           "fileName": "data/user-link-clicks.js",
//!           "globalName": "YTD.user_link_clicks.part0",
//!           "count": "0"
//!         }
//!       ]
//!     },
//!     "verified": {
//!       "files": [
//!         {
//!           "fileName": "data/verified.js",
//!           "globalName": "YTD.verified.part0",
//!           "count": "1"
//!         }
//!       ]
//!     }
//!   }
//! ]
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
/// use twitter_archive::convert::date_time_iso_8601::FORMAT;
///
/// use twitter_archive::structs::manifest::Manifest;
///
/// let generation_date_string = "2023-08-30T23:20:03.000Z";
/// let generation_date_native_time = NaiveDateTime::parse_from_str(&generation_date_string, FORMAT).unwrap();
/// let generation_date_date_time = DateTime::<Utc>::from_naive_utc_and_offset(generation_date_native_time, Utc);
///
/// let json = format!(r#"{{
///   "userInfo": {{
///     "accountId": "111111111",
///     "userName": "S0_And_S0",
///     "displayName": "S0AndS0.eth"
///   }},
///   "archiveInfo": {{
///     "sizeBytes": "44546997",
///     "generationDate": "{generation_date_string}",
///     "isPartialArchive": false,
///     "maxPartSizeBytes": "53687091200"
///   }},
///   "readmeInfo": {{
///     "fileName": "data/README.txt",
///     "directory": "data/",
///     "name": "README.txt"
///   }},
///   "dataTypes": {{
///     "account": {{
///       "files": [
///         {{
///           "fileName": "data/account.js",
///           "globalName": "YTD.account.part0",
///           "count": "1"
///         }}
///       ]
///     }},
///     "accountCreationIp": {{
///       "files": [
///         {{
///           "fileName": "data/account-creation-ip.js",
///           "globalName": "YTD.account_creation_ip.part0",
///           "count": "1"
///         }}
///       ]
///     }},
///     "accountLabel": {{
///       "files": [
///         {{
///           "fileName": "data/account-label.js",
///           "globalName": "YTD.account_label.part0",
///           "count": "1"
///         }}
///       ]
///     }},
///     "accountSuspension": {{
///       "files": [
///         {{
///           "fileName": "data/account-suspension.js",
///           "globalName": "YTD.account_suspension.part0",
///           "count": "0"
///         }}
///       ]
///     }},
///     "accountTimezone": {{
///       "files": [
///         {{
///           "fileName": "data/account-timezone.js",
///           "globalName": "YTD.account_timezone.part0",
///           "count": "1"
///         }}
///       ]
///     }},
///     "adEngagements": {{
///       "files": [
///         {{
///           "fileName": "data/ad-engagements.js",
///           "globalName": "YTD.ad_engagements.part0",
///           "count": "122"
///         }}
///       ]
///     }},
///     "adImpressions": {{
///       "files": [
///         {{
///           "fileName": "data/ad-impressions.js",
///           "globalName": "YTD.ad_impressions.part0",
///           "count": "125"
///         }}
///       ]
///     }},
///     "adMobileConversionsAttributed": {{
///       "files": [
///         {{
///           "fileName": "data/ad-mobile-conversions-attributed.js",
///           "globalName": "YTD.ad_mobile_conversions_attributed.part0",
///           "count": "0"
///         }}
///       ]
///     }},
///     "adMobileConversionsUnattributed": {{
///       "files": [
///         {{
///           "fileName": "data/ad-mobile-conversions-unattributed.js",
///           "globalName": "YTD.ad_mobile_conversions_unattributed.part0",
///           "count": "0"
///         }}
///       ]
///     }},
///     "adOnlineConversionsAttributed": {{
///       "files": [
///         {{
///           "fileName": "data/ad-online-conversions-attributed.js",
///           "globalName": "YTD.ad_online_conversions_attributed.part0",
///           "count": "0"
///         }}
///       ]
///     }},
///     "adOnlineConversionsUnattributed": {{
///       "files": [
///         {{
///           "fileName": "data/ad-online-conversions-unattributed.js",
///           "globalName": "YTD.ad_online_conversions_unattributed.part0",
///           "count": "0"
///         }}
///       ]
///     }},
///     "ageinfo": {{
///       "files": [
///         {{
///           "fileName": "data/ageinfo.js",
///           "globalName": "YTD.ageinfo.part0",
///           "count": "1"
///         }}
///       ]
///     }},
///     "app": {{
///       "files": [
///         {{
///           "fileName": "data/app.js",
///           "globalName": "YTD.app.part0",
///           "count": "0"
///         }}
///       ]
///     }},
///     "block": {{
///       "files": [
///         {{
///           "fileName": "data/block.js",
///           "globalName": "YTD.block.part0",
///           "count": "10"
///         }}
///       ]
///     }},
///     "branchLinks": {{
///       "files": [
///         {{
///           "fileName": "data/branch-links.js",
///           "globalName": "YTD.branch_links.part0",
///           "count": "0"
///         }}
///       ]
///     }},
///     "catalogItem": {{
///       "files": [
///         {{
///           "fileName": "data/catalog-item.js",
///           "globalName": "YTD.catalog_item.part0",
///           "count": "0"
///         }}
///       ]
///     }},
///     "commerceCatalog": {{
///       "files": [
///         {{
///           "fileName": "data/commerce-catalog.js",
///           "globalName": "YTD.commerce_catalog.part0",
///           "count": "0"
///         }}
///       ]
///     }},
///     "communityNote": {{
///       "files": [
///         {{
///           "fileName": "data/community-note.js",
///           "globalName": "YTD.community_note.part0",
///           "count": "0"
///         }}
///       ]
///     }},
///     "communityNoteRating": {{
///       "files": [
///         {{
///           "fileName": "data/community-note-rating.js",
///           "globalName": "YTD.community_note_rating.part0",
///           "count": "1"
///         }}
///       ]
///     }},
///     "communityNoteTombstone": {{
///       "files": [
///         {{
///           "fileName": "data/community-note-tombstone.js",
///           "globalName": "YTD.community_note_tombstone.part0",
///           "count": "0"
///         }}
///       ]
///     }},
///     "communityTweet": {{
///       "mediaDirectory": "data/community_tweet_media",
///       "files": [
///         {{
///           "fileName": "data/community-tweet.js",
///           "globalName": "YTD.community_tweet.part0",
///           "count": "0"
///         }}
///       ]
///     }},
///     "communityTweetMedia": {{
///       "mediaDirectory": "data/community_tweet_media"
///     }},
///     "connectedApplication": {{
///       "files": [
///         {{
///           "fileName": "data/connected-application.js",
///           "globalName": "YTD.connected_application.part0",
///           "count": "1"
///         }}
///       ]
///     }},
///     "contact": {{
///       "files": [
///         {{
///           "fileName": "data/contact.js",
///           "globalName": "YTD.contact.part0",
///           "count": "0"
///         }}
///       ]
///     }},
///     "deletedNoteTweet": {{
///       "files": [
///         {{
///           "fileName": "data/deleted-note-tweet.js",
///           "globalName": "YTD.deleted_note_tweet.part0",
///           "count": "0"
///         }}
///       ]
///     }},
///     "deletedTweetHeaders": {{
///       "files": [
///         {{
///           "fileName": "data/deleted-tweet-headers.js",
///           "globalName": "YTD.deleted_tweet_headers.part0",
///           "count": "3"
///         }}
///       ]
///     }},
///     "deletedTweets": {{
///       "mediaDirectory": "data/deleted_tweets_media",
///       "files": [
///         {{
///           "fileName": "data/deleted-tweets.js",
///           "globalName": "YTD.deleted_tweets.part0",
///           "count": "3"
///         }}
///       ]
///     }},
///     "deletedTweetsMedia": {{
///       "mediaDirectory": "data/deleted_tweets_media"
///     }},
///     "deviceToken": {{
///       "files": [
///         {{
///           "fileName": "data/device-token.js",
///           "globalName": "YTD.device_token.part0",
///           "count": "600"
///         }}
///       ]
///     }},
///     "directMessageGroupHeaders": {{
///       "files": [
///         {{
///           "fileName": "data/direct-message-group-headers.js",
///           "globalName": "YTD.direct_message_group_headers.part0",
///           "count": "2"
///         }}
///       ]
///     }},
///     "directMessageHeaders": {{
///       "files": [
///         {{
///           "fileName": "data/direct-message-headers.js",
///           "globalName": "YTD.direct_message_headers.part0",
///           "count": "84"
///         }}
///       ]
///     }},
///     "directMessageMute": {{
///       "files": [
///         {{
///           "fileName": "data/direct-message-mute.js",
///           "globalName": "YTD.direct_message_mute.part0",
///           "count": "0"
///         }}
///       ]
///     }},
///     "directMessages": {{
///       "mediaDirectory": "data/direct_messages_media",
///       "files": [
///         {{
///           "fileName": "data/direct-messages.js",
///           "globalName": "YTD.direct_messages.part0",
///           "count": "84"
///         }}
///       ]
///     }},
///     "directMessagesGroup": {{
///       "mediaDirectory": "data/direct_messages_group_media",
///       "files": [
///         {{
///           "fileName": "data/direct-messages-group.js",
///           "globalName": "YTD.direct_messages_group.part0",
///           "count": "2"
///         }}
///       ]
///     }},
///     "directMessagesGroupMedia": {{
///       "mediaDirectory": "data/direct_messages_group_media"
///     }},
///     "directMessagesMedia": {{
///       "mediaDirectory": "data/direct_messages_media"
///     }},
///     "emailAddressChange": {{
///       "files": [
///         {{
///           "fileName": "data/email-address-change.js",
///           "globalName": "YTD.email_address_change.part0",
///           "count": "2"
///         }}
///       ]
///     }},
///     "follower": {{
///       "files": [
///         {{
///           "fileName": "data/follower.js",
///           "globalName": "YTD.follower.part0",
///           "count": "289"
///         }}
///       ]
///     }},
///     "following": {{
///       "files": [
///         {{
///           "fileName": "data/following.js",
///           "globalName": "YTD.following.part0",
///           "count": "283"
///         }}
///       ]
///     }},
///     "ipAudit": {{
///       "files": [
///         {{
///           "fileName": "data/ip-audit.js",
///           "globalName": "YTD.ip_audit.part0",
///           "count": "542"
///         }}
///       ]
///     }},
///     "keyRegistry": {{
///       "files": [
///         {{
///           "fileName": "data/key-registry.js",
///           "globalName": "YTD.key_registry.part0",
///           "count": "1"
///         }}
///       ]
///     }},
///     "like": {{
///       "files": [
///         {{
///           "fileName": "data/like.js",
///           "globalName": "YTD.like.part0",
///           "count": "25548"
///         }}
///       ]
///     }},
///     "listsCreated": {{
///       "files": [
///         {{
///           "fileName": "data/lists-created.js",
///           "globalName": "YTD.lists_created.part0",
///           "count": "0"
///         }}
///       ]
///     }},
///     "listsMember": {{
///       "files": [
///         {{
///           "fileName": "data/lists-member.js",
///           "globalName": "YTD.lists_member.part0",
///           "count": "11"
///         }}
///       ]
///     }},
///     "listsSubscribed": {{
///       "files": [
///         {{
///           "fileName": "data/lists-subscribed.js",
///           "globalName": "YTD.lists_subscribed.part0",
///           "count": "0"
///         }}
///       ]
///     }},
///     "moment": {{
///       "mediaDirectory": "data/moments_media",
///       "files": [
///         {{
///           "fileName": "data/moment.js",
///           "globalName": "YTD.moment.part0",
///           "count": "0"
///         }}
///       ]
///     }},
///     "momentsMedia": {{
///       "mediaDirectory": "data/moments_media"
///     }},
///     "momentsTweetsMedia": {{
///       "mediaDirectory": "data/moments_tweets_media"
///     }},
///     "mute": {{
///       "files": [
///         {{
///           "fileName": "data/mute.js",
///           "globalName": "YTD.mute.part0",
///           "count": "4086"
///         }}
///       ]
///     }},
///     "niDevices": {{
///       "files": [
///         {{
///           "fileName": "data/ni-devices.js",
///           "globalName": "YTD.ni_devices.part0",
///           "count": "1"
///         }}
///       ]
///     }},
///     "noteTweet": {{
///       "files": [
///         {{
///           "fileName": "data/note-tweet.js",
///           "globalName": "YTD.note_tweet.part0",
///           "count": "0"
///         }}
///       ]
///     }},
///     "periscopeAccountInformation": {{
///       "files": [
///         {{
///           "fileName": "data/periscope-account-information.js",
///           "globalName": "YTD.periscope_account_information.part0",
///           "count": "0"
///         }}
///       ]
///     }},
///     "periscopeBanInformation": {{
///       "files": [
///         {{
///           "fileName": "data/periscope-ban-information.js",
///           "globalName": "YTD.periscope_ban_information.part0",
///           "count": "0"
///         }}
///       ]
///     }},
///     "periscopeBroadcastMetadata": {{
///       "files": [
///         {{
///           "fileName": "data/periscope-broadcast-metadata.js",
///           "globalName": "YTD.periscope_broadcast_metadata.part0",
///           "count": "0"
///         }}
///       ]
///     }},
///     "periscopeCommentsMadeByUser": {{
///       "files": [
///         {{
///           "fileName": "data/periscope-comments-made-by-user.js",
///           "globalName": "YTD.periscope_comments_made_by_user.part0",
///           "count": "0"
///         }}
///       ]
///     }},
///     "periscopeExpiredBroadcasts": {{
///       "files": [
///         {{
///           "fileName": "data/periscope-expired-broadcasts.js",
///           "globalName": "YTD.periscope_expired_broadcasts.part0",
///           "count": "0"
///         }}
///       ]
///     }},
///     "periscopeFollowers": {{
///       "files": [
///         {{
///           "fileName": "data/periscope-followers.js",
///           "globalName": "YTD.periscope_followers.part0",
///           "count": "0"
///         }}
///       ]
///     }},
///     "periscopeProfileDescription": {{
///       "files": [
///         {{
///           "fileName": "data/periscope-profile-description.js",
///           "globalName": "YTD.periscope_profile_description.part0",
///           "count": "0"
///         }}
///       ]
///     }},
///     "personalization": {{
///       "files": [
///         {{
///           "fileName": "data/personalization.js",
///           "globalName": "YTD.personalization.part0",
///           "count": "1"
///         }}
///       ]
///     }},
///     "phoneNumber": {{
///       "files": [
///         {{
///           "fileName": "data/phone-number.js",
///           "globalName": "YTD.phone_number.part0",
///           "count": "1"
///         }}
///       ]
///     }},
///     "productDrop": {{
///       "files": [
///         {{
///           "fileName": "data/product-drop.js",
///           "globalName": "YTD.product_drop.part0",
///           "count": "0"
///         }}
///       ]
///     }},
///     "productSet": {{
///       "files": [
///         {{
///           "fileName": "data/product-set.js",
///           "globalName": "YTD.product_set.part0",
///           "count": "0"
///         }}
///       ]
///     }},
///     "professionalData": {{
///       "files": [
///         {{
///           "fileName": "data/professional-data.js",
///           "globalName": "YTD.professional_data.part0",
///           "count": "0"
///         }}
///       ]
///     }},
///     "profile": {{
///       "mediaDirectory": "data/profile_media",
///       "files": [
///         {{
///           "fileName": "data/profile.js",
///           "globalName": "YTD.profile.part0",
///           "count": "1"
///         }}
///       ]
///     }},
///     "profileMedia": {{
///       "mediaDirectory": "data/profile_media"
///     }},
///     "protectedHistory": {{
///       "files": [
///         {{
///           "fileName": "data/protected-history.js",
///           "globalName": "YTD.protected_history.part0",
///           "count": "0"
///         }}
///       ]
///     }},
///     "replyPrompt": {{
///       "files": [
///         {{
///           "fileName": "data/reply-prompt.js",
///           "globalName": "YTD.reply_prompt.part0",
///           "count": "0"
///         }}
///       ]
///     }},
///     "savedSearch": {{
///       "files": [
///         {{
///           "fileName": "data/saved-search.js",
///           "globalName": "YTD.saved_search.part0",
///           "count": "0"
///         }}
///       ]
///     }},
///     "screenNameChange": {{
///       "files": [
///         {{
///           "fileName": "data/screen-name-change.js",
///           "globalName": "YTD.screen_name_change.part0",
///           "count": "1"
///         }}
///       ]
///     }},
///     "shopModule": {{
///       "files": [
///         {{
///           "fileName": "data/shop-module.js",
///           "globalName": "YTD.shop_module.part0",
///           "count": "0"
///         }}
///       ]
///     }},
///     "shopifyAccount": {{
///       "files": [
///         {{
///           "fileName": "data/shopify-account.js",
///           "globalName": "YTD.shopify_account.part0",
///           "count": "0"
///         }}
///       ]
///     }},
///     "smartblock": {{
///       "files": [
///         {{
///           "fileName": "data/smartblock.js",
///           "globalName": "YTD.smartblock.part0",
///           "count": "0"
///         }}
///       ]
///     }},
///     "spacesMetadata": {{
///       "files": [
///         {{
///           "fileName": "data/spaces-metadata.js",
///           "globalName": "YTD.spaces_metadata.part0",
///           "count": "0"
///         }}
///       ]
///     }},
///     "sso": {{
///       "files": [
///         {{
///           "fileName": "data/sso.js",
///           "globalName": "YTD.sso.part0",
///           "count": "0"
///         }}
///       ]
///     }},
///     "tweetHeaders": {{
///       "files": [
///         {{
///           "fileName": "data/tweet-headers.js",
///           "globalName": "YTD.tweet_headers.part0",
///           "count": "3601"
///         }}
///       ]
///     }},
///     "tweetdeck": {{
///       "files": [
///         {{
///           "fileName": "data/tweetdeck.js",
///           "globalName": "YTD.tweetdeck.part0",
///           "count": "2"
///         }}
///       ]
///     }},
///     "tweets": {{
///       "mediaDirectory": "data/tweets_media",
///       "files": [
///         {{
///           "fileName": "data/tweets.js",
///           "globalName": "YTD.tweets.part0",
///           "count": "3601"
///         }}
///       ]
///     }},
///     "tweetsMedia": {{
///       "mediaDirectory": "data/tweets_media"
///     }},
///     "twitterArticle": {{
///       "mediaDirectory": "data/twitter_article_media",
///       "files": [
///         {{
///           "fileName": "data/twitter-article.js",
///           "globalName": "YTD.twitter_article.part0",
///           "count": "0"
///         }}
///       ]
///     }},
///     "twitterArticleMedia": {{
///       "mediaDirectory": "data/twitter_article_media"
///     }},
///     "twitterArticleMetadata": {{
///       "files": [
///         {{
///           "fileName": "data/twitter-article-metadata.js",
///           "globalName": "YTD.twitter_article_metadata.part0",
///           "count": "0"
///         }}
///       ]
///     }},
///     "twitterCircle": {{
///       "mediaDirectory": "data/twitter_circle_tweet_media",
///       "files": [
///         {{
///           "fileName": "data/twitter-circle.js",
///           "globalName": "YTD.twitter_circle.part0",
///           "count": "1"
///         }}
///       ]
///     }},
///     "twitterCircleMember": {{
///       "files": [
///         {{
///           "fileName": "data/twitter-circle-member.js",
///           "globalName": "YTD.twitter_circle_member.part0",
///           "count": "0"
///         }}
///       ]
///     }},
///     "twitterCircleTweet": {{
///       "mediaDirectory": "data/twitter_circle_tweet_media",
///       "files": [
///         {{
///           "fileName": "data/twitter-circle-tweet.js",
///           "globalName": "YTD.twitter_circle_tweet.part0",
///           "count": "0"
///         }}
///       ]
///     }},
///     "twitterCircleTweetMedia": {{
///       "mediaDirectory": "data/twitter_circle_tweet_media"
///     }},
///     "twitterShop": {{
///       "files": [
///         {{
///           "fileName": "data/twitter-shop.js",
///           "globalName": "YTD.twitter_shop.part0",
///           "count": "0"
///         }}
///       ]
///     }},
///     "userLinkClicks": {{
///       "files": [
///         {{
///           "fileName": "data/user-link-clicks.js",
///           "globalName": "YTD.user_link_clicks.part0",
///           "count": "0"
///         }}
///       ]
///     }},
///     "verified": {{
///       "files": [
///         {{
///           "fileName": "data/verified.js",
///           "globalName": "YTD.verified.part0",
///           "count": "1"
///         }}
///       ]
///     }}
///   }}
/// }}"#);
///
/// let data: Manifest = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.user_info.account_id, "111111111");
/// assert_eq!(data.user_info.user_name, "S0_And_S0");
/// assert_eq!(data.user_info.display_name, "S0AndS0.eth");
///
/// assert_eq!(data.archive_info.size_bytes, 44546997);
/// assert_eq!(data.archive_info.generation_date, generation_date_date_time);
/// assert_eq!(data.archive_info.is_partial_archive, false);
/// assert_eq!(data.archive_info.max_part_size_bytes, 53687091200);
///
/// assert_eq!(data.readme_info.file_name, "data/README.txt");
/// assert_eq!(data.readme_info.directory, "data/");
/// assert_eq!(data.readme_info.name, "README.txt");
///
/// assert_eq!(data.data_types.twitter_circle_tweet.media_directory, "data/twitter_circle_tweet_media");
///
/// assert_eq!(data.data_types.twitter_circle_tweet.files.len(), 1);
/// assert_eq!(data.data_types.twitter_circle_tweet.files[0].file_name, "data/twitter-circle-tweet.js");
/// assert_eq!(data.data_types.twitter_circle_tweet.files[0].global_name, "YTD.twitter_circle_tweet.part0");
/// assert_eq!(data.data_types.twitter_circle_tweet.files[0].count, 0);
///
/// assert_eq!(data.data_types.verified.files.len(), 1);
/// assert_eq!(data.data_types.verified.files[0].file_name, "data/verified.js");
/// assert_eq!(data.data_types.verified.files[0].global_name, "YTD.verified.part0");
/// assert_eq!(data.data_types.verified.files[0].count, 1);
///
/// // Re-serialize is equivalent to original data without pretty printing
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct Manifest {
	/// Data about Twitter user; `account_id`, `user_name`, and `display_name`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "accountId": "111111111",
	///   "userName": "S0_And_S0",
	///   "displayName": "S0AndS0.eth"
	/// }
	/// ```
	pub user_info: UserInfo,

	/// Metadata about archive; `size_bytes`, `generation_date`, `is_partial_archive`, and
	/// `max_part_size_bytes`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "sizeBytes": "44546997",
	///   "generationDate": "2023-08-30T23:20:03.000Z",
	///   "isPartialArchive": false,
	///   "maxPartSizeBytes": "53687091200"
	/// }
	/// ```
	pub archive_info: ArchiveInfo,

	/// Metadata about archive `README.txt` file; `file_name`, `directory`, and `name`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "fileName": "data/README.txt",
	///   "directory": "data/",
	///   "name": "README.txt"
	/// }
	/// ```
	pub readme_info: ReadmeInfo,

	/// Metadata that points to various files within archive and hints on how to possibly parse
	/// JavaScript into JSON
	///
	/// ## Example JSON data
	/// ```json
	/// {
	///   "account": {
	///     "files": [
	///       {
	///         "fileName": "data/account.js",
	///         "globalName": "YTD.account.part0",
	///         "count": "1"
	///       }
	///     ]
	///   },
	///   "accountCreationIp": {
	///     "files": [
	///       {
	///         "fileName": "data/account-creation-ip.js",
	///         "globalName": "YTD.account_creation_ip.part0",
	///         "count": "1"
	///       }
	///     ]
	///   },
	///   "accountLabel": {
	///     "files": [
	///       {
	///         "fileName": "data/account-label.js",
	///         "globalName": "YTD.account_label.part0",
	///         "count": "1"
	///       }
	///     ]
	///   },
	///   "accountSuspension": {
	///     "files": [
	///       {
	///         "fileName": "data/account-suspension.js",
	///         "globalName": "YTD.account_suspension.part0",
	///         "count": "0"
	///       }
	///     ]
	///   },
	///   "accountTimezone": {
	///     "files": [
	///       {
	///         "fileName": "data/account-timezone.js",
	///         "globalName": "YTD.account_timezone.part0",
	///         "count": "1"
	///       }
	///     ]
	///   },
	///   "adEngagements": {
	///     "files": [
	///       {
	///         "fileName": "data/ad-engagements.js",
	///         "globalName": "YTD.ad_engagements.part0",
	///         "count": "122"
	///       }
	///     ]
	///   },
	///   "adImpressions": {
	///     "files": [
	///       {
	///         "fileName": "data/ad-impressions.js",
	///         "globalName": "YTD.ad_impressions.part0",
	///         "count": "125"
	///       }
	///     ]
	///   },
	///   "adMobileConversionsAttributed": {
	///     "files": [
	///       {
	///         "fileName": "data/ad-mobile-conversions-attributed.js",
	///         "globalName": "YTD.ad_mobile_conversions_attributed.part0",
	///         "count": "0"
	///       }
	///     ]
	///   },
	///   "adMobileConversionsUnattributed": {
	///     "files": [
	///       {
	///         "fileName": "data/ad-mobile-conversions-unattributed.js",
	///         "globalName": "YTD.ad_mobile_conversions_unattributed.part0",
	///         "count": "0"
	///       }
	///     ]
	///   },
	///   "adOnlineConversionsAttributed": {
	///     "files": [
	///       {
	///         "fileName": "data/ad-online-conversions-attributed.js",
	///         "globalName": "YTD.ad_online_conversions_attributed.part0",
	///         "count": "0"
	///       }
	///     ]
	///   },
	///   "adOnlineConversionsUnattributed": {
	///     "files": [
	///       {
	///         "fileName": "data/ad-online-conversions-unattributed.js",
	///         "globalName": "YTD.ad_online_conversions_unattributed.part0",
	///         "count": "0"
	///       }
	///     ]
	///   },
	///   "ageinfo": {
	///     "files": [
	///       {
	///         "fileName": "data/ageinfo.js",
	///         "globalName": "YTD.ageinfo.part0",
	///         "count": "1"
	///       }
	///     ]
	///   },
	///   "app": {
	///     "files": [
	///       {
	///         "fileName": "data/app.js",
	///         "globalName": "YTD.app.part0",
	///         "count": "0"
	///       }
	///     ]
	///   },
	///   "block": {
	///     "files": [
	///       {
	///         "fileName": "data/block.js",
	///         "globalName": "YTD.block.part0",
	///         "count": "10"
	///       }
	///     ]
	///   },
	///   "branchLinks": {
	///     "files": [
	///       {
	///         "fileName": "data/branch-links.js",
	///         "globalName": "YTD.branch_links.part0",
	///         "count": "0"
	///       }
	///     ]
	///   },
	///   "catalogItem": {
	///     "files": [
	///       {
	///         "fileName": "data/catalog-item.js",
	///         "globalName": "YTD.catalog_item.part0",
	///         "count": "0"
	///       }
	///     ]
	///   },
	///   "commerceCatalog": {
	///     "files": [
	///       {
	///         "fileName": "data/commerce-catalog.js",
	///         "globalName": "YTD.commerce_catalog.part0",
	///         "count": "0"
	///       }
	///     ]
	///   },
	///   "communityNote": {
	///     "files": [
	///       {
	///         "fileName": "data/community-note.js",
	///         "globalName": "YTD.community_note.part0",
	///         "count": "0"
	///       }
	///     ]
	///   },
	///   "communityNoteRating": {
	///     "files": [
	///       {
	///         "fileName": "data/community-note-rating.js",
	///         "globalName": "YTD.community_note_rating.part0",
	///         "count": "1"
	///       }
	///     ]
	///   },
	///   "communityNoteTombstone": {
	///     "files": [
	///       {
	///         "fileName": "data/community-note-tombstone.js",
	///         "globalName": "YTD.community_note_tombstone.part0",
	///         "count": "0"
	///       }
	///     ]
	///   },
	///   "communityTweet": {
	///     "mediaDirectory": "data/community_tweet_media",
	///     "files": [
	///       {
	///         "fileName": "data/community-tweet.js",
	///         "globalName": "YTD.community_tweet.part0",
	///         "count": "0"
	///       }
	///     ]
	///   },
	///   "communityTweetMedia": {
	///     "mediaDirectory": "data/community_tweet_media"
	///   },
	///   "connectedApplication": {
	///     "files": [
	///       {
	///         "fileName": "data/connected-application.js",
	///         "globalName": "YTD.connected_application.part0",
	///         "count": "1"
	///       }
	///     ]
	///   },
	///   "contact": {
	///     "files": [
	///       {
	///         "fileName": "data/contact.js",
	///         "globalName": "YTD.contact.part0",
	///         "count": "0"
	///       }
	///     ]
	///   },
	///   "deletedNoteTweet": {
	///     "files": [
	///       {
	///         "fileName": "data/deleted-note-tweet.js",
	///         "globalName": "YTD.deleted_note_tweet.part0",
	///         "count": "0"
	///       }
	///     ]
	///   },
	///   "deletedTweetHeaders": {
	///     "files": [
	///       {
	///         "fileName": "data/deleted-tweet-headers.js",
	///         "globalName": "YTD.deleted_tweet_headers.part0",
	///         "count": "3"
	///       }
	///     ]
	///   },
	///   "deletedTweets": {
	///     "mediaDirectory": "data/deleted_tweets_media",
	///     "files": [
	///       {
	///         "fileName": "data/deleted-tweets.js",
	///         "globalName": "YTD.deleted_tweets.part0",
	///         "count": "3"
	///       }
	///     ]
	///   },
	///   "deletedTweetsMedia": {
	///     "mediaDirectory": "data/deleted_tweets_media"
	///   },
	///   "deviceToken": {
	///     "files": [
	///       {
	///         "fileName": "data/device-token.js",
	///         "globalName": "YTD.device_token.part0",
	///         "count": "600"
	///       }
	///     ]
	///   },
	///   "directMessageGroupHeaders": {
	///     "files": [
	///       {
	///         "fileName": "data/direct-message-group-headers.js",
	///         "globalName": "YTD.direct_message_group_headers.part0",
	///         "count": "2"
	///       }
	///     ]
	///   },
	///   "directMessageHeaders": {
	///     "files": [
	///       {
	///         "fileName": "data/direct-message-headers.js",
	///         "globalName": "YTD.direct_message_headers.part0",
	///         "count": "84"
	///       }
	///     ]
	///   },
	///   "directMessageMute": {
	///     "files": [
	///       {
	///         "fileName": "data/direct-message-mute.js",
	///         "globalName": "YTD.direct_message_mute.part0",
	///         "count": "0"
	///       }
	///     ]
	///   },
	///   "directMessages": {
	///     "mediaDirectory": "data/direct_messages_media",
	///     "files": [
	///       {
	///         "fileName": "data/direct-messages.js",
	///         "globalName": "YTD.direct_messages.part0",
	///         "count": "84"
	///       }
	///     ]
	///   },
	///   "directMessagesGroup": {
	///     "mediaDirectory": "data/direct_messages_group_media",
	///     "files": [
	///       {
	///         "fileName": "data/direct-messages-group.js",
	///         "globalName": "YTD.direct_messages_group.part0",
	///         "count": "2"
	///       }
	///     ]
	///   },
	///   "directMessagesGroupMedia": {
	///     "mediaDirectory": "data/direct_messages_group_media"
	///   },
	///   "directMessagesMedia": {
	///     "mediaDirectory": "data/direct_messages_media"
	///   },
	///   "emailAddressChange": {
	///     "files": [
	///       {
	///         "fileName": "data/email-address-change.js",
	///         "globalName": "YTD.email_address_change.part0",
	///         "count": "2"
	///       }
	///     ]
	///   },
	///   "follower": {
	///     "files": [
	///       {
	///         "fileName": "data/follower.js",
	///         "globalName": "YTD.follower.part0",
	///         "count": "289"
	///       }
	///     ]
	///   },
	///   "following": {
	///     "files": [
	///       {
	///         "fileName": "data/following.js",
	///         "globalName": "YTD.following.part0",
	///         "count": "283"
	///       }
	///     ]
	///   },
	///   "ipAudit": {
	///     "files": [
	///       {
	///         "fileName": "data/ip-audit.js",
	///         "globalName": "YTD.ip_audit.part0",
	///         "count": "542"
	///       }
	///     ]
	///   },
	///   "keyRegistry": {
	///     "files": [
	///       {
	///         "fileName": "data/key-registry.js",
	///         "globalName": "YTD.key_registry.part0",
	///         "count": "1"
	///       }
	///     ]
	///   },
	///   "like": {
	///     "files": [
	///       {
	///         "fileName": "data/like.js",
	///         "globalName": "YTD.like.part0",
	///         "count": "25548"
	///       }
	///     ]
	///   },
	///   "listsCreated": {
	///     "files": [
	///       {
	///         "fileName": "data/lists-created.js",
	///         "globalName": "YTD.lists_created.part0",
	///         "count": "0"
	///       }
	///     ]
	///   },
	///   "listsMember": {
	///     "files": [
	///       {
	///         "fileName": "data/lists-member.js",
	///         "globalName": "YTD.lists_member.part0",
	///         "count": "11"
	///       }
	///     ]
	///   },
	///   "listsSubscribed": {
	///     "files": [
	///       {
	///         "fileName": "data/lists-subscribed.js",
	///         "globalName": "YTD.lists_subscribed.part0",
	///         "count": "0"
	///       }
	///     ]
	///   },
	///   "moment": {
	///     "mediaDirectory": "data/moments_media",
	///     "files": [
	///       {
	///         "fileName": "data/moment.js",
	///         "globalName": "YTD.moment.part0",
	///         "count": "0"
	///       }
	///     ]
	///   },
	///   "momentsMedia": {
	///     "mediaDirectory": "data/moments_media"
	///   },
	///   "momentsTweetsMedia": {
	///     "mediaDirectory": "data/moments_tweets_media"
	///   },
	///   "mute": {
	///     "files": [
	///       {
	///         "fileName": "data/mute.js",
	///         "globalName": "YTD.mute.part0",
	///         "count": "4086"
	///       }
	///     ]
	///   },
	///   "niDevices": {
	///     "files": [
	///       {
	///         "fileName": "data/ni-devices.js",
	///         "globalName": "YTD.ni_devices.part0",
	///         "count": "1"
	///       }
	///     ]
	///   },
	///   "noteTweet": {
	///     "files": [
	///       {
	///         "fileName": "data/note-tweet.js",
	///         "globalName": "YTD.note_tweet.part0",
	///         "count": "0"
	///       }
	///     ]
	///   },
	///   "periscopeAccountInformation": {
	///     "files": [
	///       {
	///         "fileName": "data/periscope-account-information.js",
	///         "globalName": "YTD.periscope_account_information.part0",
	///         "count": "0"
	///       }
	///     ]
	///   },
	///   "periscopeBanInformation": {
	///     "files": [
	///       {
	///         "fileName": "data/periscope-ban-information.js",
	///         "globalName": "YTD.periscope_ban_information.part0",
	///         "count": "0"
	///       }
	///     ]
	///   },
	///   "periscopeBroadcastMetadata": {
	///     "files": [
	///       {
	///         "fileName": "data/periscope-broadcast-metadata.js",
	///         "globalName": "YTD.periscope_broadcast_metadata.part0",
	///         "count": "0"
	///       }
	///     ]
	///   },
	///   "periscopeCommentsMadeByUser": {
	///     "files": [
	///       {
	///         "fileName": "data/periscope-comments-made-by-user.js",
	///         "globalName": "YTD.periscope_comments_made_by_user.part0",
	///         "count": "0"
	///       }
	///     ]
	///   },
	///   "periscopeExpiredBroadcasts": {
	///     "files": [
	///       {
	///         "fileName": "data/periscope-expired-broadcasts.js",
	///         "globalName": "YTD.periscope_expired_broadcasts.part0",
	///         "count": "0"
	///       }
	///     ]
	///   },
	///   "periscopeFollowers": {
	///     "files": [
	///       {
	///         "fileName": "data/periscope-followers.js",
	///         "globalName": "YTD.periscope_followers.part0",
	///         "count": "0"
	///       }
	///     ]
	///   },
	///   "periscopeProfileDescription": {
	///     "files": [
	///       {
	///         "fileName": "data/periscope-profile-description.js",
	///         "globalName": "YTD.periscope_profile_description.part0",
	///         "count": "0"
	///       }
	///     ]
	///   },
	///   "personalization": {
	///     "files": [
	///       {
	///         "fileName": "data/personalization.js",
	///         "globalName": "YTD.personalization.part0",
	///         "count": "1"
	///       }
	///     ]
	///   },
	///   "phoneNumber": {
	///     "files": [
	///       {
	///         "fileName": "data/phone-number.js",
	///         "globalName": "YTD.phone_number.part0",
	///         "count": "1"
	///       }
	///     ]
	///   },
	///   "productDrop": {
	///     "files": [
	///       {
	///         "fileName": "data/product-drop.js",
	///         "globalName": "YTD.product_drop.part0",
	///         "count": "0"
	///       }
	///     ]
	///   },
	///   "productSet": {
	///     "files": [
	///       {
	///         "fileName": "data/product-set.js",
	///         "globalName": "YTD.product_set.part0",
	///         "count": "0"
	///       }
	///     ]
	///   },
	///   "professionalData": {
	///     "files": [
	///       {
	///         "fileName": "data/professional-data.js",
	///         "globalName": "YTD.professional_data.part0",
	///         "count": "0"
	///       }
	///     ]
	///   },
	///   "profile": {
	///     "mediaDirectory": "data/profile_media",
	///     "files": [
	///       {
	///         "fileName": "data/profile.js",
	///         "globalName": "YTD.profile.part0",
	///         "count": "1"
	///       }
	///     ]
	///   },
	///   "profileMedia": {
	///     "mediaDirectory": "data/profile_media"
	///   },
	///   "protectedHistory": {
	///     "files": [
	///       {
	///         "fileName": "data/protected-history.js",
	///         "globalName": "YTD.protected_history.part0",
	///         "count": "0"
	///       }
	///     ]
	///   },
	///   "replyPrompt": {
	///     "files": [
	///       {
	///         "fileName": "data/reply-prompt.js",
	///         "globalName": "YTD.reply_prompt.part0",
	///         "count": "0"
	///       }
	///     ]
	///   },
	///   "savedSearch": {
	///     "files": [
	///       {
	///         "fileName": "data/saved-search.js",
	///         "globalName": "YTD.saved_search.part0",
	///         "count": "0"
	///       }
	///     ]
	///   },
	///   "screenNameChange": {
	///     "files": [
	///       {
	///         "fileName": "data/screen-name-change.js",
	///         "globalName": "YTD.screen_name_change.part0",
	///         "count": "1"
	///       }
	///     ]
	///   },
	///   "shopModule": {
	///     "files": [
	///       {
	///         "fileName": "data/shop-module.js",
	///         "globalName": "YTD.shop_module.part0",
	///         "count": "0"
	///       }
	///     ]
	///   },
	///   "shopifyAccount": {
	///     "files": [
	///       {
	///         "fileName": "data/shopify-account.js",
	///         "globalName": "YTD.shopify_account.part0",
	///         "count": "0"
	///       }
	///     ]
	///   },
	///   "smartblock": {
	///     "files": [
	///       {
	///         "fileName": "data/smartblock.js",
	///         "globalName": "YTD.smartblock.part0",
	///         "count": "0"
	///       }
	///     ]
	///   },
	///   "spacesMetadata": {
	///     "files": [
	///       {
	///         "fileName": "data/spaces-metadata.js",
	///         "globalName": "YTD.spaces_metadata.part0",
	///         "count": "0"
	///       }
	///     ]
	///   },
	///   "sso": {
	///     "files": [
	///       {
	///         "fileName": "data/sso.js",
	///         "globalName": "YTD.sso.part0",
	///         "count": "0"
	///       }
	///     ]
	///   },
	///   "tweetHeaders": {
	///     "files": [
	///       {
	///         "fileName": "data/tweet-headers.js",
	///         "globalName": "YTD.tweet_headers.part0",
	///         "count": "3601"
	///       }
	///     ]
	///   },
	///   "tweetdeck": {
	///     "files": [
	///       {
	///         "fileName": "data/tweetdeck.js",
	///         "globalName": "YTD.tweetdeck.part0",
	///         "count": "2"
	///       }
	///     ]
	///   },
	///   "tweets": {
	///     "mediaDirectory": "data/tweets_media",
	///     "files": [
	///       {
	///         "fileName": "data/tweets.js",
	///         "globalName": "YTD.tweets.part0",
	///         "count": "3601"
	///       }
	///     ]
	///   },
	///   "tweetsMedia": {
	///     "mediaDirectory": "data/tweets_media"
	///   },
	///   "twitterArticle": {
	///     "mediaDirectory": "data/twitter_article_media",
	///     "files": [
	///       {
	///         "fileName": "data/twitter-article.js",
	///         "globalName": "YTD.twitter_article.part0",
	///         "count": "0"
	///       }
	///     ]
	///   },
	///   "twitterArticleMedia": {
	///     "mediaDirectory": "data/twitter_article_media"
	///   },
	///   "twitterArticleMetadata": {
	///     "files": [
	///       {
	///         "fileName": "data/twitter-article-metadata.js",
	///         "globalName": "YTD.twitter_article_metadata.part0",
	///         "count": "0"
	///       }
	///     ]
	///   },
	///   "twitterCircle": {
	///     "mediaDirectory": "data/twitter_circle_tweet_media",
	///     "files": [
	///       {
	///         "fileName": "data/twitter-circle.js",
	///         "globalName": "YTD.twitter_circle.part0",
	///         "count": "1"
	///       }
	///     ]
	///   },
	///   "twitterCircleMember": {
	///     "files": [
	///       {
	///         "fileName": "data/twitter-circle-member.js",
	///         "globalName": "YTD.twitter_circle_member.part0",
	///         "count": "0"
	///       }
	///     ]
	///   },
	///   "twitterCircleTweet": {
	///     "mediaDirectory": "data/twitter_circle_tweet_media",
	///     "files": [
	///       {
	///         "fileName": "data/twitter-circle-tweet.js",
	///         "globalName": "YTD.twitter_circle_tweet.part0",
	///         "count": "0"
	///       }
	///     ]
	///   },
	///   "twitterCircleTweetMedia": {
	///     "mediaDirectory": "data/twitter_circle_tweet_media"
	///   },
	///   "twitterShop": {
	///     "files": [
	///       {
	///         "fileName": "data/twitter-shop.js",
	///         "globalName": "YTD.twitter_shop.part0",
	///         "count": "0"
	///       }
	///     ]
	///   },
	///   "userLinkClicks": {
	///     "files": [
	///       {
	///         "fileName": "data/user-link-clicks.js",
	///         "globalName": "YTD.user_link_clicks.part0",
	///         "count": "0"
	///       }
	///     ]
	///   },
	///   "verified": {
	///     "files": [
	///       {
	///         "fileName": "data/verified.js",
	///         "globalName": "YTD.verified.part0",
	///         "count": "1"
	///       }
	///     ]
	///   }
	/// }
	/// ```
	pub data_types: DataTypes,
}

/// ## Example
///
/// ```
/// use twitter_archive::structs::manifest::UserInfo;
///
/// let json = r#"{
///   "accountId": "111111111",
///   "userName": "S0_And_S0",
///   "displayName": "S0AndS0.eth"
/// }"#;
///
/// let data: UserInfo = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.account_id, "111111111");
/// assert_eq!(data.user_name, "S0_And_S0");
/// assert_eq!(data.display_name, "S0AndS0.eth");
///
/// // Re-serialize is equivalent to original data without pretty printing
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
	/// URL formats;
	///
	/// - Desktop: https://twitter.com/i/web/status/{id_str}
	/// - Mobile: https://mobile.twitter.com/i/web/status/{id_str}
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "accountId": "111111111" }
	/// ```
	pub account_id: String,

	/// Value that may be mentioned via `@NAME`
	///
	/// URL formats;
	///
	/// - Desktop: https://twitter.com/{screen_name}
	///
	/// > Note; redirects to log-in if not logged in, and redirections may be broken.  Thanks be to
	/// > Mr. Musk !-D
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "userName": "S0_And_S0" }
	/// ```
	pub user_name: String,

	/// Value that is shown to web-clients
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "displayName": "S0AndS0.eth" }
	/// ```
	pub display_name: String,
}

/// ## Example
///
/// ```
/// use chrono::{DateTime, NaiveDateTime, Utc};
///
/// use twitter_archive::convert::date_time_iso_8601::FORMAT;
///
/// use twitter_archive::structs::manifest::ArchiveInfo;
///
/// let generation_date_string = "2023-08-30T23:20:03.000Z";
/// let generation_date_native_time = NaiveDateTime::parse_from_str(&generation_date_string, FORMAT).unwrap();
/// let generation_date_date_time = DateTime::<Utc>::from_naive_utc_and_offset(generation_date_native_time, Utc);
///
/// let json = format!(r#"{{
///   "sizeBytes": "44546997",
///   "generationDate": "{generation_date_string}",
///   "isPartialArchive": false,
///   "maxPartSizeBytes": "53687091200"
/// }}"#);
///
/// let data: ArchiveInfo = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.size_bytes, 44546997);
/// assert_eq!(data.generation_date, generation_date_date_time);
/// assert_eq!(data.is_partial_archive, false);
/// assert_eq!(data.max_part_size_bytes, 53687091200);
///
/// // Re-serialize is equivalent to original data without pretty printing
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct ArchiveInfo {
	/// Size of archive mesured in bytes
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "sizeBytes": "44546997" }
	/// ```
	#[serde(with = "convert::number_like_string")]
	pub size_bytes: usize,

	/// When archive was generated
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "generationDate": "2023-08-30T23:20:03.000Z" }
	/// ```
	#[serde(with = "convert::date_time_iso_8601")]
	pub generation_date: DateTime<Utc>,

	/// Set to `true` if/when select portions of user data are archived instead of full account
	/// history of activity
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "isPartialArchive": false }
	/// ```
	pub is_partial_archive: bool,

	/// Size of partial archive max mesured in bytes?
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "maxPartSizeBytes": "53687091200" }
	/// ```
	#[serde(with = "convert::number_like_string")]
	pub max_part_size_bytes: usize,
}

/// ## Example
///
/// ```
/// use twitter_archive::structs::manifest::ReadmeInfo;
///
/// let json = r#"{
///   "fileName": "data/README.txt",
///   "directory": "data/",
///   "name": "README.txt"
/// }"#;
///
/// let data: ReadmeInfo = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.file_name, "data/README.txt");
/// assert_eq!(data.directory, "data/");
/// assert_eq!(data.name, "README.txt");
///
/// // Re-serialize is equivalent to original data without pretty printing
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct ReadmeInfo {
	/// Full relative file path with extension within archive
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "fileName": "data/README.txt" }
	/// ```
	pub file_name: String,

	/// Directory, with trailing forward-slash (`/`), file is saved under
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "directory": "data/" }
	/// ```
	pub directory: String,

	/// File name with file extension
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "name": "README.txt" }
	/// ```
	pub name: String,
}

/// ## Example
///
/// ```
/// use twitter_archive::structs::manifest::DataTypes;
///
/// let json = r#"{
///   "account": {
///     "files": [
///       {
///         "fileName": "data/account.js",
///         "globalName": "YTD.account.part0",
///         "count": "1"
///       }
///     ]
///   },
///   "accountCreationIp": {
///     "files": [
///       {
///         "fileName": "data/account-creation-ip.js",
///         "globalName": "YTD.account_creation_ip.part0",
///         "count": "1"
///       }
///     ]
///   },
///   "accountLabel": {
///     "files": [
///       {
///         "fileName": "data/account-label.js",
///         "globalName": "YTD.account_label.part0",
///         "count": "1"
///       }
///     ]
///   },
///   "accountSuspension": {
///     "files": [
///       {
///         "fileName": "data/account-suspension.js",
///         "globalName": "YTD.account_suspension.part0",
///         "count": "0"
///       }
///     ]
///   },
///   "accountTimezone": {
///     "files": [
///       {
///         "fileName": "data/account-timezone.js",
///         "globalName": "YTD.account_timezone.part0",
///         "count": "1"
///       }
///     ]
///   },
///   "adEngagements": {
///     "files": [
///       {
///         "fileName": "data/ad-engagements.js",
///         "globalName": "YTD.ad_engagements.part0",
///         "count": "122"
///       }
///     ]
///   },
///   "adImpressions": {
///     "files": [
///       {
///         "fileName": "data/ad-impressions.js",
///         "globalName": "YTD.ad_impressions.part0",
///         "count": "125"
///       }
///     ]
///   },
///   "adMobileConversionsAttributed": {
///     "files": [
///       {
///         "fileName": "data/ad-mobile-conversions-attributed.js",
///         "globalName": "YTD.ad_mobile_conversions_attributed.part0",
///         "count": "0"
///       }
///     ]
///   },
///   "adMobileConversionsUnattributed": {
///     "files": [
///       {
///         "fileName": "data/ad-mobile-conversions-unattributed.js",
///         "globalName": "YTD.ad_mobile_conversions_unattributed.part0",
///         "count": "0"
///       }
///     ]
///   },
///   "adOnlineConversionsAttributed": {
///     "files": [
///       {
///         "fileName": "data/ad-online-conversions-attributed.js",
///         "globalName": "YTD.ad_online_conversions_attributed.part0",
///         "count": "0"
///       }
///     ]
///   },
///   "adOnlineConversionsUnattributed": {
///     "files": [
///       {
///         "fileName": "data/ad-online-conversions-unattributed.js",
///         "globalName": "YTD.ad_online_conversions_unattributed.part0",
///         "count": "0"
///       }
///     ]
///   },
///   "ageinfo": {
///     "files": [
///       {
///         "fileName": "data/ageinfo.js",
///         "globalName": "YTD.ageinfo.part0",
///         "count": "1"
///       }
///     ]
///   },
///   "app": {
///     "files": [
///       {
///         "fileName": "data/app.js",
///         "globalName": "YTD.app.part0",
///         "count": "0"
///       }
///     ]
///   },
///   "block": {
///     "files": [
///       {
///         "fileName": "data/block.js",
///         "globalName": "YTD.block.part0",
///         "count": "10"
///       }
///     ]
///   },
///   "branchLinks": {
///     "files": [
///       {
///         "fileName": "data/branch-links.js",
///         "globalName": "YTD.branch_links.part0",
///         "count": "0"
///       }
///     ]
///   },
///   "catalogItem": {
///     "files": [
///       {
///         "fileName": "data/catalog-item.js",
///         "globalName": "YTD.catalog_item.part0",
///         "count": "0"
///       }
///     ]
///   },
///   "commerceCatalog": {
///     "files": [
///       {
///         "fileName": "data/commerce-catalog.js",
///         "globalName": "YTD.commerce_catalog.part0",
///         "count": "0"
///       }
///     ]
///   },
///   "communityNote": {
///     "files": [
///       {
///         "fileName": "data/community-note.js",
///         "globalName": "YTD.community_note.part0",
///         "count": "0"
///       }
///     ]
///   },
///   "communityNoteRating": {
///     "files": [
///       {
///         "fileName": "data/community-note-rating.js",
///         "globalName": "YTD.community_note_rating.part0",
///         "count": "1"
///       }
///     ]
///   },
///   "communityNoteTombstone": {
///     "files": [
///       {
///         "fileName": "data/community-note-tombstone.js",
///         "globalName": "YTD.community_note_tombstone.part0",
///         "count": "0"
///       }
///     ]
///   },
///   "communityTweet": {
///     "mediaDirectory": "data/community_tweet_media",
///     "files": [
///       {
///         "fileName": "data/community-tweet.js",
///         "globalName": "YTD.community_tweet.part0",
///         "count": "0"
///       }
///     ]
///   },
///   "communityTweetMedia": {
///     "mediaDirectory": "data/community_tweet_media"
///   },
///   "connectedApplication": {
///     "files": [
///       {
///         "fileName": "data/connected-application.js",
///         "globalName": "YTD.connected_application.part0",
///         "count": "1"
///       }
///     ]
///   },
///   "contact": {
///     "files": [
///       {
///         "fileName": "data/contact.js",
///         "globalName": "YTD.contact.part0",
///         "count": "0"
///       }
///     ]
///   },
///   "deletedNoteTweet": {
///     "files": [
///       {
///         "fileName": "data/deleted-note-tweet.js",
///         "globalName": "YTD.deleted_note_tweet.part0",
///         "count": "0"
///       }
///     ]
///   },
///   "deletedTweetHeaders": {
///     "files": [
///       {
///         "fileName": "data/deleted-tweet-headers.js",
///         "globalName": "YTD.deleted_tweet_headers.part0",
///         "count": "3"
///       }
///     ]
///   },
///   "deletedTweets": {
///     "mediaDirectory": "data/deleted_tweets_media",
///     "files": [
///       {
///         "fileName": "data/deleted-tweets.js",
///         "globalName": "YTD.deleted_tweets.part0",
///         "count": "3"
///       }
///     ]
///   },
///   "deletedTweetsMedia": {
///     "mediaDirectory": "data/deleted_tweets_media"
///   },
///   "deviceToken": {
///     "files": [
///       {
///         "fileName": "data/device-token.js",
///         "globalName": "YTD.device_token.part0",
///         "count": "600"
///       }
///     ]
///   },
///   "directMessageGroupHeaders": {
///     "files": [
///       {
///         "fileName": "data/direct-message-group-headers.js",
///         "globalName": "YTD.direct_message_group_headers.part0",
///         "count": "2"
///       }
///     ]
///   },
///   "directMessageHeaders": {
///     "files": [
///       {
///         "fileName": "data/direct-message-headers.js",
///         "globalName": "YTD.direct_message_headers.part0",
///         "count": "84"
///       }
///     ]
///   },
///   "directMessageMute": {
///     "files": [
///       {
///         "fileName": "data/direct-message-mute.js",
///         "globalName": "YTD.direct_message_mute.part0",
///         "count": "0"
///       }
///     ]
///   },
///   "directMessages": {
///     "mediaDirectory": "data/direct_messages_media",
///     "files": [
///       {
///         "fileName": "data/direct-messages.js",
///         "globalName": "YTD.direct_messages.part0",
///         "count": "84"
///       }
///     ]
///   },
///   "directMessagesGroup": {
///     "mediaDirectory": "data/direct_messages_group_media",
///     "files": [
///       {
///         "fileName": "data/direct-messages-group.js",
///         "globalName": "YTD.direct_messages_group.part0",
///         "count": "2"
///       }
///     ]
///   },
///   "directMessagesGroupMedia": {
///     "mediaDirectory": "data/direct_messages_group_media"
///   },
///   "directMessagesMedia": {
///     "mediaDirectory": "data/direct_messages_media"
///   },
///   "emailAddressChange": {
///     "files": [
///       {
///         "fileName": "data/email-address-change.js",
///         "globalName": "YTD.email_address_change.part0",
///         "count": "2"
///       }
///     ]
///   },
///   "follower": {
///     "files": [
///       {
///         "fileName": "data/follower.js",
///         "globalName": "YTD.follower.part0",
///         "count": "289"
///       }
///     ]
///   },
///   "following": {
///     "files": [
///       {
///         "fileName": "data/following.js",
///         "globalName": "YTD.following.part0",
///         "count": "283"
///       }
///     ]
///   },
///   "ipAudit": {
///     "files": [
///       {
///         "fileName": "data/ip-audit.js",
///         "globalName": "YTD.ip_audit.part0",
///         "count": "542"
///       }
///     ]
///   },
///   "keyRegistry": {
///     "files": [
///       {
///         "fileName": "data/key-registry.js",
///         "globalName": "YTD.key_registry.part0",
///         "count": "1"
///       }
///     ]
///   },
///   "like": {
///     "files": [
///       {
///         "fileName": "data/like.js",
///         "globalName": "YTD.like.part0",
///         "count": "25548"
///       }
///     ]
///   },
///   "listsCreated": {
///     "files": [
///       {
///         "fileName": "data/lists-created.js",
///         "globalName": "YTD.lists_created.part0",
///         "count": "0"
///       }
///     ]
///   },
///   "listsMember": {
///     "files": [
///       {
///         "fileName": "data/lists-member.js",
///         "globalName": "YTD.lists_member.part0",
///         "count": "11"
///       }
///     ]
///   },
///   "listsSubscribed": {
///     "files": [
///       {
///         "fileName": "data/lists-subscribed.js",
///         "globalName": "YTD.lists_subscribed.part0",
///         "count": "0"
///       }
///     ]
///   },
///   "moment": {
///     "mediaDirectory": "data/moments_media",
///     "files": [
///       {
///         "fileName": "data/moment.js",
///         "globalName": "YTD.moment.part0",
///         "count": "0"
///       }
///     ]
///   },
///   "momentsMedia": {
///     "mediaDirectory": "data/moments_media"
///   },
///   "momentsTweetsMedia": {
///     "mediaDirectory": "data/moments_tweets_media"
///   },
///   "mute": {
///     "files": [
///       {
///         "fileName": "data/mute.js",
///         "globalName": "YTD.mute.part0",
///         "count": "4086"
///       }
///     ]
///   },
///   "niDevices": {
///     "files": [
///       {
///         "fileName": "data/ni-devices.js",
///         "globalName": "YTD.ni_devices.part0",
///         "count": "1"
///       }
///     ]
///   },
///   "noteTweet": {
///     "files": [
///       {
///         "fileName": "data/note-tweet.js",
///         "globalName": "YTD.note_tweet.part0",
///         "count": "0"
///       }
///     ]
///   },
///   "periscopeAccountInformation": {
///     "files": [
///       {
///         "fileName": "data/periscope-account-information.js",
///         "globalName": "YTD.periscope_account_information.part0",
///         "count": "0"
///       }
///     ]
///   },
///   "periscopeBanInformation": {
///     "files": [
///       {
///         "fileName": "data/periscope-ban-information.js",
///         "globalName": "YTD.periscope_ban_information.part0",
///         "count": "0"
///       }
///     ]
///   },
///   "periscopeBroadcastMetadata": {
///     "files": [
///       {
///         "fileName": "data/periscope-broadcast-metadata.js",
///         "globalName": "YTD.periscope_broadcast_metadata.part0",
///         "count": "0"
///       }
///     ]
///   },
///   "periscopeCommentsMadeByUser": {
///     "files": [
///       {
///         "fileName": "data/periscope-comments-made-by-user.js",
///         "globalName": "YTD.periscope_comments_made_by_user.part0",
///         "count": "0"
///       }
///     ]
///   },
///   "periscopeExpiredBroadcasts": {
///     "files": [
///       {
///         "fileName": "data/periscope-expired-broadcasts.js",
///         "globalName": "YTD.periscope_expired_broadcasts.part0",
///         "count": "0"
///       }
///     ]
///   },
///   "periscopeFollowers": {
///     "files": [
///       {
///         "fileName": "data/periscope-followers.js",
///         "globalName": "YTD.periscope_followers.part0",
///         "count": "0"
///       }
///     ]
///   },
///   "periscopeProfileDescription": {
///     "files": [
///       {
///         "fileName": "data/periscope-profile-description.js",
///         "globalName": "YTD.periscope_profile_description.part0",
///         "count": "0"
///       }
///     ]
///   },
///   "personalization": {
///     "files": [
///       {
///         "fileName": "data/personalization.js",
///         "globalName": "YTD.personalization.part0",
///         "count": "1"
///       }
///     ]
///   },
///   "phoneNumber": {
///     "files": [
///       {
///         "fileName": "data/phone-number.js",
///         "globalName": "YTD.phone_number.part0",
///         "count": "1"
///       }
///     ]
///   },
///   "productDrop": {
///     "files": [
///       {
///         "fileName": "data/product-drop.js",
///         "globalName": "YTD.product_drop.part0",
///         "count": "0"
///       }
///     ]
///   },
///   "productSet": {
///     "files": [
///       {
///         "fileName": "data/product-set.js",
///         "globalName": "YTD.product_set.part0",
///         "count": "0"
///       }
///     ]
///   },
///   "professionalData": {
///     "files": [
///       {
///         "fileName": "data/professional-data.js",
///         "globalName": "YTD.professional_data.part0",
///         "count": "0"
///       }
///     ]
///   },
///   "profile": {
///     "mediaDirectory": "data/profile_media",
///     "files": [
///       {
///         "fileName": "data/profile.js",
///         "globalName": "YTD.profile.part0",
///         "count": "1"
///       }
///     ]
///   },
///   "profileMedia": {
///     "mediaDirectory": "data/profile_media"
///   },
///   "protectedHistory": {
///     "files": [
///       {
///         "fileName": "data/protected-history.js",
///         "globalName": "YTD.protected_history.part0",
///         "count": "0"
///       }
///     ]
///   },
///   "replyPrompt": {
///     "files": [
///       {
///         "fileName": "data/reply-prompt.js",
///         "globalName": "YTD.reply_prompt.part0",
///         "count": "0"
///       }
///     ]
///   },
///   "savedSearch": {
///     "files": [
///       {
///         "fileName": "data/saved-search.js",
///         "globalName": "YTD.saved_search.part0",
///         "count": "0"
///       }
///     ]
///   },
///   "screenNameChange": {
///     "files": [
///       {
///         "fileName": "data/screen-name-change.js",
///         "globalName": "YTD.screen_name_change.part0",
///         "count": "1"
///       }
///     ]
///   },
///   "shopModule": {
///     "files": [
///       {
///         "fileName": "data/shop-module.js",
///         "globalName": "YTD.shop_module.part0",
///         "count": "0"
///       }
///     ]
///   },
///   "shopifyAccount": {
///     "files": [
///       {
///         "fileName": "data/shopify-account.js",
///         "globalName": "YTD.shopify_account.part0",
///         "count": "0"
///       }
///     ]
///   },
///   "smartblock": {
///     "files": [
///       {
///         "fileName": "data/smartblock.js",
///         "globalName": "YTD.smartblock.part0",
///         "count": "0"
///       }
///     ]
///   },
///   "spacesMetadata": {
///     "files": [
///       {
///         "fileName": "data/spaces-metadata.js",
///         "globalName": "YTD.spaces_metadata.part0",
///         "count": "0"
///       }
///     ]
///   },
///   "sso": {
///     "files": [
///       {
///         "fileName": "data/sso.js",
///         "globalName": "YTD.sso.part0",
///         "count": "0"
///       }
///     ]
///   },
///   "tweetHeaders": {
///     "files": [
///       {
///         "fileName": "data/tweet-headers.js",
///         "globalName": "YTD.tweet_headers.part0",
///         "count": "3601"
///       }
///     ]
///   },
///   "tweetdeck": {
///     "files": [
///       {
///         "fileName": "data/tweetdeck.js",
///         "globalName": "YTD.tweetdeck.part0",
///         "count": "2"
///       }
///     ]
///   },
///   "tweets": {
///     "mediaDirectory": "data/tweets_media",
///     "files": [
///       {
///         "fileName": "data/tweets.js",
///         "globalName": "YTD.tweets.part0",
///         "count": "3601"
///       }
///     ]
///   },
///   "tweetsMedia": {
///     "mediaDirectory": "data/tweets_media"
///   },
///   "twitterArticle": {
///     "mediaDirectory": "data/twitter_article_media",
///     "files": [
///       {
///         "fileName": "data/twitter-article.js",
///         "globalName": "YTD.twitter_article.part0",
///         "count": "0"
///       }
///     ]
///   },
///   "twitterArticleMedia": {
///     "mediaDirectory": "data/twitter_article_media"
///   },
///   "twitterArticleMetadata": {
///     "files": [
///       {
///         "fileName": "data/twitter-article-metadata.js",
///         "globalName": "YTD.twitter_article_metadata.part0",
///         "count": "0"
///       }
///     ]
///   },
///   "twitterCircle": {
///     "mediaDirectory": "data/twitter_circle_tweet_media",
///     "files": [
///       {
///         "fileName": "data/twitter-circle.js",
///         "globalName": "YTD.twitter_circle.part0",
///         "count": "1"
///       }
///     ]
///   },
///   "twitterCircleMember": {
///     "files": [
///       {
///         "fileName": "data/twitter-circle-member.js",
///         "globalName": "YTD.twitter_circle_member.part0",
///         "count": "0"
///       }
///     ]
///   },
///   "twitterCircleTweet": {
///     "mediaDirectory": "data/twitter_circle_tweet_media",
///     "files": [
///       {
///         "fileName": "data/twitter-circle-tweet.js",
///         "globalName": "YTD.twitter_circle_tweet.part0",
///         "count": "0"
///       }
///     ]
///   },
///   "twitterCircleTweetMedia": {
///     "mediaDirectory": "data/twitter_circle_tweet_media"
///   },
///   "twitterShop": {
///     "files": [
///       {
///         "fileName": "data/twitter-shop.js",
///         "globalName": "YTD.twitter_shop.part0",
///         "count": "0"
///       }
///     ]
///   },
///   "userLinkClicks": {
///     "files": [
///       {
///         "fileName": "data/user-link-clicks.js",
///         "globalName": "YTD.user_link_clicks.part0",
///         "count": "0"
///       }
///     ]
///   },
///   "verified": {
///     "files": [
///       {
///         "fileName": "data/verified.js",
///         "globalName": "YTD.verified.part0",
///         "count": "1"
///       }
///     ]
///   }
/// }"#;
///
/// let data: DataTypes = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.twitter_circle_tweet.media_directory, "data/twitter_circle_tweet_media");
///
/// assert_eq!(data.twitter_circle_tweet.files.len(), 1);
/// assert_eq!(data.twitter_circle_tweet.files[0].file_name, "data/twitter-circle-tweet.js");
/// assert_eq!(data.twitter_circle_tweet.files[0].global_name, "YTD.twitter_circle_tweet.part0");
/// assert_eq!(data.twitter_circle_tweet.files[0].count, 0);
///
/// assert_eq!(data.verified.files.len(), 1);
/// assert_eq!(data.verified.files[0].file_name, "data/verified.js");
/// assert_eq!(data.verified.files[0].global_name, "YTD.verified.part0");
/// assert_eq!(data.verified.files[0].count, 1);
///
/// // Re-serialize is equivalent to original data without pretty printing
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct DataTypes {
	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/account.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/account.js",
	///       "globalName": "YTD.account.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub account: FileObject,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/account-creation-ip.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/account-creation-ip.js",
	///       "globalName": "YTD.account_creation_ip.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub account_creation_ip: FileObject,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/account-label.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/account-label.js",
	///       "globalName": "YTD.account_label.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub account_label: FileObject,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/account-suspension.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/account-suspension.js",
	///       "globalName": "YTD.account_suspension.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub account_suspension: FileObject,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/account-timezone.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/account-timezone.js",
	///       "globalName": "YTD.account_timezone.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub account_timezone: FileObject,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/ad-engagements.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/ad-engagements.js",
	///       "globalName": "YTD.ad_engagements.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub ad_engagements: FileObject,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/ad-impressions.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/ad-impressions.js",
	///       "globalName": "YTD.ad_impressions.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub ad_impressions: FileObject,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/ad-mobile-conversions-attributed.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/ad-mobile-conversions-attributed.js",
	///       "globalName": "YTD.ad_mobile_conversions_attributed.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub ad_mobile_conversions_attributed: FileObject,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/ad-mobile-conversions-unattributed.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/ad-mobile-conversions-unattributed.js",
	///       "globalName": "YTD.ad_mobile_conversions_unattributed.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub ad_mobile_conversions_unattributed: FileObject,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/ad-online-conversions-attributed.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/ad-online-conversions-attributed.js",
	///       "globalName": "YTD.ad_online_conversions_attributed.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub ad_online_conversions_attributed: FileObject,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/ad-online-conversions-unattributed.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/ad-online-conversions-unattributed.js",
	///       "globalName": "YTD.ad_online_conversions_unattributed.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub ad_online_conversions_unattributed: FileObject,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/ageinfo.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/ageinfo.js",
	///       "globalName": "YTD.ageinfo.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub ageinfo: FileObject,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/app.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/app.js",
	///       "globalName": "YTD.app.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub app: FileObject,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/block.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/block.js",
	///       "globalName": "YTD.block.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub block: FileObject,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/branch-links.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/branch-links.js",
	///       "globalName": "YTD.branch_links.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub branch_links: FileObject,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/catalog-item.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/catalog-item.js",
	///       "globalName": "YTD.catalog_item.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub catalog_item: FileObject,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/commerce-catalog.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/commerce-catalog.js",
	///       "globalName": "YTD.commerce_catalog.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub commerce_catalog: FileObject,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/community-note.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/community-note.js",
	///       "globalName": "YTD.community_note.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub community_note: FileObject,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/community-note-rating.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/community-note-rating.js",
	///       "globalName": "YTD.community_note_rating.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub community_note_rating: FileObject,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/community-note-tombstone.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/community-note-tombstone.js",
	///       "globalName": "YTD.community_note_tombstone.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub community_note_tombstone: FileObject,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/community-tweet.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "mediaDirectory": "data/community_tweet",
	///   "files": [
	///     {
	///       "fileName": "data/community-tweet.js",
	///       "globalName": "YTD.community_tweet.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub community_tweet: MediaDirectoryWithFiles,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/community-tweet-media.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "mediaDirectory": "data/community_tweet_media" }
	/// ```
	pub community_tweet_media: MediaDirectory,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/connected-application.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/connected-application.js",
	///       "globalName": "YTD.connected_application.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub connected_application: FileObject,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/contact.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/contact.js",
	///       "globalName": "YTD.contact.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub contact: FileObject,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/deleted-note-tweet.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/deleted-note-tweet.js",
	///       "globalName": "YTD.deleted_note_tweet.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub deleted_note_tweet: FileObject,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/deleted-tweet-headers.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/deleted-tweet-headers.js",
	///       "globalName": "YTD.deleted_tweet_headers.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub deleted_tweet_headers: FileObject,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/deleted-tweets.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "mediaDirectory": "data/deleted_tweets",
	///   "files": [
	///     {
	///       "fileName": "data/deleted-tweets.js",
	///       "globalName": "YTD.deleted_tweets.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub deleted_tweets: MediaDirectoryWithFiles,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/deleted-tweets-media.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "mediaDirectory": "data/deleted_tweets_media" }
	/// ```
	pub deleted_tweets_media: MediaDirectory,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/device-token.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/device-token.js",
	///       "globalName": "YTD.device_token.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub device_token: FileObject,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/direct-message-group-headers.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/direct-message-group-headers.js",
	///       "globalName": "YTD.direct_message_group_headers.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub direct_message_group_headers: FileObject,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/direct-message-headers.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/direct-message-headers.js",
	///       "globalName": "YTD.direct_message_headers.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub direct_message_headers: FileObject,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/direct-message-mute.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/direct-message-mute.js",
	///       "globalName": "YTD.direct_message_mute.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub direct_message_mute: FileObject,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/direct-messages.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "mediaDirectory": "data/direct_messages",
	///   "files": [
	///     {
	///       "fileName": "data/direct-messages.js",
	///       "globalName": "YTD.direct_messages.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub direct_messages: MediaDirectoryWithFiles,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/direct-messages-group.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "mediaDirectory": "data/direct_messages_group",
	///   "files": [
	///     {
	///       "fileName": "data/direct-messages-group.js",
	///       "globalName": "YTD.direct_messages_group.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub direct_messages_group: MediaDirectoryWithFiles,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/direct-messages-group-media.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "mediaDirectory": "data/direct_messages_group_media" }
	/// ```
	pub direct_messages_group_media: MediaDirectory,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/direct-messages-media.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "mediaDirectory": "data/direct_messages_media" }
	/// ```
	pub direct_messages_media: MediaDirectory,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/email-address-change.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/email-address-change.js",
	///       "globalName": "YTD.email_address_change.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub email_address_change: FileObject,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/follower.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/follower.js",
	///       "globalName": "YTD.follower.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub follower: FileObject,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/following.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/following.js",
	///       "globalName": "YTD.following.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub following: FileObject,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/ip-audit.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/ip-audit.js",
	///       "globalName": "YTD.ip_audit.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub ip_audit: FileObject,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/key-registry.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/key-registry.js",
	///       "globalName": "YTD.key_registry.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub key_registry: FileObject,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/like.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/like.js",
	///       "globalName": "YTD.like.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub like: FileObject,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/lists-created.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/lists-created.js",
	///       "globalName": "YTD.lists_created.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub lists_created: FileObject,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/lists-member.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/lists-member.js",
	///       "globalName": "YTD.lists_member.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub lists_member: FileObject,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/lists-subscribed.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/lists-subscribed.js",
	///       "globalName": "YTD.lists_subscribed.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub lists_subscribed: FileObject,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/moment.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "mediaDirectory": "data/moment",
	///   "files": [
	///     {
	///       "fileName": "data/moment.js",
	///       "globalName": "YTD.moment.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub moment: MediaDirectoryWithFiles,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/moments-media.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "mediaDirectory": "data/moments_media" }
	/// ```
	pub moments_media: MediaDirectory,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/moments-tweets-media.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "mediaDirectory": "data/moments_tweets_media" }
	/// ```
	pub moments_tweets_media: MediaDirectory,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/mute.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/mute.js",
	///       "globalName": "YTD.mute.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub mute: FileObject,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/ni-devices.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/ni-devices.js",
	///       "globalName": "YTD.ni_devices.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub ni_devices: FileObject,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/note-tweet.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/note-tweet.js",
	///       "globalName": "YTD.note_tweet.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub note_tweet: FileObject,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/periscope-account-information.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/periscope-account-information.js",
	///       "globalName": "YTD.periscope_account_information.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub periscope_account_information: FileObject,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/periscope-ban-information.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/periscope-ban-information.js",
	///       "globalName": "YTD.periscope_ban_information.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub periscope_ban_information: FileObject,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/periscope-broadcast-metadata.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/periscope-broadcast-metadata.js",
	///       "globalName": "YTD.periscope_broadcast_metadata.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub periscope_broadcast_metadata: FileObject,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/periscope-comments-made-by-user.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/periscope-comments-made-by-user.js",
	///       "globalName": "YTD.periscope_comments_made_by_user.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub periscope_comments_made_by_user: FileObject,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/periscope-expired-broadcasts.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/periscope-expired-broadcasts.js",
	///       "globalName": "YTD.periscope_expired_broadcasts.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub periscope_expired_broadcasts: FileObject,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/periscope-followers.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/periscope-followers.js",
	///       "globalName": "YTD.periscope_followers.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub periscope_followers: FileObject,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/periscope-profile-description.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/periscope-profile-description.js",
	///       "globalName": "YTD.periscope_profile_description.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub periscope_profile_description: FileObject,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/personalization.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/personalization.js",
	///       "globalName": "YTD.personalization.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub personalization: FileObject,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/phone-number.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/phone-number.js",
	///       "globalName": "YTD.phone_number.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub phone_number: FileObject,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/product-drop.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/product-drop.js",
	///       "globalName": "YTD.product_drop.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub product_drop: FileObject,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/product-set.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/product-set.js",
	///       "globalName": "YTD.product_set.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub product_set: FileObject,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/professional-data.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/professional-data.js",
	///       "globalName": "YTD.professional_data.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub professional_data: FileObject,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/profile.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "mediaDirectory": "data/profile",
	///   "files": [
	///     {
	///       "fileName": "data/profile.js",
	///       "globalName": "YTD.profile.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub profile: MediaDirectoryWithFiles,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/profile-media.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "mediaDirectory": "data/profile_media" }
	/// ```
	pub profile_media: MediaDirectory,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/protected-history.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/protected-history.js",
	///       "globalName": "YTD.protected_history.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub protected_history: FileObject,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/reply-prompt.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/reply-prompt.js",
	///       "globalName": "YTD.reply_prompt.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub reply_prompt: FileObject,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/saved-search.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/saved-search.js",
	///       "globalName": "YTD.saved_search.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub saved_search: FileObject,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/screen-name-change.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/screen-name-change.js",
	///       "globalName": "YTD.screen_name_change.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub screen_name_change: FileObject,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/shop-module.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/shop-module.js",
	///       "globalName": "YTD.shop_module.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub shop_module: FileObject,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/shopify-account.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/shopify-account.js",
	///       "globalName": "YTD.shopify_account.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub shopify_account: FileObject,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/smartblock.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/smartblock.js",
	///       "globalName": "YTD.smartblock.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub smartblock: FileObject,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/spaces-metadata.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/spaces-metadata.js",
	///       "globalName": "YTD.spaces_metadata.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub spaces_metadata: FileObject,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/sso.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/sso.js",
	///       "globalName": "YTD.sso.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub sso: FileObject,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/tweet-headers.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/tweet-headers.js",
	///       "globalName": "YTD.tweet_headers.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub tweet_headers: FileObject,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/tweetdeck.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/tweetdeck.js",
	///       "globalName": "YTD.tweetdeck.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub tweetdeck: FileObject,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/tweets.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "mediaDirectory": "data/tweets",
	///   "files": [
	///     {
	///       "fileName": "data/tweets.js",
	///       "globalName": "YTD.tweets.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub tweets: MediaDirectoryWithFiles,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/tweets-media.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "mediaDirectory": "data/tweets_media" }
	/// ```
	pub tweets_media: MediaDirectory,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/twitter-article.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "mediaDirectory": "data/twitter_article",
	///   "files": [
	///     {
	///       "fileName": "data/twitter-article.js",
	///       "globalName": "YTD.twitter_article.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub twitter_article: MediaDirectoryWithFiles,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/twitter-article-media.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "mediaDirectory": "data/twitter_article_media" }
	/// ```
	pub twitter_article_media: MediaDirectory,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/twitter-article-metadata.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/twitter-article-metadata.js",
	///       "globalName": "YTD.twitter_article_metadata.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub twitter_article_metadata: FileObject,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/twitter-circle.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "mediaDirectory": "data/twitter_circle",
	///   "files": [
	///     {
	///       "fileName": "data/twitter-circle.js",
	///       "globalName": "YTD.twitter_circle.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub twitter_circle: MediaDirectoryWithFiles,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/twitter-circle-member.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/twitter-circle-member.js",
	///       "globalName": "YTD.twitter_circle_member.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub twitter_circle_member: FileObject,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/twitter-circle-tweet.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "mediaDirectory": "data/twitter_circle_tweet",
	///   "files": [
	///     {
	///       "fileName": "data/twitter-circle-tweet.js",
	///       "globalName": "YTD.twitter_circle_tweet.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub twitter_circle_tweet: MediaDirectoryWithFiles,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/twitter-circle-tweet-media.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "mediaDirectory": "data/twitter_circle_tweet_media" }
	/// ```
	pub twitter_circle_tweet_media: MediaDirectory,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/twitter-shop.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/twitter-shop.js",
	///       "globalName": "YTD.twitter_shop.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub twitter_shop: FileObject,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/user-link-clicks.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/user-link-clicks.js",
	///       "globalName": "YTD.user_link_clicks.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub user_link_clicks: FileObject,

	/// Metadata that usually points to `twitter-archive-<UID>.zip:data/verified.js`
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/verified.js",
	///       "globalName": "YTD.verified.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub verified: FileObject,
}

/// Data structure common to some media `manifest.dataTypes` that point to a directory and files
///
/// ## Example
///
/// ```
/// use twitter_archive::structs::manifest::MediaDirectoryWithFiles;
///
/// let json = r#"{
///   "mediaDirectory": "data/twitter_circle_tweet_media",
///   "files": [
///     {
///       "fileName": "data/twitter-circle-tweet.js",
///       "globalName": "YTD.twitter_circle_tweet.part0",
///       "count": "0"
///     }
///   ]
/// }"#;
///
/// let data: MediaDirectoryWithFiles = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.media_directory, "data/twitter_circle_tweet_media");
///
/// assert_eq!(data.files.len(), 1);
/// assert_eq!(data.files[0].file_name, "data/twitter-circle-tweet.js");
/// assert_eq!(data.files[0].global_name, "YTD.twitter_circle_tweet.part0");
/// assert_eq!(data.files[0].count, 0);
///
/// // Re-serialize is equivalent to original data without pretty printing
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct MediaDirectoryWithFiles {
	/// Relative path within archive to directory that contain media files
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "mediaDirectory": "data/twitter_circle_tweet_media" }
	/// ```
	pub media_directory: String,

	/// Metadata that points to where further JavaScript/JSON data may be parsed from
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/twitter-circle-tweet.js",
	///       "globalName": "YTD.twitter_circle_tweet.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub files: Vec<File>,
}

/// Data structure common to most media `manifest.dataTypes`
///
/// ## Example
///
/// ```
/// use twitter_archive::structs::manifest::MediaDirectory;
///
/// let json = r#"{
///   "mediaDirectory": "data/twitter_circle_tweet_media"
/// }"#;
///
/// let data: MediaDirectory = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.media_directory, "data/twitter_circle_tweet_media");
///
/// // Re-serialize is equivalent to original data without pretty printing
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct MediaDirectory {
	/// Relative path within archive to directory that contain media files
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "mediaDirectory": "data/twitter_circle_tweet_media" }
	/// ```
	pub media_directory: String,
}

/// Data structure common to most non-media `manifest.dataTypes`
///
/// ## Example
///
/// ```
/// use twitter_archive::structs::manifest::FileObject;
///
/// let json = r#"{
///   "files": [
///     {
///       "fileName": "data/twitter-shop.js",
///       "globalName": "YTD.twitter_shop.part0",
///       "count": "0"
///     }
///   ]
/// }"#;
///
/// let data: FileObject = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.files.len(), 1);
/// assert_eq!(data.files[0].file_name, "data/twitter-shop.js");
/// assert_eq!(data.files[0].global_name, "YTD.twitter_shop.part0");
/// assert_eq!(data.files[0].count, 0);
///
/// // Re-serialize is equivalent to original data without pretty printing
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct FileObject {
	/// List of metadata pointers to JavaScript/JSON files
	///
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "files": [
	///     {
	///       "fileName": "data/twitter-shop.js",
	///       "globalName": "YTD.twitter_shop.part0",
	///       "count": "0"
	///     }
	///   ]
	/// }
	/// ```
	pub files: Vec<File>,
}

/// Points to file path within zip archive and describes JavaScript pointer data may be accessed
///
/// ## Example
///
/// ```
/// use twitter_archive::structs::manifest::File;
///
/// let json = r#"{
///   "fileName": "data/twitter-shop.js",
///   "globalName": "YTD.twitter_shop.part0",
///   "count": "0"
/// }"#;
///
/// let data: File = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.file_name, "data/twitter-shop.js");
/// assert_eq!(data.global_name, "YTD.twitter_shop.part0");
/// assert_eq!(data.count, 0);
///
/// // Re-serialize is equivalent to original data without pretty printing
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct File {
	/// Relative path to file within archive
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "fileName": "data/twitter-shop.js" }
	/// ```
	pub file_name: String,

	/// JavaScript Object pointer(s) to data if loaded by web-browser
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "globalName": "YTD.twitter_shop.part0" }
	/// ```
	pub global_name: String,

	/// Count seems to not be directly associated with number of files; perhaps it provides a count
	/// of JSON objects within a given file?
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "count": "0" }
	/// ```
	#[serde(with = "convert::number_like_string")]
	pub count: usize,
}
