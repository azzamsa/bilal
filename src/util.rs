use salah::prelude::*;

pub fn to_local(time: salah::DateTime<salah::Utc>) -> salah::DateTime<salah::Local> {
    DateTime::<Local>::from(time)
}

pub fn to_json(state: String, content: String) -> String {
    format!(r#"{{state:"{}", "text": "{}"}}"#, state, content)
}

pub fn fmt_time(time: salah::DateTime<salah::Local>) -> String {
    time.format("%-l:%M %p").to_string()
}
