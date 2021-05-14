use clap::{Arg, App, AppSettings};
use std::ffi::OsStr;
use std::io::Result;
use std::path::Path;
use std::path::PathBuf;

pub mod base_dpc;
pub mod fuel_dpc;
pub mod lz;

use base_dpc::Options;
use base_dpc::DPC;
use fuel_dpc::FuelDPC;

#[allow(dead_code)]
mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

fn main() -> Result<()> {
	let mut version_string = String::from("version ");

	version_string.push_str(built_info::PKG_VERSION);

	if let Some(hash) = built_info::GIT_COMMIT_HASH {
		version_string.push(';');
		version_string.push_str(hash);
	}

	version_string.push(';');
	version_string.push_str(built_info::TARGET);

	if let Some(ci) = built_info::CI_PLATFORM {
		version_string.push(';');
		version_string.push_str(ci);
	}

    let matches = App::new("dpc")
        .version(version_string.as_str())
        .author("widberg <https://github.com/widberg>")
        .about("Work with DPC files")
        .arg(Arg::with_name("INPUT")
                 .short("i")
                 .long("input")
                 .takes_value(true)
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
				.possible_values(&["fuel"])
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
				.conflicts_with("CREATE")
				.requires("INPUT")
				.help("DPC -> directory"))
		.arg(Arg::with_name("CREATE")
				.short("c")
				.long("create")
				.conflicts_with("EXTRACT")
				.requires("INPUT")
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
		.arg(Arg::with_name("CUSTOM_ARGS")
				.last(true)
				.required(false)
				.help("Supply arguments directly to the dpc backend"))
		.after_help("EXAMPLES:\n    -g fuel -- -h\n    -cflO -g fuel -i BIKE.DPC.d -o BIKE.DPC\n    -ef -g fuel -i /FUEL/**/*.DPC")
		.setting(AppSettings::ArgRequiredElseHelp)
        .get_matches_from(wild::args_os());

	let options = Options::from(&matches);

	let mut custom_args: Vec<&OsStr> = vec![&OsStr::new("--")];

	match matches.values_of_os("CUSTOM_ARGS") {
		Some(args) => custom_args.extend(args),
		None => (),
	};

	let dpc = match matches.value_of("GAME") {
		None => panic!("Game is required"), // default to fuel until other games are supported
		Some(game) => match game {
			"fuel" => FuelDPC::new(&options, &custom_args),
			_ => panic!("bad game"),
		},
	};

	if !matches.is_present("EXTRACT") && !matches.is_present("CREATE") {
		return Ok(());
	}

	let input_path_strings = matches.values_of_os("INPUT").unwrap().into_iter();

	if input_path_strings.len() > 1 && matches.is_present("OUTPUT") {
		panic!("Cannot specify output path for more than one input path.");
	}

	for input_path_string in input_path_strings {
		let input_path = Path::new(input_path_string);

		if matches.is_present("EXTRACT") {
			let output_path = match matches.value_of_os("OUTPUT") {
				Some(output_path_string) => PathBuf::from(output_path_string),
				None => input_path.with_extension("DPC.d"),
			};

			match dpc.extract(&input_path, &output_path.as_path()) {
				Ok(_) => (),
				Err(error) => panic!("Extraction error: {:?}", error),
			};
		} else {
			let output_path = match matches.value_of_os("OUTPUT") {
				Some(output_path_string) => PathBuf::from(output_path_string),
				None => input_path.with_extension("DPC"),
			};

			match dpc.create(&input_path, &output_path.as_path()) {
				Ok(_) => (),
				Err(error) => panic!("Creation error: {:?}", error),
			};
		}
	}

	Ok(())
}
