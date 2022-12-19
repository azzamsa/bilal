use std::process;

use anyhow::Result;
use atty::Stream;
use clap::Parser;

use bilal::{
    cli::{Color, Mode, Opts},
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

    let prayers = prayer::all()?;
    let printer = Printer::new(prayers, show_color, opts.json);

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
