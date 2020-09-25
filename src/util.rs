use salah::prelude::*;

/// Convert UTC time into local.
pub fn to_local(time: salah::DateTime<salah::Utc>) -> salah::DateTime<salah::Local> {
    DateTime::<Local>::from(time)
}

/// Wrap a prayer info into JSON representation.
pub fn to_json(state: String, content: String) -> String {
    format!(r#"{{state:"{}", "text": "{}"}}"#, state, content)
}

/// Format prayer time object into string format.
pub fn fmt_time(time: salah::DateTime<salah::Local>) -> String {
    time.format("%-l:%M %p").to_string()
}
