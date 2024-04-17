#!/usr/bin/env rust

//! Tweeter archives as of 2023-08-31 have private data found under;
//!
//!   twitter-<DATE>-<UID>.zip:data/personalization.js
//!
//! ## Warnings
//!
//! - `.[].<KEY_NAME>.LocationHistory` data structure is subject to future changes
//!
//! ## Example file reader
//!
//! ```no_build
//! use std::io::Read;
//! use std::{fs, path};
//! use zip::read::ZipArchive;
//!
//! use twitter_archive::structs::personalization;
//!
//! fn main() {
//!     let input_file = "~/Downloads/twitter-archive.zip";
//!
//!     let file_descriptor = fs::File::open(input_file).expect("Unable to read --input-file");
//!     let mut zip_archive = ZipArchive::new(file_descriptor).unwrap();
//!     let mut zip_file = zip_archive.by_name("data/personalization.js").unwrap();
//!     let mut buff = String::new();
//!     zip_file.read_to_string(&mut buff).unwrap();
//!
//!     let json = buff.replacen("window.YTD.personalization.part0 = ", "", 1);
//!     let data: Vec<personalization::P13nDataObject> = serde_json::from_str(&json).expect("Unable to parse");
//!
//!     for (index, object) in data.iter().enumerate() {
//!         /* Do stuff with each `p13nData` entry */
//!         println!("Personalization index: {index}");
//!         println!("Demographics language: {}", object.p13n_data.demographics.languages[0].language);
//!         println!("Demographics gender: {}", object.p13n_data.demographics.gender_info.gender);
//!     }
//! }
//! ```
//!
//! ## Example `twitter-<DATE>-<UID>.zip:data/personalization.js` content
//!
//! ```javascript
//! window.YTD.personalization.part0 = [
//!   {
//!     "device" : {
//!       "phoneNumber" : "+15551234567"
//!     }
//!   }
//! ]
//! ```

use derive_more::Display;
use serde::{Deserialize, Serialize};

use crate::convert;

