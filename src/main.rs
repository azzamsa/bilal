use std::process;

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
    match opts.color {
        Color::Always => {
            owo_colors::set_override(true);
        }
        Color::Never => {
            owo_colors::set_override(false);
        }
        Color::Auto => {
            owo_colors::unset_override();
        }
    };

    // Never colorize JSON output
    if opts.json {
        owo_colors::set_override(false);
    }

    let config = config::read()?;
    let prayers = prayer::all(config.clone())?;
    let printer = Printer::new(prayers, opts.json, config);

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
