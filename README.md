<div align="center">
<h1>Bilal</h1>

<a href="https://builds.sr.ht/~azzamsa/bilal?">
<img src="https://builds.sr.ht/~azzamsa/bilal.svg">
</a>
<a href="https://crates.io/crates/bilal">
<img src="https://img.shields.io/crates/v/bilal.svg">
</a>
<a href=" https://docs.rs/bilal/">
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

## Usage

Create a file named `bilal.toml` in `~/.config/bilal/`. If you are on `Windows`, put it under `\AppData\Bilal\`. Then add your configuration:

``` toml
timezone = 7
latitude = -6.18233995
longitude = 106.84287154
madhab = "Shafi"
method = "Egyptian"
```

You can get latitude and longitude value from [mapcoordinates](https://www.mapcoordinates.net/en).
Other [madhab](https://docs.rs/islam/0.1.1/islam/pray/madhab/enum.Madhab.html#variants)
and [method](https://docs.rs/islam/0.1.1/islam/pray/method/enum.Method.html#variants) options are available.

## Usage with other tools

You can use Bilal with `i3status-rs` to show salah time in your status.

![i3status-bilal](docs/i3rs.png)

![i3status-bilal-urgent](docs/i3rs-urgent.png)

`i3status-rs` configuration Example:

``` bash
[[block]]
block = "custom"
cycle = ["bilal -c -J", "bilal -n -J"]
on_click = "<command>"
interval = 300
json = true
```

## Installation

### From binaries

The [release page](https://git.sr.ht/~azzamsa/bilal.rs/refs/) includes
pre-compiled binaries for GNU/Linux, macOS and Windows.

### From source

Using Rust's package manager [cargo](https://github.com/rust-lang/cargo):

``` bash
cargo install bilal
```


## Development

``` bash
git clone https://git.sr.ht/~azzamsa/bilal.rs
cd bilal.rs

# Run unit tests and integration tests
cargo test

# Install
cargo install --path .
```

## Contributing

For reporting issues, visit the tracker here: https://todo.sr.ht/~azzamsa/Bilal

Please send patches to `~azzamsa/public-inbox@lists.sr.ht`

## Origin of the name

The name Bilal was chosen in reference to the Bilal bin Rabah. The first
mu'azzin, chosen by Muhammad PBUH himself.

## License

Copyright (c) 2020 Azzamsa

Bilal is distributed under the terms of [GPL V3 License](LICENSE).


