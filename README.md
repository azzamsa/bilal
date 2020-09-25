# Bilal

A CLI salah time

## Demo

``` bash
$ bilal --help    
Bilal [A CLI salah time] 0.1.0

USAGE:
    bilal [FLAGS]

FLAGS:
    -a, --all        Show all Salah time
    -c, --current    Show current Salah
    -h, --help       Prints help information
    -n, --next       Show next Salah
    -V, --version    Prints version information

```

## Features

- Show all salah time in current day
- Show current salah time and its remaining time
- Show next salah time

## Usage

Create a file named `bilal.toml` in `~/.config/bilal/`. Then add your
configuration:


``` toml
latitude = 41.0096334
longitude = 28.9651646
```

You can get latitude and longitude value from
[mapcoordinates](https://www.mapcoordinates.net/en)


## Installation

## With cargo (from source)

``` bash
$ git clone https://git.sr.ht/~azzamsa/bilal.rs bilal
$ cd bilal
$ cargo build --release
$ cp target/release/bilal /usr/local/bin/
```

## From binaries

Download the binary from the [Release](https://git.sr.ht/~azzamsa/bilal.rs/refs/)


## Usage with other tools

You can use Bilal with i3status-rs to show salah time in your status bar.

i3status-rs configuration Example:

``` bash
[[block]]
block = "custom"
cycle = ["bilal -c", "bilal -n"]
on_click = "<command>"
interval = 300
```
## Contributing

For reporting issues, visit the tracker here: https://todo.sr.ht/~azzamsa/Bilal

## Origin of the name

The name Bilal was chosen in reference to the Bilal bin Rabah. The first
mu'azzin, chosen by Muhammad PBUH himself.


## License

Copyright (c) 2020 Azzamsa

Bilal is distributed under the terms of [GPL V3 License](LICENSE).


