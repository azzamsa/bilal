mod config;
mod util;

use clap::{App, AppSettings, Arg};
use colored::*;
use indexmap::IndexMap;
use salah::prelude::*;

use util::to_local;

fn main() {
    let matches = App::new("Bilal [A CLI salah time]")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::ColoredHelp)
        .version("0.1.0")
        .arg(
            Arg::new("next")
                .short('n')
                .long("next")
                .about("Show next Salah"),
        )
        .arg(
            Arg::new("current")
                .short('c')
                .long("current")
                .about("Show current Salah"),
        )
        .arg(
            Arg::new("all")
                .short('a')
                .long("all")
                .about("Show all Salah time"),
        )
        .get_matches();

    if matches.is_present("all") {
        show_all_salah();
    }
    if matches.is_present("current") {
        show_current_salah();
    }
    if matches.is_present("next") {
        show_next_salah();
    }
}

fn get_prayers_time(latitude: f64, longitude: f64) -> Result<salah::PrayerTimes, String> {
    let city = Coordinates::new(latitude, longitude);
    let date = Utc::today();
    let params = Configuration::with(Method::Singapore, Madhab::Shafi);
    let prayers = PrayerSchedule::new()
        .on(date)
        .for_location(city)
        .with_configuration(params)
        .calculate();
    return prayers;
}

fn show_all_salah() {
    let config = config::get_config();
    let mut prayers = IndexMap::new();
    let prayers_time = get_prayers_time(config["latitude"], config["longitude"]);

    match prayers_time {
        Ok(prayer) => {
            prayers.insert(Prayer::Fajr.name(), to_local(prayer.time(Prayer::Fajr)));
            prayers.insert(
                Prayer::Sunrise.name(),
                to_local(prayer.time(Prayer::Sunrise)),
            );
            prayers.insert(Prayer::Dhuhr.name(), to_local(prayer.time(Prayer::Dhuhr)));
            prayers.insert(Prayer::Asr.name(), to_local(prayer.time(Prayer::Asr)));
            prayers.insert(
                Prayer::Maghrib.name(),
                to_local(prayer.time(Prayer::Maghrib)),
            );
            prayers.insert(Prayer::Isha.name(), to_local(prayer.time(Prayer::Isha)));
            prayers.insert(Prayer::Qiyam.name(), to_local(prayer.time(Prayer::Qiyam)));
        }
        Err(error) => println!("Could not calculate prayer times: {}", error),
    }

    for prayer in prayers {
        println!("{}: {}", prayer.0, prayer.1.format("%-l:%M %p"));
    }
}

fn show_current_salah() {
    let config = config::get_config();
    let prayers_time = get_prayers_time(config["latitude"], config["longitude"]);

    match prayers_time {
        Ok(prayer) => {
            let (hours, minutes) = prayer.time_remaining();
            if minutes < 30 {
                println!(
                    "{}",
                    format!("{} ({}:{})", prayer.current().name(), hours, minutes).red()
                );
            } else {
                println!("{} ({}:{})", prayer.current().name(), hours, minutes);
            }
        }
        Err(error) => println!("Could not calculate prayer times: {}", error),
    }
}

fn show_next_salah() {
    let config = config::get_config();
    let prayers_time = get_prayers_time(config["latitude"], config["longitude"]);

    match prayers_time {
        Ok(prayer) => {
            println!(
                "{} ({})",
                prayer.next().name(),
                to_local(prayer.time(prayer.next()))
                    .format("%-l:%M %p")
                    .to_string()
            );
        }
        Err(error) => println!("Could not calculate prayer times: {}", error),
    }
}
