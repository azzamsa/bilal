# Bilal

**bilal** is a CLI salah time that has configuration feature, small, fast, and just one single binary.

![demo](https://git.sr.ht/~azzamsa/blobs/blob/master/bilal/bilal.gif)

## Features

- Show all salah time in current day
- Show current salah time and its remaining time
- Show next salah time
- Show result in JSON format
- Cross-platform

## Usage

Create a file named `config.toml` in `~/.config/bilal/` (*nix) or
`%APPDATA\Azzamsa\Bilal` (Windows). Then add your configuration:


``` toml
latitude = 41.0096334
longitude = 28.9651646
madhab = "Shafi"
method = "Singapore"
```

To learn more about the config, see the [config documentation](doc/wiki.md#cofiguration).

## Installation

## With cargo (from source)

``` bash
$ git clone https://git.sr.ht/~azzamsa/bilal.rs
$ cd bilal.rs
$ cargo build --release
$ cp target/release/bilal /usr/local/bin/
```

## From binaries

Download the binary from the [Release](https://git.sr.ht/~azzamsa/bilal.rs/refs/)


## Usage with other tools

You can use Bilal with i3status-rs to show salah time in your status bar.

![i3status-bilal](https://git.sr.ht/~azzamsa/blobs/blob/master/bilal/salah-0.1.3.png)

![i3status-bilal-urgent](https://git.sr.ht/~azzamsa/blobs/blob/master/bilal/salah-0.1.3-urgent.png)

i3status-rs configuration Example:

``` bash
[[block]]
block = "custom"
cycle = ["bilal -c -j", "bilal -n -j"]
on_click = "<command>"
interval = 300
json = true
```
## Contributing

For reporting issues, visit the tracker here: https://todo.sr.ht/~azzamsa/Bilal

## Origin of the name

The name Bilal was chosen in reference to the Bilal bin Rabah. The first
mu'azzin, chosen by Muhammad PBUH himself.


## License

Copyright (c) 2020 Azzamsa

Bilal is distributed under the terms of [GPL V3 License](LICENSE).


