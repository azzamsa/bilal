use indexmap::IndexMap;
use salah::prelude::*;

use crate::config;
use crate::util::to_local;

/// Setup a prayer config, and get all its time.
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

/// Returns all prayers.
pub fn get_all_prayers() -> IndexMap<String, salah::DateTime<salah::Local>> {
    let config = config::get_config();
    let mut prayers = IndexMap::new();
    let prayers_time = get_prayers_time(config.latitude, config.longitude);

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
pub fn get_current_prayer() -> Result<salah::PrayerTimes, String> {
    let config = config::get_config();
    let prayers_time = get_prayers_time(config.latitude, config.longitude);

    match prayers_time {
        Ok(prayer) => Ok(prayer),
        Err(error) => Err(format!("Could not calculate prayer times: {}", error)),
    }
}

/// Returns current prayer.
pub fn get_next_prayer() -> Result<(String, salah::DateTime<salah::Local>), String> {
    let config = config::get_config();
    let prayers_time = get_prayers_time(config.latitude, config.longitude);

    match prayers_time {
        Ok(prayer) => Ok((prayer.next().name(), to_local(prayer.time(prayer.next())))),
        Err(error) => Err(format!("Could not calculate prayer times: {}", error)),
    }
}
