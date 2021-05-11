use clap::{Arg, App};
use std::path::Path;
use std::io::Result;

pub mod base_dpc;
pub mod fuel_dpc;

use base_dpc::Options;
use base_dpc::DPC;
use fuel_dpc::FuelDPC;

#[allow(dead_code)]
mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}


fn main() -> Result<()> {
	let mut version_string = String::from(built_info::PKG_VERSION);

	if let Some(hash) = built_info::GIT_COMMIT_HASH {
		version_string.push(';');
		version_string.push_str(hash);
	}

    let matches = App::new("dpc")
        .version(version_string.as_str())
        .author("widberg <https://github.com/widberg>")
        .about("Work with DPC files")
        .arg(Arg::with_name("INPUT")
                 .short("i")
                 .long("input")
                 .takes_value(true)
				 .required(true)
                 .help("The input DPC file"))
        .arg(Arg::with_name("OUTPUT")
                 .short("o")
                 .long("output")
                 .takes_value(true)
                 .help("The output directory"))
		.arg(Arg::with_name("GAME")
				.short("g")
				.long("game")
				.takes_value(true)
				.required(true)
				.help("The game the dpc should be compatible with"))
		.arg(Arg::with_name("QUIET")
				.short("q")
				.long("quiet")
				.help("No console output"))
		.arg(Arg::with_name("FORCE")
				.short("f")
				.long("force")
				.help("Don't ask about existing folder"))
		.arg(Arg::with_name("EXTRACT")
				.short("e")
				.long("extract")
				.help("DPC -> directory"))
		.arg(Arg::with_name("CREATE")
				.short("c")
				.long("create")
				.help("directory -> DPC"))
		.arg(Arg::with_name("UNSAFE")
				.short("u")
				.long("unsafe")
				.help("Don't check the version string for compatibility"))
		.arg(Arg::with_name("LZ")
				.short("l")
				.long("lz")
				.help("Apply Asobo LZ compression/deflation when appropriate"))
		.arg(Arg::with_name("OPTIMIZATION")
				.short("O")
				.long("optimization")
				.help("Optimize the DPC"))
        .get_matches_from(wild::args());

	if matches.is_present("EXTRACT") == matches.is_present("CREATE") {
		panic!("Exactly one of -e/-c must be present.");
	}

	let options = Options::from(&matches);

	let dpc = match matches.value_of("GAME") {
		None => panic!("bad game"),
		Some(game) => match game {
			"fuel" => FuelDPC::new(&options),
			_ => panic!("bad game"),
		},
	};

	let input_path = Path::new(matches.value_of("INPUT").unwrap());
	// doesnt work for creation because no .DPC ext
	let output_path = Path::new(matches.value_of("OUTPUT").unwrap_or(input_path.file_stem().unwrap().to_str().unwrap()));

	if matches.is_present("EXTRACT") {
		match dpc.extract(&input_path, &output_path) {
			Ok(_) => (),
			Err(error) => panic!("Extraction error: {:?}", error),
		};
	} else {
		match dpc.create(&input_path, &output_path) {
			Ok(_) => (),
			Err(error) => panic!("Creation error: {:?}", error),
		};
	}

	Ok(())
}