/// ## Example
///
/// ```
/// use twitter_archive::structs::personalization::P13nDataObject;
///
/// let json = r##"{
///   "p13nData": {
///     "demographics": {
///       "languages": [
///         {
///           "language": "English",
///           "isDisabled": false
///         }
///       ],
///       "genderInfo": {
///         "gender": "unknown",
///         "genderOverride": "Borg"
///       }
///     },
///     "interests": {
///       "interests": [
///         {
///           "name": "#HappyFriday",
///           "isDisabled": false
///         }
///       ],
///       "partnerInterests": [],
///       "audienceAndAdvertisers": {
///         "lookalikeAdvertisers": [
///           "@EXAMPLE_ONE",
///           "@EXAMPLE_TWO"
///         ],
///         "advertisers": [],
///         "doNotReachAdvertisers": [],
///         "catalogAudienceAdvertisers": [],
///         "numAudiences": "0"
///       },
///       "shows": [
///         "1899",
///         "DuckTales"
///       ]
///     },
///     "locationHistory": [],
///     "inferredAgeInfo": {
///       "age": [
///         "13-99"
///       ],
///       "birthDate": ""
///     }
///   }
/// }"##;
///
/// let data: P13nDataObject = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.p13n_data.demographics.languages[0].language, "English");
/// assert_eq!(data.p13n_data.demographics.languages[0].is_disabled, false);
/// assert_eq!(data.p13n_data.demographics.gender_info.gender, "unknown");
/// assert_eq!(data.p13n_data.demographics.gender_info.gender_override, "Borg");
///
/// assert_eq!(data.p13n_data.interests.interests[0].name, "#HappyFriday");
/// assert_eq!(data.p13n_data.interests.interests[0].is_disabled, false);
/// assert_eq!(data.p13n_data.interests.partner_interests.len(), 0);
/// assert_eq!(data.p13n_data.interests.audience_and_advertisers.lookalike_advertisers.len(), 2);
/// assert_eq!(data.p13n_data.interests.audience_and_advertisers.advertisers.len(), 0);
/// assert_eq!(data.p13n_data.interests.audience_and_advertisers.do_not_reach_advertisers.len(), 0);
/// assert_eq!(data.p13n_data.interests.audience_and_advertisers.catalog_audience_advertisers.len(), 0);
/// assert_eq!(data.p13n_data.interests.audience_and_advertisers.num_audiences, 0);
/// assert_eq!(data.p13n_data.interests.shows[0], "1899");
/// assert_eq!(data.p13n_data.interests.shows[1], "DuckTales");
///
/// assert_eq!(data.p13n_data.location_history.len(), 0);
///
/// assert_eq!(data.p13n_data.inferred_age_info.age[0], "13-99");
/// assert_eq!(data.p13n_data.inferred_age_info.birth_date, "");
///
/// // Re-serialize is equivalent to original data
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct P13nDataObject {
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "p13nData": {
	///     "demographics": {
	///       "languages": [
	///         {
	///           "language": "English",
	///           "isDisabled": false
	///         }
	///       ],
	///       "genderInfo": {
	///         "gender": "unknown",
	///         "genderOverride": "Borg"
	///       }
	///     },
	///     "interests": {
	///       "interests": [
	///         {
	///           "name": "#HappyFriday",
	///           "isDisabled": false
	///         }
	///       ],
	///       "partnerInterests": [],
	///       "audienceAndAdvertisers": {
	///         "lookalikeAdvertisers": [
	///           "@EXAMPLE_ONE",
	///           "@EXAMPLE_TWO"
	///         ],
	///         "advertisers": [],
	///         "doNotReachAdvertisers": [],
	///         "catalogAudienceAdvertisers": [],
	///         "numAudiences": "0"
	///       },
	///       "shows": [
	///         "1899",
	///         "DuckTales"
	///       ]
	///     },
	///     "locationHistory": [],
	///     "inferredAgeInfo": {
	///       "age": [
	///         "13-99"
	///       ],
	///       "birthDate": ""
	///     }
	///   }
	/// }
	/// ```
	pub p13n_data: P13nData,
}

/// ## Example
///
/// ```
/// use twitter_archive::structs::personalization::P13nData;
///
/// let json = r##"{
///   "demographics": {
///     "languages": [
///       {
///         "language": "English",
///         "isDisabled": false
///       }
///     ],
///     "genderInfo": {
///       "gender": "unknown",
///       "genderOverride": "Borg"
///     }
///   },
///   "interests": {
///     "interests": [
///       {
///         "name": "#HappyFriday",
///         "isDisabled": false
///       }
///     ],
///     "partnerInterests": [],
///     "audienceAndAdvertisers": {
///       "lookalikeAdvertisers": [
///         "@EXAMPLE_ONE",
///         "@EXAMPLE_TWO"
///       ],
///       "advertisers": [],
///       "doNotReachAdvertisers": [],
///       "catalogAudienceAdvertisers": [],
///       "numAudiences": "0"
///     },
///     "shows": [
///       "1899",
///       "DuckTales"
///     ]
///   },
///   "locationHistory": [],
///   "inferredAgeInfo": {
///     "age": [
///       "13-99"
///     ],
///     "birthDate": ""
///   }
/// }"##;
///
/// let data: P13nData = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.demographics.languages[0].language, "English");
/// assert_eq!(data.demographics.languages[0].is_disabled, false);
/// assert_eq!(data.demographics.gender_info.gender, "unknown");
/// assert_eq!(data.demographics.gender_info.gender_override, "Borg");
///
/// assert_eq!(data.interests.interests[0].name, "#HappyFriday");
/// assert_eq!(data.interests.interests[0].is_disabled, false);
/// assert_eq!(data.interests.partner_interests.len(), 0);
/// assert_eq!(data.interests.audience_and_advertisers.lookalike_advertisers.len(), 2);
/// assert_eq!(data.interests.audience_and_advertisers.advertisers.len(), 0);
/// assert_eq!(data.interests.audience_and_advertisers.do_not_reach_advertisers.len(), 0);
/// assert_eq!(data.interests.audience_and_advertisers.catalog_audience_advertisers.len(), 0);
/// assert_eq!(data.interests.audience_and_advertisers.num_audiences, 0);
/// assert_eq!(data.interests.shows[0], "1899");
/// assert_eq!(data.interests.shows[1], "DuckTales");
///
/// assert_eq!(data.location_history.len(), 0);
///
/// assert_eq!(data.inferred_age_info.age[0], "13-99");
/// assert_eq!(data.inferred_age_info.birth_date, "");
///
/// // Re-serialize is equivalent to original data
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct P13nData {
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "demographics": {
	///     "languages": [
	///       {
	///         "language": "English",
	///         "isDisabled": false
	///       }
	///     ],
	///     "genderInfo": {
	///       "gender": "unknown",
	///       "genderOverride": "Borg"
	///     }
	///   }
	/// }
	/// ```
	pub demographics: Demographics,

	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "interests": {
	///     "interests": [
	///       {
	///         "name": "#HappyFriday",
	///         "isDisabled": false
	///       }
	///     ],
	///     "partnerInterests": [],
	///     "audienceAndAdvertisers": {
	///       "lookalikeAdvertisers": [
	///         "@EXAMPLE_ONE",
	///         "@EXAMPLE_TWO"
	///       ],
	///       "advertisers": [],
	///       "doNotReachAdvertisers": [],
	///       "catalogAudienceAdvertisers": [],
	///       "numAudiences": "0"
	///     },
	///     "shows": [
	///       "1899",
	///       "DuckTales"
	///     ]
	///   }
	/// }
	/// ```
	pub interests: Interests,

	/// WARNING: this type may be wrong!
	///
	/// ## Example JSON data
	///
	/// ```json
	/// { "locationHistory": [] }
	/// ```
	pub location_history: Vec<String>,

	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "inferredAgeInfo": {
	///     "age": [
	///       "13-99"
	///     ],
	///     "birthDate": ""
	///   }
	/// }
	/// ```
	pub inferred_age_info: InferredAgeInfo,
}

/// ## Example
///
/// ```
/// use twitter_archive::structs::personalization::Demographics;
///
/// let json = r#"{
///   "languages": [
///     {
///       "language": "English",
///       "isDisabled": false
///     }
///   ],
///   "genderInfo": {
///     "gender": "unknown",
///     "genderOverride": "Borg"
///   }
/// }"#;
///
/// let data: Demographics = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.languages[0].language, "English");
/// assert_eq!(data.languages[0].is_disabled, false);
/// assert_eq!(data.gender_info.gender, "unknown");
/// assert_eq!(data.gender_info.gender_override, "Borg");
///
/// // Re-serialize is equivalent to original data
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct Demographics {
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "languages": [
	///     {
	///       "language": "English",
	///       "isDisabled": false
	///     }
	///   ]
	/// }
	/// ```
	pub languages: Vec<LanguageEntry>,

	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "genderInfo": {
	///     "gender": "unknown",
	///     "genderOverride": "Borg"
	///   }
	/// }
	/// ```
	pub gender_info: GenderInfo,
}

/// ## Example
///
/// ```
/// use twitter_archive::structs::personalization::LanguageEntry;
///
/// let json = r#"{
///   "language": "English",
///   "isDisabled": false
/// }"#;
///
/// let data: LanguageEntry = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.language, "English");
/// assert_eq!(data.is_disabled, false);
///
/// // Re-serialize is equivalent to original data
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct LanguageEntry {
	/// ## Example JSON data
	///
	/// ```json
	/// { "language": "English" }
	/// ```
	pub language: String,

	/// ## Example JSON data
	///
	/// ```json
	/// { "isDisabled": false }
	/// ```
	pub is_disabled: bool,
}

/// ## Example
///
/// ```
/// use twitter_archive::structs::personalization::GenderInfo;
///
/// let json = r#"{
///   "gender": "unknown",
///   "genderOverride": "Borg"
/// }"#;
///
/// let data: GenderInfo = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.gender, "unknown");
/// assert_eq!(data.gender_override, "Borg");
///
/// // Re-serialize is equivalent to original data
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct GenderInfo {
	/// ## Example JSON data
	///
	/// ```json
	/// { "gender": "unknown" }
	/// ```
	pub gender: String,

	/// ## Example JSON data
	///
	/// ```json
	/// { "genderOverride": "Borg" }
	/// ```
	pub gender_override: String,
}

/// ## Example
///
/// ```
/// use twitter_archive::structs::personalization::Interests;
///
/// let json = r##"{
///   "interests": [
///     {
///       "name": "#HappyFriday",
///       "isDisabled": false
///     }
///   ],
///   "partnerInterests": [],
///   "audienceAndAdvertisers": {
///     "lookalikeAdvertisers": [
///       "@EXAMPLE_ONE",
///       "@EXAMPLE_TWO"
///     ],
///     "advertisers": [],
///     "doNotReachAdvertisers": [],
///     "catalogAudienceAdvertisers": [],
///     "numAudiences": "0"
///   },
///   "shows": [
///     "1899",
///     "DuckTales"
///   ]
/// }"##;
///
/// let data: Interests = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.interests[0].name, "#HappyFriday");
/// assert_eq!(data.interests[0].is_disabled, false);
/// assert_eq!(data.partner_interests.len(), 0);
/// assert_eq!(data.audience_and_advertisers.lookalike_advertisers.len(), 2);
/// assert_eq!(data.audience_and_advertisers.advertisers.len(), 0);
/// assert_eq!(data.audience_and_advertisers.do_not_reach_advertisers.len(), 0);
/// assert_eq!(data.audience_and_advertisers.catalog_audience_advertisers.len(), 0);
/// assert_eq!(data.audience_and_advertisers.num_audiences, 0);
/// assert_eq!(data.shows[0], "1899");
/// assert_eq!(data.shows[1], "DuckTales");
///
/// // Re-serialize is equivalent to original data
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct Interests {
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "interests": [
	///     {
	///       "name": "#HappyFriday",
	///       "isDisabled": false
	///     }
	///   ]
	/// }
	/// ```
	pub interests: Vec<Interest>,

	/// ## Example JSON data
	///
	/// ```json
	/// { "partnerInterests": [] }
	/// ```
	pub partner_interests: Vec<String>,

	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "audienceAndAdvertisers": {
	///     "lookalikeAdvertisers": [
	///       "@EXAMPLE_ONE",
	///       "@EXAMPLE_TWO"
	///     ],
	///     "advertisers": [],
	///     "doNotReachAdvertisers": [],
	///     "catalogAudienceAdvertisers": [],
	///     "numAudiences": "0"
	///   }
	/// }
	/// ```
	pub audience_and_advertisers: AudienceAndAdvertisers,

	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "shows": [
	///     "1899",
	///     "DuckTales"
	///   ]
	/// }
	/// ```
	pub shows: Vec<String>,
}

/// ## Example
///
/// ```
/// use twitter_archive::structs::personalization::Interest;
///
/// let json = r##"{
///   "name": "#HappyFriday",
///   "isDisabled": false
/// }"##;
///
/// let data: Interest = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.name, "#HappyFriday");
/// assert_eq!(data.is_disabled, false);
///
/// // Re-serialize is equivalent to original data
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct Interest {
	/// ## Example JSON data
	///
	/// ```json
	/// { "name": "#HappyFriday" }
	/// ```
	pub name: String,

	/// ## Example JSON data
	///
	/// ```json
	/// { "isDisabled": false }
	/// ```
	pub is_disabled: bool,
}

/// ## Example
///
/// ```
/// use twitter_archive::structs::personalization::AudienceAndAdvertisers;
///
/// let json = r#"{
///   "lookalikeAdvertisers": [
///     "@EXAMPLE_ONE",
///     "@EXAMPLE_TWO"
///   ],
///   "advertisers": [],
///   "doNotReachAdvertisers": [],
///   "catalogAudienceAdvertisers": [],
///   "numAudiences": "0"
/// }"#;
///
/// let data: AudienceAndAdvertisers = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.lookalike_advertisers[0], "@EXAMPLE_ONE");
/// assert_eq!(data.lookalike_advertisers[1], "@EXAMPLE_TWO");
/// assert_eq!(data.advertisers.len(), 0);
/// assert_eq!(data.do_not_reach_advertisers.len(), 0);
/// assert_eq!(data.catalog_audience_advertisers.len(), 0);
/// assert_eq!(data.num_audiences, 0);
///
/// // Re-serialize is equivalent to original data
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct AudienceAndAdvertisers {
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "lookalikeAdvertisers": [
	///     "@EXAMPLE_ONE",
	///     "@EXAMPLE_TWO"
	///   ]
	/// }
	/// ```
	pub lookalike_advertisers: Vec<String>,

	/// ## Example JSON data
	///
	/// ```json
	/// { "advertisers": [] }
	/// ```
	pub advertisers: Vec<String>,

	/// ## Example JSON data
	///
	/// ```json
	/// { "doNotReachAdvertisers": [] }
	/// ```
	pub do_not_reach_advertisers: Vec<String>,

	/// ## Example JSON data
	///
	/// ```json
	/// { "catalogAudienceAdvertisers": [] }
	/// ```
	pub catalog_audience_advertisers: Vec<String>,

	/// ## Example JSON data
	///
	/// ```json
	/// { "numAudiences": "0" }
	/// ```
	#[serde(with = "convert::number_like_string")]
	pub num_audiences: usize,
}

// TODO: find examples from which data structure(s) may be defined
// /// ## Example
// ///
// /// ```
// /// use twitter_archive::structs::personalization::LocationHistory;
// ///
// /// let json = r#"{ }"#;
// ///
// /// let data: LocationHistory = serde_json::from_str(&json).unwrap();
// ///
// /// // De-serialized properties
// /// // assert_eq!(data., "");
// ///
// /// // Re-serialize is equivalent to original data without pretty printing
// /// assert_eq!(serde_json::to_string(&data).unwrap(), json.replace("\n", "").replace(" ", ""));
// /// ```
// #[derive(Deserialize, Serialize, Debug, Clone, Display)]
// #[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
// #[serde(rename_all = "camelCase")]
// pub struct LocationHistory {
// 	todo!();
// }

/// ## Example
///
/// ```
/// use twitter_archive::structs::personalization::InferredAgeInfo;
///
/// let json = r#"{
///   "age": [
///     "13-99"
///   ],
///   "birthDate": ""
/// }"#;
///
/// let data: InferredAgeInfo = serde_json::from_str(&json).unwrap();
///
/// // De-serialized properties
/// assert_eq!(data.age[0], "13-99");
/// assert_eq!(data.birth_date, "");
///
/// // Re-serialize is equivalent to original data
/// assert_eq!(serde_json::to_string_pretty(&data).unwrap(), json);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Display)]
#[display(fmt = "{}", "serde_json::to_value(self).unwrap()")]
#[serde(rename_all = "camelCase")]
pub struct InferredAgeInfo {
	/// ## Example JSON data
	///
	/// ```json
	/// {
	///   "age": [
	///     "13-99"
	///   ]
	/// }
	/// ```
	pub age: Vec<String>,

	/// ## Example JSON data
	///
	/// ```json
	/// { "birthDate": "" }
	/// ```
	pub birth_date: String,
}
