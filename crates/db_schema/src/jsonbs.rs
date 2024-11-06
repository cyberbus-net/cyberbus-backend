use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use ts_rs::TS; 
use diesel_as_jsonb::AsJsonb;

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

impl Trophy {
    /// Creates a new trophy with the given name and current timestamp
    pub fn new(name: String) -> Self {
        Self {
            name,
            rewarded_at: Some(Utc::now()),
        }
    }
}

pub fn new_trophy_case_by_trophy_names(trophie_names: Vec<String>) -> TrophyCase {
    let new_trophy_case = TrophyCase {
        trophies: Some(
            trophie_names
                .iter()
                .map(|trophie_names| Trophy::new(trophie_names.to_string()))
                .collect(),
        ),
    };
    new_trophy_case
}
