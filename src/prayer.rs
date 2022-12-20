use islam::pray::{Config, Location, Madhab, Method, PrayerSchedule, PrayerTimes};
use islam::time::OffsetDateTime;

use crate::config;
use crate::error::Error;

/// Returns all prayers
pub fn all() -> Result<PrayerTimes, Error> {
    let config = config::read()?;

    let timezone = config.timezone;
    let latitude = config.latitude;
    let longitude = config.longitude;
    let method = method(&config.method)?;
    let madhab = madhab(&config.madhab)?;

    let jakarta_city = Location::new(latitude, longitude, timezone);
    let today = OffsetDateTime::now_local()?.date();
    let conf = Config::new().with(method, madhab);
    let prayer_times = PrayerSchedule::new(jakarta_city)?
        .on(today)
        .with_config(conf)
        .calculate()?;

    Ok(prayer_times)
}

/// Get a method
fn method(method: &str) -> Result<Method, Error> {
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
        _ => Err(Error::InvalidMethod(method.to_string())),
    }
}

/// Get a madhab
fn madhab(madhab: &str) -> Result<Madhab, Error> {
    match madhab {
        "Shafi" => Ok(Madhab::Shafi),
        "Hanafi" => Ok(Madhab::Hanafi),
        _ => Err(Error::InvalidMadhab(madhab.to_string())),
    }
}
