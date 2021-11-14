use clap::{crate_version, App, AppSettings, Arg};

/// Build a clap app
pub fn build() -> App<'static, 'static> {
    let app = App::new("Bilal [A CLI salah time]")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::ColoredHelp)
        .version(crate_version!())
        .arg(
            Arg::with_name("salah")
                .possible_values(&["all", "next", "current"])
                .default_value("all")
                .hide_possible_values(true)
                .takes_value(true)
                .help("A Salah to show"),
        )
        .arg(
            Arg::with_name("json")
                .short("J")
                .long("json")
                .help("Display Salah in JSON formatted string"),
        )
        .arg(
            Arg::with_name("color")
                .long("color")
                .value_name("WHEN")
                .possible_values(&["always", "auto", "never"])
                .default_value("always")
                .hide_possible_values(true)
                .takes_value(true)
                .help("Display Salah in colored output"),
        );
    app
}
