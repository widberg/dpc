use bimap::BiHashMap;
use clap::{Arg, App};
use std::fs;
use std::fs::File;
use indicatif::ProgressBar;
use std::path::Path;
use std::io::Result;
use serde_json::json;
use std::io::Read;
use std::io::Write;
use nom::*;

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
				 .required(true)
                 .help("The output directory"))
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
        .get_matches();

	if matches.is_present("EXTRACT") == matches.is_present("CREATE") {
		panic!("Exactly one of -e/-c must be present.");
	}

	let is_quiet = matches.is_present("QUIET");
	let is_force = matches.is_present("FORCE");
	let is_extract = matches.is_present("EXTRACT");

	let input_path = Path::new(matches.value_of("INPUT").unwrap());
	let output_path = Path::new(matches.value_of("OUTPUT").unwrap());

	if is_extract {
		match extract(&input_path, &output_path, is_quiet, is_force) {
			Ok(_) => (),
			Err(error) => panic!("Extraction error: {:?}", error),
		};
	} else {
		panic!("DPC creation not supported.");
	}

	Ok(())
}

pub fn extract<P: AsRef<Path>>(input_path: &P, output_path: &P, is_quiet: bool, is_force: bool) -> Result<()> {
	let mut class_names = BiHashMap::<u32, &str>::new();
	class_names.insert(549480509, "Omni_Z");
    class_names.insert(705810152, "Rtc_Z");
    class_names.insert(838505646, "GenWorld_Z");
    class_names.insert(848525546, "LightData_Z");
    class_names.insert(849267944, "Sound_Z");
    class_names.insert(849861735, "MaterialObj_Z");
    class_names.insert(866453734, "RotShape_Z");
    class_names.insert(954499543, "ParticlesData_Z");
    class_names.insert(968261323, "World_Z");
    class_names.insert(1114947943, "Warp_Z");
    class_names.insert(1135194223, "Spline_Z");
    class_names.insert(1175485833, "Anim_Z");
    class_names.insert(1387343541, "Mesh_Z");
    class_names.insert(1391959958, "UserDefine_Z");
    class_names.insert(1396791303, "Skin_Z");
    class_names.insert(1471281566, "Bitmap_Z");
    class_names.insert(1536002910, "Font_Z");
    class_names.insert(1625945536, "RotShapeData_Z");
    class_names.insert(1706265229, "Surface_Z");
    class_names.insert(1910554652, "SplineGraph_Z");
    class_names.insert(1943824915, "Lod_Z");
    class_names.insert(2204276779, "Material_Z");
    class_names.insert(2245010728, "Node_Z");
    class_names.insert(2259852416, "Binary_Z");
    class_names.insert(2398393906, "CollisionVol_Z");
    class_names.insert(2906362741, "WorldRef_Z");
    class_names.insert(3312018398, "Particles_Z");
    class_names.insert(3412401859, "LodData_Z");
    class_names.insert(3611002348, "Skel_Z");
    class_names.insert(3626109572, "MeshData_Z");
    class_names.insert(3747817665, "SurfaceDatas_Z");
    class_names.insert(3834418854, "MaterialAnim_Z");
    class_names.insert(3845834591, "GwRoad_Z");
    class_names.insert(4096629181, "GameObj_Z");
    class_names.insert(4240844041, "Camera_Z");
	
	let mut input_file = File::open(input_path.as_ref()).unwrap_or_else(|why| {
        panic!("Problem opening the input file: {:?}", why.kind());
    });

	if output_path.as_ref().exists() && !is_force {
		panic!("Output directory already exists. Choose a new output directory or run the program with the -f flag to overwrite the existing directory.");
	}

	fs::create_dir_all(output_path.as_ref()).unwrap_or_else(|why| {
        panic!("Problem creating the output directory: {:?}", why.kind());
    });

	let manifest_path = output_path.as_ref().join("manifest.json");
	let mut manifest_file = File::create(manifest_path).unwrap_or_else(|why| {
        panic!("Problem creating the manifest file: {:?}", why.kind());
    });

	if !is_quiet {
		let pb = ProgressBar::new(100);

		for _ in 0..100 {
			pb.inc(1);
			std::thread::sleep(std::time::Duration::from_millis(10));
		}

		pb.finish_and_clear();
	}

	#[derive(Clone,Copy,Debug,PartialEq,Eq)]
	struct Header<'a> {
		version_string: &'a str,
	}

	named!(parse_version_string<Header>,
		do_parse!(
			version_string: take_str!(255) >>
			(Header {
				version_string: version_string,
			})
		)
	);

	let mut buffer = [0; 255];
	input_file.read(&mut buffer)?;
	let header = match parse_version_string(&buffer) {
		Ok((_, h)) => h,
		Err(_) => panic!(),
	};

	println!("{}", header.version_string);

	// ...

	let john = json!({
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    });

	manifest_file.write_all(john.to_string().as_bytes()).unwrap_or_else(|why| {
        panic!("Problem writing the manifest file: {:?}", why.kind());
    });

	return Ok(());
}
