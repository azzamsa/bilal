mod config;
mod prayer;
mod util;

use clap::{App, AppSettings, Arg};
use colored::*;

fn main() {
    let mut _is_json: bool = false;
    let matches = App::new("Bilal [A CLI salah time]")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::ColoredHelp)
        .version("0.1.0")
        .arg(
            Arg::new("next")
                .short('n')
                .long("next")
                .about("Show next Salah"),
        )
        .arg(
            Arg::new("current")
                .short('c')
                .long("current")
                .about("Show current Salah"),
        )
        .arg(
            Arg::new("all")
                .short('a')
                .long("all")
                .about("Show all Salah time"),
        )
        .arg(
            Arg::new("json")
                .short('j')
                .long("json")
                .about("Display salah in json formatted string"),
        )
        .get_matches();

    if matches.is_present("json") {
        _is_json = true;
    } else {
        _is_json = false;
    }

    if matches.is_present("all") {
        show_all_prayers();
    }
    if matches.is_present("current") {
        show_current_prayer(_is_json);
    }
    if matches.is_present("next") {
        show_next_prayer(_is_json);
    }
}

fn show_all_prayers() {
    let prayers = prayer::get_all_prayers();
    for prayer in prayers {
        println!(
            "{}",
            format!("{}: {}", prayer.0, prayer.1.format("%-l:%M %p")),
        )
    }
}

fn show_current_prayer(is_json: bool) {
    let current_prayer = prayer::get_current_prayer().unwrap();
    let (hours, minutes) = current_prayer.time_remaining();

    let _current_prayer_fmt = format!(
        "{} ({}:{})",
        current_prayer.current().name(),
        hours,
        minutes
    );

    if is_json {
        if minutes < 30 && hours == 0 {
            println!(
                "{}",
                format!(r#"{{state:"Critical", "text": "{}"}}"#, _current_prayer_fmt)
            );
        } else {
            println!(
                "{}",
                format!(r#"{{state:"info", "text": "{}"}}"#, _current_prayer_fmt),
            );
        }
    } else {
        if minutes < 30 && hours == 0 {
            println!("{}", format!("{}", _current_prayer_fmt.red()));
        } else {
            println!("{}", format!("{}", _current_prayer_fmt));
        }
    }
}

fn show_next_prayer(is_json: bool) {
    let (prayer_name, time) = prayer::get_next_prayer().unwrap();
    if is_json {
        let prayer_fmt = format!("{} ({})", prayer_name, time.format("%-l:%M %p").to_string());
        println!(r#"{{state:"info", "text": "{}"}}"#, prayer_fmt);
    } else {
        println!("{} ({})", prayer_name, time.format("%-l:%M %p").to_string());
    }
}
