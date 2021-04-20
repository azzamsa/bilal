use chrono::Local;
use islam::pray::Config;
use islam::pray::Madhab;
use islam::pray::Method;
use islam::pray::{Location, PrayerSchedule, PrayerTimes};

use crate::config;
use crate::error::BilalError;

/// Get a method
fn method(method: &str) -> Result<Method, BilalError> {
    match method {
        "Karachi" => Ok(Method::Karachi),
        "MuslimWorldLeague" => Ok(Method::MuslimWorldLeague),
        "Egyptian" => Ok(Method::Egyptian),
        "UmmAlQura" => Ok(Method::UmmAlQura),
        "NorthAmerica" => Ok(Method::NorthAmerica),
        "French" => Ok(Method::French),
        "Singapore" => Ok(Method::Singapore),
        "Russia" => Ok(Method::Russia),
        "FixedInterval" => Ok(Method::FixedInterval),
        _ => Err(BilalError::InvalidMethod(method.to_string())),
    }
}

/// Get a madhab
fn madhab(madhab: &str) -> Result<Madhab, BilalError> {
    match madhab {
        "Shafi" => Ok(Madhab::Shafi),
        "Hanafi" => Ok(Madhab::Hanafi),
        _ => Err(BilalError::InvalidMadhab(madhab.to_string())),
    }
}

/// Returns all prayers
pub fn all() -> Result<PrayerTimes, BilalError> {
    let config = config::get()?;

    let timezone = config.timezone;
    let latitude = config.latitude;
    let longitude = config.longitude;
    let method = method(&config.method)?;
    let madhab = madhab(&config.madhab)?;

    let jakarta_city = Location::new(latitude, longitude, timezone);
    let date = Local::today();
    let conf = Config::new().with(method, madhab);
    let prayer_times = PrayerSchedule::new(jakarta_city)
        .on(date)
        .with_config(conf)
        .calculate();

    Ok(prayer_times)
}
