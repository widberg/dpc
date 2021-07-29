use std::convert::From;
use std::ffi::OsStr;
use std::io::Result;
use std::path::Path;

use clap::ArgMatches;

#[derive(Clone, Copy)]
pub struct Options {
    pub is_quiet: bool,
    pub is_force: bool,
    pub is_unsafe: bool,
    pub is_lz: bool,
    pub is_optimization: bool,
    pub is_recursive: bool,
}

impl From<&ArgMatches<'_>> for Options {
    fn from(arg_matches: &ArgMatches) -> Self {
        Options {
            is_quiet: arg_matches.is_present("QUIET"),
            is_force: arg_matches.is_present("FORCE"),
            is_unsafe: arg_matches.is_present("UNSAFE"),
            is_lz: arg_matches.is_present("LZ"),
            is_optimization: arg_matches.is_present("OPTIMIZATION"),
            is_recursive: arg_matches.is_present("RECURSIVE"),
        }
    }
}

pub trait DPC {
    fn new(options: &Options, custom_args: &Vec<&OsStr>) -> Self;
    fn extract<P: AsRef<Path>>(&mut self, input_path: &P, output_path: &P) -> Result<()>;
    fn create<P: AsRef<Path>>(&mut self, input_path: &P, output_path: &P) -> Result<()>;
    fn validate<P: AsRef<Path>>(&self, input_path: &P, output_path: &P) -> Result<()>;
    fn compress_object<P: AsRef<Path>>(&self, input_path: &P, output_path: &P) -> Result<()>;
    fn decompress_object<P: AsRef<Path>>(&self, input_path: &P, output_path: &P) -> Result<()>;
    fn split_object<P: AsRef<Path>>(&self, input_path: &P, output_path: &P) -> Result<()>;
    fn fmt_extract<P: AsRef<Path>>(&self, input_path: &P, output_path: &P) -> Result<()>;
    fn fmt_create<P: AsRef<Path>>(&self, input_path: &P, output_path: &P) -> Result<()>;
}
