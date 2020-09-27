use indexmap::IndexMap;
use salah::prelude::{Configuration, Coordinates, Prayer, PrayerSchedule, PrayerTimes};

use crate::config;
use crate::util::to_local;

/// Setup a prayer config, and get all its time.
fn get_prayers_time(config: config::Config) -> Result<salah::PrayerTimes, String> {
    let city = Coordinates::new(config.latitude, config.longitude);
    let date = salah::Utc::today();
    let params = Configuration::with(config.method, config.madhab);
    let prayers = PrayerSchedule::new()
        .on(date)
        .for_location(city)
        .with_configuration(params)
        .calculate();
    return prayers;
}

/// Returns all prayers.
pub fn get_all_prayers() -> IndexMap<String, salah::DateTime<salah::Local>> {
    let mut prayers = IndexMap::new();
    let prayers_time = get_prayers_time(config::get_config());

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

    return prayers;
}

/// Returns current prayer.
pub fn get_current_prayer() -> Result<PrayerTimes, String> {
    let prayers_time = get_prayers_time(config::get_config());

    match prayers_time {
        Ok(prayer) => Ok(prayer),
        Err(error) => Err(format!("Could not calculate prayer times: {}", error)),
    }
}

/// Returns current prayer.
pub fn get_next_prayer() -> Result<(String, salah::DateTime<salah::Local>), String> {
    let prayers_time = get_prayers_time(config::get_config());

    match prayers_time {
        Ok(prayer) => Ok((prayer.next().name(), to_local(prayer.time(prayer.next())))),
        Err(error) => Err(format!("Could not calculate prayer times: {}", error)),
    }
}
