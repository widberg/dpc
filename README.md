# DPC

Tar analogous utility to work with the DPC file format from Asobo Studio games.

<sup>This repository is a relative of the main [FMTK repository](https://github.com/widberg/fmtk).</sup>

## Help

```plaintext
dpc version 0.1.1;452c6d7a015dc87f10545c1793f0d3b6e6f0afbc;x86_64-pc-windows-msvc
widberg <https://github.com/widberg>
Work with DPC files

USAGE:
    dpc.exe [FLAGS] [OPTIONS] --game <GAME> [-- <CUSTOM_ARGS>]
    dpc.exe <SUBCOMMAND>

FLAGS:
    -c, --create          directory -> DPC
    -e, --extract         DPC -> directory
    -f, --force           Don't ask about existing folder
    -l, --lz              Apply Asobo LZ compression/deflation when appropriate
    -O, --optimization    Optimize the DPC
    -q, --quiet           No console output
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
    help    Prints this message or the help of the given subcommand(s)
    lz      Used to compress raw files
    obj     Used to compress object files

EXAMPLES:
    -g fuel -- -h
    -cflO -g fuel -i BIKE.DPC.d -o BIKE.DPC
    -ef -g fuel -i /FUEL/**/*.DPC
```
