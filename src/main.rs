mod config;
mod util;

use clap::{App, Arg};
use indexmap::IndexMap;
use salah::prelude::*;

use util::to_local;

fn main() {
    let matches = App::new("Salah Time")
        .version("0.1.0")
        .arg(
            Arg::new("remaining")
                .short('r')
                .long("remaining")
                .about("Show remaining time of next Salah"),
        )
        .arg(
            Arg::new("exact")
                .short('e')
                .long("exact")
                .about("Show exact time of next Salah"),
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
    if matches.is_present("exact") {
        show_current_salah();
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
            println!(
                "Current: {} ({})",
                prayer.current().name(),
                to_local(prayer.time(prayer.current())).format("%-l:%M %p").to_string()
            );
        }
        Err(error) => println!("Could not calculate prayer times: {}", error),
    }
}
