#![allow(clippy::wildcard_imports)]

use std::env;
use std::process;

use anyhow::Result;
use atty::Stream;

use bilal::app;
use bilal::output::Printer;
use bilal::prayer;

fn run() -> Result<()> {
    let matches = app::build().get_matches_from(env::args_os());

    let show_color = match matches.value_of("color") {
        Some("never") => false,
        Some("auto") => atty::is(Stream::Stdout),
        _ => true,
    };
    let json_format = matches.is_present("json");

    let prayers = prayer::all()?;
    let printer = Printer::new(prayers, show_color, json_format);

    if matches.is_present("all") {
        printer.all()?;
    };
    if matches.is_present("current") {
        printer.current()?;
    }
    if matches.is_present("next") {
        printer.next()?;
    }

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {:?}", err);
        process::exit(1);
    }
}
