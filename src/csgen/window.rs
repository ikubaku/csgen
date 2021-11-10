use serde_derive::{Serialize, Deserialize};
use chrono::NaiveTime;

#[derive(Serialize, Deserialize)]
pub struct Window {
    pub(crate) start: NaiveTime,
    pub(crate) end: NaiveTime,
}
