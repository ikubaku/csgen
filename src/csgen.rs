use crate::csgen::settings::GenerationSettings;
use chrono::{DateTime, Datelike, Duration, Local, NaiveTime, Timelike};

pub mod settings;
pub mod window;

pub struct CsGen {
    settings: GenerationSettings,
}

impl CsGen {
    pub fn from_settings(settings: GenerationSettings) -> Self {
        CsGen { settings }
    }

    pub fn generate(&self) {
        let mut current_datetime = DateTime::from(self.settings.start);
        while current_datetime < self.settings.end {
            self.generate_for_date(current_datetime);
            current_datetime = current_datetime + Duration::days(1);
        }
    }

    fn generate_for_date(&self, date: DateTime<Local>) {
        let window = if self.the_date_is_holiday(date) {
            &self.settings.holiday_window
        } else {
            &self.settings.window
        };

        let mut current_datetime = date
            .with_hour(window.start.hour())
            .unwrap()
            .with_minute(window.start.minute())
            .unwrap()
            .with_second(window.start.second())
            .unwrap();
        let end_datetime = date
            .with_hour(window.end.hour())
            .unwrap()
            .with_minute(window.end.minute())
            .unwrap()
            .with_second(window.end.second())
            .unwrap();

        while current_datetime < end_datetime {
            println!("{}", current_datetime.format("%m/%d (%a) %R-"));
            current_datetime =
                current_datetime + (self.settings.step - NaiveTime::from_hms(0, 0, 0));
        }
    }

    fn the_date_is_holiday(&self, date: DateTime<Local>) -> bool {
        if self.settings.holiday_weekdays.contains(&date.weekday()) {
            return true;
        } else {
            for holiday_datetime in &self.settings.holidays {
                let holiday_date = holiday_datetime.date();
                if holiday_date == date.date() {
                    return true;
                }
            }
            return false;
        }
    }
}
