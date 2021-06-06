use clap::{App, AppSettings, Arg, SubCommand};
use std::ffi::{OsStr, OsString};
use std::io::Result;
use std::path::Path;
use std::path::PathBuf;

pub mod base_dpc;
pub mod fuel_dpc;
pub mod fuel_fmt;
pub mod lz;

use base_dpc::Options;
use base_dpc::DPC;
use fuel_dpc::FuelDPC;
use lz::LZ;

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
				 .global(true)
                 .help("The input DPC file"))
        .arg(Arg::with_name("OUTPUT")
                 .short("o")
                 .long("output")
                 .takes_value(true)
				 .requires("INPUT")
				 .global(true)
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
				.conflicts_with("VALIDATE")
				.requires("INPUT")
				.help("DPC -> directory"))
		.arg(Arg::with_name("CREATE")
				.short("c")
				.long("create")
				.conflicts_with("EXTRACT")
				.conflicts_with("VALIDATE")
				.requires("INPUT")
				.help("directory -> DPC"))
		.arg(Arg::with_name("VALIDATE")
				.short("v")
				.long("validate")
				.conflicts_with("CREATE")
				.conflicts_with("EXTRACT")
				.requires("INPUT")
				.help("Checks if your DPC is valid"))
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
		.subcommand(SubCommand::with_name("lz")
				.about("Used to compress raw files")
				.arg(Arg::with_name("ALGORITHM")
						.short("a")
						.long("algorithm")
						.takes_value(true)
						.required(true)
						.requires("INPUT")
						.possible_values(&["lzss", "lz4"])
						.help("The algorithm the raw file should be compatible with"))
				.arg(Arg::with_name("COMPRESS")
						.short("c")
						.long("compress")
						.requires("INPUT")
						.conflicts_with("DECOMPRESS")
						.help("compress the file"))
				.arg(Arg::with_name("DECOMPRESS")
						.short("d")
						.long("decompress")
						.requires("INPUT")
						.conflicts_with("COMPRESS")
						.help("decompress the file"))
				.after_help("EXAMPLES:\n    lz -ac lzss -i raw.dat\n    lz -ad lz4 -i raw.dat")
				.settings(&[AppSettings::ArgRequiredElseHelp]))
		.subcommand(SubCommand::with_name("obj")
				.about("Used to compress object files")
				.arg(Arg::with_name("GAME")
						.short("g")
						.long("game")
						.takes_value(true)
						.required(true)
						.possible_values(&["fuel"])
						.help("The game the object should be compatible with"))
				.arg(Arg::with_name("COMPRESS")
						.short("c")
						.long("compress")
						.requires("INPUT")
						.conflicts_with("DECOMPRESS")
						.help("compress the file"))
				.arg(Arg::with_name("DECOMPRESS")
						.short("d")
						.long("decompress")
						.requires("INPUT")
						.conflicts_with("COMPRESS")
						.help("decompress the file"))
				.arg(Arg::with_name("SPLIT")
						.short("s")
						.long("split")
						.requires("INPUT")
						.help("split the file"))
				.settings(&[AppSettings::ArgRequiredElseHelp]))
		.subcommand(SubCommand::with_name("fmt")
				.about("Used to format object files")
				.arg(Arg::with_name("GAME")
						.short("g")
						.long("game")
						.takes_value(true)
						.required(true)
						.possible_values(&["fuel"])
						.help("The game the object should be compatible with"))
				.arg(Arg::with_name("CREATE")
						.short("c")
						.long("create")
						.requires("INPUT")
						.conflicts_with("EXTRACT")
						.help("create the file"))
				.arg(Arg::with_name("EXTRACT")
						.short("e")
						.long("extract")
						.requires("INPUT")
						.conflicts_with("CREATE")
						.help("extract the file"))
				.settings(&[AppSettings::ArgRequiredElseHelp]))
		.after_help("EXAMPLES:\n    -g fuel -- -h\n    -cflO -g fuel -i BIKE.DPC.d -o BIKE.DPC\n    -ef -g fuel -i /FUEL/**/*.DPC")
		.settings(&[AppSettings::ArgRequiredElseHelp, AppSettings::SubcommandsNegateReqs, AppSettings::ArgsNegateSubcommands])
        .get_matches_from(wild::args_os());

    if let Some(subcommand_matches) = matches.subcommand_matches("lz") {
        let input_path_string = matches.value_of_os("INPUT").unwrap();
        let input_path = Path::new(input_path_string);

        let output_path = match subcommand_matches.value_of_os("OUTPUT") {
            Some(output_path_string) => PathBuf::from(output_path_string),
            None => input_path.with_extension(if subcommand_matches.is_present("COMPRESS") {
                "comp"
            } else {
                "uncomp"
            }),
        };

        match subcommand_matches.value_of("ALGORITHM") {
            None => panic!("Algorithm is required"),
            Some(algorithm) => match algorithm {
                "lzss" => {
                    if subcommand_matches.is_present("COMPRESS") {
                        lz::LZLZSS::compress(&input_path, &output_path.as_path())?;
                    } else {
                        lz::LZLZSS::decompress(&input_path, &output_path.as_path())?;
                    }
                }
                "lz4" => {
                    if subcommand_matches.is_present("COMPRESS") {
                        lz::LZLZ4::compress(&input_path, &output_path.as_path())?;
                    } else {
                        lz::LZLZ4::decompress(&input_path, &output_path.as_path())?;
                    }
                }
                _ => panic!("bad algorithm"),
            },
        };

        return Ok(());
    }

    let options = Options::from(&matches);

    let custom_args: Vec<&OsStr> = match matches.values_of_os("CUSTOM_ARGS") {
        Some(args) => args.collect(),
        None => vec![],
    };

	if let Some(subcommand_matches) = matches.subcommand_matches("obj") {
        let input_path_string = matches.value_of_os("INPUT").unwrap();
        let mut input_path = Path::new(input_path_string);

        let output_path = match matches.value_of_os("OUTPUT") {
            Some(output_path_string) => Path::new(output_path_string),
            None => input_path,
        };

		let dpc = match subcommand_matches.value_of("GAME") {
			None => panic!("Game is required"), // default to fuel until other games are supported
			Some(game) => match game {
				"fuel" => FuelDPC::new(&options, &custom_args),
				_ => panic!("bad game"),
			},
		};

		if subcommand_matches.is_present("COMPRESS") {
			dpc.compress_object(&input_path, &output_path)?;
			input_path = output_path;
		} else if subcommand_matches.is_present("DECOMPRESS") {
			dpc.decompress_object(&input_path, &output_path)?;
			input_path = output_path;
		}
		
		if subcommand_matches.is_present("SPLIT") {
			dpc.split_object(&input_path, &output_path)?;
		}

        return Ok(());
    }

	if let Some(subcommand_matches) = matches.subcommand_matches("fmt") {
        let input_path_string = matches.value_of_os("INPUT").unwrap();
        let input_path = Path::new(input_path_string);
		let mut t = OsString::new();

        let output_path = match matches.value_of_os("OUTPUT") {
            Some(output_path_string) => Path::new(output_path_string),
            None => { t.push(input_path.as_os_str()); t.push(".d"); Path::new(&t) },
        };

		let dpc = match subcommand_matches.value_of("GAME") {
			None => panic!("Game is required"), // default to fuel until other games are supported
			Some(game) => match game {
				"fuel" => FuelDPC::new(&options, &custom_args),
				_ => panic!("bad game"),
			},
		};

		if subcommand_matches.is_present("CREATE") {
			dpc.fmt_create(&input_path, &output_path)?;
		} else if subcommand_matches.is_present("EXTRACT") {
			dpc.fmt_extract(&input_path, &output_path)?;
		}

        return Ok(());
    }

    let dpc = match matches.value_of("GAME") {
        None => panic!("Game is required"), // default to fuel until other games are supported
        Some(game) => match game {
            "fuel" => FuelDPC::new(&options, &custom_args),
            _ => panic!("bad game"),
        },
    };

    if !matches.is_present("EXTRACT")
        && !matches.is_present("CREATE")
        && !matches.is_present("VALIDATE")
    {
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

            dpc.extract(&input_path, &output_path.as_path())?
        } else if matches.is_present("CREATE") {
            let output_path = match matches.value_of_os("OUTPUT") {
                Some(output_path_string) => PathBuf::from(output_path_string),
                None => input_path.with_extension("DPC"),
            };

            match dpc.create(&input_path, &output_path.as_path()) {
                Ok(_) => (),
                Err(error) => panic!("Creation error: {:?}", error),
            };
        } else if matches.is_present("VALIDATE") {
            let output_path = match matches.value_of_os("OUTPUT") {
                Some(output_path_string) => PathBuf::from(output_path_string),
                None => input_path.with_extension("DPC.json"),
            };

            match dpc.validate(&input_path, &output_path.as_path()) {
                Ok(_) => (),
                Err(error) => panic!("Validation error: {:?}", error),
            };
        } else {
            panic!("Unreachable")
        }
    }

    Ok(())
}
