# DPC

```plaintext
dpc version 0.1.0;faeb7f5b90ab75047c0b861f945332f938c61f23;x86_64-pc-windows-msvc
widberg <https://github.com/widberg>
Work with DPC files

USAGE:
    dpc.exe [FLAGS] [OPTIONS] --game <GAME> --input <INPUT> [-- <CUSTOM_ARGS>]

FLAGS:
    -c, --create          directory -> DPC
    -e, --extract         DPC -> directory
    -f, --force           Don't ask about existing folder
    -l, --lz              Apply Asobo LZ compression/deflation when appropriate        
    -O, --optimization    Optimize the DPC
    -q, --quiet           No console output
    -u, --unsafe          Don't check the version string for compatibility
    -h, --help            Prints help information
    -V, --version         Prints version information

OPTIONS:
    -g, --game <GAME>        The game the dpc should be compatible with [possible values: fuel]
    -i, --input <INPUT>      The input DPC file
    -o, --output <OUTPUT>    The output directory

ARGS:
    <CUSTOM_ARGS>    Supply arguments directly to the dpc backend
```
