# DPC

Tar analogous utility to work with the DPC file format from Asobo Studio games.

<sup>This repository is a relative of the main [FMTK repository](https://github.com/widberg/fmtk).</sup>

## Supported Games/Versions

All versions listed have full support. The bold versions are the version that the respective game is equipped to load. Unstable games are games that may be processed by the tool using the `--unsafe` option but do not have a dedicated DPC backend.

* Ratatouille (unstable)
* WALL-E (unstable)
* FUEL
  * **v1.381.67.09 - Asobo Studio - Internal Cross Technology**
  * v1.381.66.09 - Asobo Studio - Internal Cross Technology
  * v1.381.65.09 - Asobo Studio - Internal Cross Technology
  * v1.381.64.09 - Asobo Studio - Internal Cross Technology
  * v1.379.60.09 - Asobo Studio - Internal Cross Technology
  * v1.325.50.07 - Asobo Studio - Internal Cross Technology
  * v1.220.50.07 - Asobo Studio - Internal Cross Technology
* Up (unstable)

## Help

```plaintext
dpc version 0.1.3;44705387770367e588fce766539eba654ff7e
018;x86_64-pc-windows-msvc
widberg <https://github.com/widberg>
Work with DPC files

USAGE:
    dpc.exe [FLAGS] [OPTIONS] --game <GAME> [-- <CUSTOM
_ARGS>]
    dpc.exe <SUBCOMMAND>

FLAGS:
    -c, --create          directory -> DPC
    -e, --extract         DPC -> directory
    -f, --force           Don't ask about existing fold
er
    -l, --lz              Apply Asobo LZ compression/de
flation when appropriate
    -O, --optimization    Optimize the DPC
    -q, --quiet           No console output
    -r, --recursive       extract the dpc and all objec
ts
    -u, --unsafe          Don't check the version strin
g for compatibility
    -v, --validate        Checks if your DPC is valid
    -h, --help            Prints help information
    -V, --version         Prints version information

OPTIONS:
    -g, --game <GAME>        The game the dpc should be
 compatible with [possible values: fuel]
    -i, --input <INPUT>      The input DPC file
    -o, --output <OUTPUT>    The output directory

ARGS:
    <CUSTOM_ARGS>    Supply arguments directly to the d
pc backend

SUBCOMMANDS:
    crc32    generate name files
    fmt      Used to format object files
    help     Prints this message or the help of the giv
en subcommand(s)
    lz       Used to compress raw files
    obj      Used to compress/split object files

EXAMPLES:
    -g fuel -- -h
    -cflO -g fuel -i BIKE.DPC.d -o BIKE.DPC
    -ef -g fuel -i /FUEL/**/*.DPC
```

## Getting Started

### Prerequisites

* [Rust](https://www.rust-lang.org/)

### Checkout

```sh
git clone https://github.com/widberg/dpc.git
cd dpc
```

### Build

```sh
cargo build --release
```
