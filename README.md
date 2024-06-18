<div align="center">
<h1>Bilal</h1>

<a href="https://github.com/azzamsa/bilal/actions/workflows/ci.yml">
<img src="https://github.com/azzamsa/bilal/actions/workflows/ci.yml/badge.svg">
</a>
<a href="https://crates.io/crates/bilal">
<img src="https://img.shields.io/crates/v/bilal.svg">
</a>
<a href="https://docs.rs/bilal/">
<img src="https://docs.rs/bilal/badge.svg">
</a>
<a href="https://azzamsa.com/support/"><img alt="Sponsor me" src="https://img.shields.io/badge/Sponsor%20Me-%F0%9F%92%96-ff69b4">
</a>
<p></p>

![demo](docs/demo.gif)

</div>

---

**bilal** is a CLI salah time.

## Features

- Show all salah time in current day
- Show current salah time and its remaining time
- Show next salah time
- Time Format (12/24-Hour)
- Fancy error message

## Usage

```bash
bilal all                            Show all salahs time
bilal current                        ... current salah time
bilal current --json                  .... with JSON format
```

To configure Bilal, first create a file named `config.toml` in `~/.config/bilal/` on Unix-like systems. On Windows, place it under `\AppData\Bilal\`. If you prefer a custom location, simply set the `BILAL_CONFIG` environment variable.

Next, add your configuration details to the `config.toml` file:

```toml
latitude = -6.18233995
longitude = 106.84287154
madhab = "Shafi"
method = "Egyptian"
```

To see more options, please read the [wiki](docs/wiki.md)

## Integration with other programs

You can use Bilal with `i3status-rust` to show salah time in your status.

![i3status-rust-bilal](docs/i3status-rust.png)

![i3status-rust-bilal-urgent](docs/i3status-rust-urgent.png)

`i3status-rust` configuration Example:

```bash
[[block]]
block = "custom"
cycle = [
        "bilal current -J",
        "bilal next -J",
        ]
on_click = "<command>"
interval = 300
json = true
```

See [more examples](examples/) to learn other variations.

If you like `bilal` to support your favourite status-bar, please open new issue
with the valid input of your status-bar. In i3status-rust
the valid input it would be `{"icon": "ICON", "state": "STATE", "text": "YOURTEXT"}`.

## Installation

### From binaries

The [release page](https://github.com/azzamsa/bilal/releases) includes
pre-compiled binaries for GNU/Linux, macOS and Windows.

### From source

Using [cargo-binstall](https://github.com/cargo-bins/cargo-binstall)

```bash
cargo binstall bilal
```

Using Rust's package manager [cargo](https://github.com/rust-lang/cargo):

```bash
cargo install bilal
```

## Development

```bash
git clone https://github.com/azzamsa/bilal

# Build
cd bilal
cargo build

# Run unit tests and integration tests
cargo test

# Install
cargo install --path .
```

## Contributing

To learn more read [contributing.md](docs/dev/contributing.md)

## Origin of the name

The name Bilal was chosen in reference to the Bilal bin Rabah. The first
mu'azzin, chosen by Muhammad PBUH himself.

## License

Copyright (c) 2020-2024 azzamsa

Bilal is distributed under the terms of [GPL V3 License](LICENSE).
