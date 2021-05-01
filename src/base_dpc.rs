use std::path::Path;
use std::io::Result;
use clap::ArgMatches;
use std::convert::From;

#[derive(Clone,Copy)]
pub struct Options {
	pub is_quiet: bool,
	pub is_force: bool,
	pub is_extract: bool,
	pub is_unsafe: bool,
	pub is_lz: bool,
}

impl From<&ArgMatches<'_>> for Options {
	fn from(arg_matches: &ArgMatches) -> Self {
		Options {
			is_quiet: arg_matches.is_present("QUIET"),
			is_force: arg_matches.is_present("FORCE"),
			is_extract: arg_matches.is_present("EXTRACT"),
			is_unsafe: arg_matches.is_present("UNSAFE"),
			is_lz: arg_matches.is_present("LZ"),
		}
	}
}

pub trait DPC {
	fn new(options: &Options) -> Self;
	fn extract<P: AsRef<Path>>(&self, input_path: &P, output_path: &P) -> Result<()>;
	fn create<P: AsRef<Path>>(&self, input_path: &P, output_path: &P) -> Result<()>;
}
