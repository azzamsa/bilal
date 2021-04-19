use clap::{crate_version, App, AppSettings, Arg};

/// Build a clap app
pub fn build() -> App<'static> {
    let app = App::new("Bilal [A CLI salah time]")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::ColoredHelp)
        .version(crate_version!())
        .arg(
            Arg::new("salah")
                .possible_values(&["all", "next", "current"])
                .default_value("all")
                .takes_value(true)
                .about("A Salah to show"),
        )
        .arg(
            Arg::new("json")
                .short('J')
                .long("json")
                .about("Display Salah in JSON formatted string"),
        )
        .arg(
            Arg::new("color")
                .long("color")
                .takes_value(true)
                .value_name("WHEN")
                .possible_values(&["always", "auto", "never"])
                .default_value("always")
                .about("Display Salah in colored output"),
        );
    app
}
