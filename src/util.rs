/// Wrap a prayer info into JSON representation.
pub fn to_json(state: String, content: String) -> String {
    if state == "" {
        format!(r#"{{"text": "{}"}}"#, content)
    } else {
        format!(r#"{{"state":"{}", "text": "{}"}}"#, state, content)
    }
}

/// Format prayer time object into string format.
pub fn fmt_time(time: salah::DateTime<salah::Local>) -> String {
    time.format("%-l:%M %p").to_string()
}

/// Format duration
pub fn fmt_duration(duration: salah::Duration) -> String {
    format!(
        "{}:{:02}",
        duration.num_hours(),
        duration.num_minutes() % 60
    )
}
