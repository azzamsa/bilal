use indexmap::IndexMap;
use salah::{Coordinates, DateTime, Local, Parameters, Prayer, PrayerTimes};

use crate::config;

/// Setup a prayer config, and get all its time.
fn get_prayers_time(config: config::Config) -> PrayerTimes {
    let city = Coordinates::new(config.latitude, config.longitude);
    let date = Local::today();
    let params = Parameters::with(config.method, config.madhab);

    PrayerTimes::calculate(date, city, params)
}

/// Returns all prayers.
pub fn get_all_prayers() -> IndexMap<String, salah::DateTime<salah::Local>> {
    let mut prayers = IndexMap::new();
    let prayer = get_prayers_time(config::get_config());

    prayers.insert(Prayer::Fajr.name(), prayer.time(Prayer::Fajr));
    prayers.insert(Prayer::Sunrise.name(), prayer.time(Prayer::Sunrise));
    prayers.insert(Prayer::Dhuhr.name(), prayer.time(Prayer::Dhuhr));
    prayers.insert(Prayer::Asr.name(), prayer.time(Prayer::Asr));
    prayers.insert(Prayer::Maghrib.name(), prayer.time(Prayer::Maghrib));
    prayers.insert(Prayer::Isha.name(), prayer.time(Prayer::Isha));
    prayers.insert(Prayer::Qiyam.name(), prayer.time(Prayer::Qiyam));

    prayers
}

/// Returns current prayer.
pub fn get_current_prayer() -> PrayerTimes {
    get_prayers_time(config::get_config())
}

/// Returns current prayer.
pub fn get_next_prayer() -> (String, DateTime<Local>) {
    let prayer = get_prayers_time(config::get_config());

    (prayer.next().name(), prayer.time(prayer.next()))
}
