use serde::{Deserialize, Serialize};
use ts_rs::TS; 
use diesel_as_jsonb::AsJsonb;

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize, TS, AsJsonb)]
pub struct PersonConfig {
    pub invite_code: Option<String>,
}
