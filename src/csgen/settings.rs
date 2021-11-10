use crate::csgen::window::Window;
use chrono::{DateTime, Local, NaiveTime, Weekday};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashSet;

// Workaround: Use NaiveTime for Duration and DateTime<Local> for Date<Local> because these
// structures does not yet support serialization/deserialization yet.
// https://github.com/chronotope/chrono/issues/117
// The conversion method is as follows:
// Duration: by the offset of the NaiveTime from NaiveTime::from_hms(0, 0, 0).
// Date<Local>: by DateTime<Local> with hours, minutes, and seconds unused.
#[derive(Serialize, Deserialize)]
pub struct GenerationSettings {
    pub(crate) start: DateTime<Local>,
    pub(crate) end: DateTime<Local>,

    #[serde(default = "default_window")]
    pub(crate) window: Window,

    #[serde(default = "default_holiday_window")]
    pub(crate) holiday_window: Window,

    #[serde(default = "default_step")]
    pub(crate) step: NaiveTime,

    #[serde(default = "default_holidays")]
    pub(crate) holidays: Vec<DateTime<Local>>,

    #[serde(default = "default_holiday_weekdays")]
    pub(crate) holiday_weekdays: HashSet<Weekday>,
}

impl Default for GenerationSettings {
    fn default() -> Self {
        GenerationSettings {
            start: Local::now(),
            end: Local::now(),
            window: default_window(),
            holiday_window: default_holiday_window(),
            step: default_step(),
            holiday_weekdays: default_holiday_weekdays(),
            holidays: default_holidays(),
        }
    }
}

fn default_window() -> Window {
    Window {
        start: NaiveTime::from_hms(0, 0, 0),
        end: NaiveTime::from_hms(23, 59, 59),
    }
}

fn default_holiday_window() -> Window {
    Window {
        start: NaiveTime::from_hms(0, 0, 0),
        end: NaiveTime::from_hms(23, 59, 59),
    }
}

fn default_step() -> NaiveTime {
    NaiveTime::from_hms(1, 0, 0)
}

fn default_holiday_weekdays() -> HashSet<Weekday> {
    HashSet::from([Weekday::Sat, Weekday::Sun])
}

fn default_holidays() -> Vec<DateTime<Local>> {
    Vec::new()
}
