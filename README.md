# DPC

Tar analogous utility to work with the BigFile format from Asobo Studio games.

<sup>This repository is a relative of the main [FMTK repository](https://github.com/widberg/fmtk).</sup>

While the tool is named DPC it works with BigFiles from other platforms too.

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
* Toy Story 3 (unstable)

## Tutorial

Since most people being pointed towards this tool don't even know how to run a command line utility, here is a quick rundown for the uninitiated.
First, download the dpc program from the [releases tab](https://github.com/widberg/dpc/releases) of this repository. Download the latest file matching your operating system. Once downloaded, unzip it with 7-zip or a similar utility.
Once it is unzipped, [open a command prompt](https://www.thewindowsclub.com/how-to-open-command-prompt-from-right-click-menu) in the folder you unzipped it to.
Now we can begin using the tool.

To extract a BigFile run the command
```sh
dpc -g fuel -eulf -i "path/to/BIGFILE.DPC" -o "path/to/BIGFILE.DPC.d"
```
where `path/to/BIGFILE.DPC` is the path of the bigfile on disk. This will create a directory `path/to/BIGFILE.DPC.d` next to the BigFile you extracted containing the extracted data.

When you are done messing around with the extracted data you may want to turn it back into a BigFile. This can be done with the command.
```sh
dpc -g fuel -culf -i "path/to/BIGFILE.DPC.d" -o "path/to/NEW_BIGFILE.DPC"
```
where `path/to/BIGFILE.DPC.d` is the path of the extracted folder on disk. This will create a BigFile `path/to/NEW_BIGFILE.DPC` next to the extracted folder.

Note that while the command contains the name of the game FUEL, these commands will work with the other "unstable" games. This is because the formats are similar enough between these games that we can piggyback off the FUEL support even if each individual game has not been considered.

This tutorial covers the most basic use case that 90% of people want this tool for; in actuality, the tool is far more powerful. To learn about the other options and subcommand, run the command `dpc --help` for more information.

## Help

```plaintext
dpc version 0.1.5;c12d4143e64e51f381196ac5c223d3ea326f2557;x86_64-pc-windows-msvc
widberg <https://github.com/widberg>
Work with DPC files

USAGE:
    dpc [FLAGS] [OPTIONS] --game <GAME> [-- <CUSTOM_ARGS>]
    dpc <SUBCOMMAND>

FLAGS:
    -c, --create          directory -> DPC
    -e, --extract         DPC -> directory
    -f, --force           Don't ask about existing folder
    -l, --lz              Apply Asobo LZ compression/deflation when appropriate
    -O, --optimization    Optimize the DPC
    -q, --quiet           No console output
    -r, --recursive       extract the dpc and all objects
    -u, --unsafe          Don't check the version string for compatibility
    -v, --validate        Checks if your DPC is valid
    -h, --help            Prints help information
    -V, --version         Prints version information

OPTIONS:
    -g, --game <GAME>        The game the dpc should be compatible with [possible values: fuel]
    -i, --input <INPUT>      The input DPC file
    -o, --output <OUTPUT>    The output directory

ARGS:
    <CUSTOM_ARGS>    Supply arguments directly to the dpc backend

SUBCOMMANDS:
    crc32    generate name files
    fmt      Used to format object files
    help     Prints this message or the help of the given subcommand(s)
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
