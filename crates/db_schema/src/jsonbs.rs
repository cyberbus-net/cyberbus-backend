use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use ts_rs::TS; 
use diesel_as_jsonb::AsJsonb;

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize, TS, AsJsonb)]
pub struct PersonConfig {
    pub invite_code: Option<String>,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize, TS, AsJsonb)]
#[cfg_attr(feature = "full", ts(export))]
pub struct TrophyCase {
    pub trophies: Option<Vec<Trophy>>,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize, TS, AsJsonb)]
#[cfg_attr(feature = "full", ts(export))]
/// The feature type for a local_user.
pub struct Trophy {
  /// Name of Trophy.
  pub name: String,
  /// Time of trophy get.
  pub rewarded_at: Option<DateTime<Utc>>,
}

pub fn new_trophy_case_by_trophy_names(trophie_names: Vec<String>) -> TrophyCase {
    let new_trophy_case = TrophyCase {
        trophies: Some(
            trophie_names
                .iter()
                .map(|trophie_names| Trophy {
                    name: trophie_names.to_string(),
                    rewarded_at: Some(Utc::now()),
                })
                .collect(),
        ),
    };
    new_trophy_case
}
