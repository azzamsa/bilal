use std::io::{self, Write};

use chrono::prelude::*;
use colored::Colorize;

use crate::error::BilalError;
use islam::pray::times::PrayerTimes;

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

        let print_output = |name: &str, prayer: DateTime<Local>| {
            writeln!(
                io::stdout(),
                "{}: {}:{}:{}",
                name,
                prayer.hour(),
                prayer.minute(),
                prayer.second()
            )
            .ok();
        };

        print_output("Fajr", prayers.fajr);
        print_output("Sherook", prayers.sherook);
        print_output("Dohr", prayers.dohr);
        print_output("Asr", prayers.asr);
        print_output("Mghreb", prayers.maghreb);
        print_output("Ishaa", prayers.ishaa);
        print_output("Fist third of night", prayers.first_third_of_night);
        print_output("Midnight", prayers.midnight);
        print_output("Last third of night", prayers.last_third_of_night);

        Ok(())
    }
    /// Show current prayer info
    pub fn current(&self) -> Result<(), BilalError> {
        let prayers = self.prayers;
        let prayer = prayers.current();
        let (hour, minute) = prayers.time_remaining();

        let remaining_fmt = {
            if !hour == 0 {
                format!("({}:{} hours)", hour, minute)
            } else {
                format!("({} minutes)", minute)
            }
        };

        // default
        let mut prayer_fmt = format!("\u{23fa} {} {}", prayer.name(), remaining_fmt);
        let state = {
            if minute < 30 {
                "Critical"
            } else {
                "Info"
            }
        };

        // JSON
        if self.json_format {
            prayer_fmt = format!(r#"{{"state":"{}", "text": "{}"}}"#, state, prayer_fmt)
        }
        // color
        if self.show_color && state == "Critical" {
            prayer_fmt = format!("{}", prayer_fmt.red());
        }

        writeln!(io::stdout(), "{}", prayer_fmt).ok();

        Ok(())
    }
    /// Show next prayer info
    pub fn next(&self) -> Result<(), BilalError> {
        let prayers = self.prayers;
        let prayer = prayers.next();
        let time = prayers.time(prayer);

        let time_fmt = format!("({}:{}:{})", time.hour(), time.minute(), time.second());

        // default
        let mut prayer_fmt = format!("\u{25b6} {} {}", prayer.name(), time_fmt);
        // JSON
        let state = "Info";
        if self.json_format {
            prayer_fmt = format!(r#"{{"state":"{}", "text": "{}"}}"#, state, prayer_fmt)
        }
        writeln!(io::stdout(), "{}", prayer_fmt).ok();

        Ok(())
    }
}
