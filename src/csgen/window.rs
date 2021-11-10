use chrono::NaiveTime;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Window {
    pub(crate) start: NaiveTime,
    pub(crate) end: NaiveTime,
}
