use clap::{crate_version, Arg, Command};

/// Build a clap app
pub fn build() -> Command<'static> {
    let app = Command::new("Bilal [A CLI salah time]")
        .arg_required_else_help(true)
        .version(crate_version!())
        .arg(
            Arg::new("salah")
                .possible_values(&["all", "next", "current"])
                .default_value("all")
                .hide_possible_values(true)
                .takes_value(true)
                .help("A Salah to show"),
        )
        .arg(
            Arg::new("json")
                .short('J')
                .long("json")
                .help("Display Salah in JSON formatted string"),
        )
        .arg(
            Arg::new("color")
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
