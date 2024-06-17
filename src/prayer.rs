use islam::salah::{Config as SalahConfig, Location, Madhab, Method, PrayerSchedule, PrayerTimes};

use crate::{config::Config, error::Error};

/// Returns all prayers
pub fn all(config: Config) -> Result<PrayerTimes, Error> {
    let latitude = config.latitude;
    let longitude = config.longitude;
    let method = method(&config.method)?;
    let madhab = madhab(&config.madhab)?;

    let jakarta_city = Location::new(latitude, longitude);
    let conf = SalahConfig::new().with(method, madhab);
    let prayer_times = PrayerSchedule::new(jakarta_city)?
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
