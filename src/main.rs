use std::process;

use atty::Stream;
use clap::Parser;
use miette::Result;

use bilal::{
    cli::{Color, Mode, Opts},
    config,
    output::Printer,
    prayer,
};

fn run() -> Result<()> {
    let opts = Opts::parse();
    let show_color = match opts.color {
        Color::Always => true,
        Color::Never => false,
        Color::Auto => atty::is(Stream::Stdout),
    };

    let config = config::read()?;
    let prayers = prayer::all(config.clone())?;
    let printer = Printer::new(prayers, show_color, opts.json, config);

    match opts.mode {
        Mode::All => {
            printer.all()?;
        }
        Mode::Current => {
            printer.current()?;
        }
        Mode::Next => {
            printer.next()?;
        }
    }

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {:?}", err);
        process::exit(1);
    }
}
