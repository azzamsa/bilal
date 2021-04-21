use std::io::{self, Write};

use colored::Colorize;
use islam::chrono::{DateTime, Local, Timelike};

use crate::error::BilalError;
use islam::pray::PrayerTimes;

pub struct Printer {
    prayers: PrayerTimes,
    show_color: bool,
    json_format: bool,
}

impl Printer {
    pub const fn new(prayers: PrayerTimes, show_color: bool, json_format: bool) -> Self {
        Self {
            prayers,
            show_color,
            json_format,
        }
    }
    /// Show all prayers info.
    pub fn all(&self) -> Result<(), BilalError> {
        let prayers = self.prayers;

        let fmt_output = |name: &str, prayer: DateTime<Local>| {
            format!("{}: {}:{}", name, prayer.hour(), prayer.minute())
        };

        Self::print(&fmt_output("Fajr", prayers.fajr));
        Self::print(&fmt_output("Sherook", prayers.sherook));
        Self::print(&fmt_output("Dohr", prayers.dohr));
        Self::print(&fmt_output("Asr", prayers.asr));
        Self::print(&fmt_output("Mghreb", prayers.maghreb));
        Self::print(&fmt_output("Ishaa", prayers.ishaa));
        Self::print(&fmt_output(
            "Fist third of night",
            prayers.first_third_of_night,
        ));
        Self::print(&fmt_output("Midnight", prayers.midnight));
        Self::print(&fmt_output(
            "Last third of night",
            prayers.last_third_of_night,
        ));

        Ok(())
    }
    /// Show current prayer info
    pub fn current(&self) -> Result<(), BilalError> {
        let prayers = self.prayers;
        let prayer = prayers.current();
        let (hour, minute) = prayers.time_remaining();

        let remaining_fmt = {
            if hour == 0 {
                format!("({} minutes)", minute)
            } else {
                format!("({}:{} hours)", hour, minute)
            }
        };

        // default
        let mut prayer_fmt = format!("{} {}", prayer.name(), remaining_fmt);
        let state = {
            if minute < 30 {
                "Critical"
            } else {
                "Info"
            }
        };

        // JSON
        if self.json_format {
            prayer_fmt = format!(
                r#"{{"icon": "{}", "state": "{}", "text": "{} {}"}}"#,
                "bilal", state, "\u{23fa} ", prayer_fmt
            )
        }
        // color
        if self.show_color && state == "Critical" && !self.json_format {
            prayer_fmt = format!("{}", prayer_fmt.red());
        }
        Self::print(&prayer_fmt);
        Ok(())
    }
    /// Show next prayer info
    pub fn next(&self) -> Result<(), BilalError> {
        let prayers = self.prayers;
        let prayer = prayers.next();
        let time = prayers.time(prayer);

        let time_fmt = format!("({}:{})", time.hour(), time.minute());

        // default
        let mut prayer_fmt = format!("{} {}", prayer.name(), time_fmt);
        // JSON
        let state = "Info";
        if self.json_format {
            prayer_fmt = format!(
                r#"{{"icon": "{}", "state": "{}", "text": "{} {}"}}"#,
                "bilal", state, "\u{25b6}", prayer_fmt
            )
        }
        Self::print(&prayer_fmt);
        Ok(())
    }
    fn print(prayer_fmt: &str) {
        writeln!(io::stdout(), "{}", prayer_fmt).ok();
    }
}
