use crate::base_dpc;
use base_dpc::DPC;
use base_dpc::Options;
use std::path::Path;
use std::io::Result;
use bimap::BiHashMap;
use std::fs;
use std::fs::File;
use indicatif::ProgressBar;
use serde_json::json;
use std::io::Read;
use std::io::Write;
use nom::*;
use nom::number::complete::*;
use nom_derive::{NomLE, Parse};
use std::io::SeekFrom;
use std::io::prelude::*;
use std::collections::HashSet;
use binwrite::BinWrite;

pub struct FuelDPC {
	options: Options,
}

impl DPC for FuelDPC {
	fn new(options: &Options) -> FuelDPC {
		FuelDPC {
			options: *options,
		}
	}

	fn extract<P: AsRef<Path>>(&self, input_path: &P, output_path: &P) -> Result<()> {
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

		if output_path.as_ref().exists() && !self.options.is_force {
			panic!("Output directory already exists. Choose a new output directory or run the program with the -f flag to overwrite the existing directory.");
		}

		fs::create_dir_all(output_path.as_ref()).unwrap_or_else(|why| {
			panic!("Problem creating the output directory: {:?}", why.kind());
		});

		let manifest_path = output_path.as_ref().join("manifest.json");
		let mut manifest_file = File::create(manifest_path).unwrap_or_else(|why| {
			panic!("Problem creating the manifest file: {:?}", why.kind());
		});

		named_args!(take_c_string_as_str(size: usize)<&str>, do_parse!(
			s: take_str!(size) >>
			(s.trim_end_matches('\0'))
		));

		#[derive(NomLE,Clone,Copy,Debug,PartialEq,Eq)]
		struct BlockDescription {
			block_type: u32,
			object_count: u32,
			padded_size: u32,
			data_size: u32,
			working_buffer_offset: u32,
			crc32: u32,
		}

		#[derive(NomLE,Clone,Debug,PartialEq,Eq)]
		struct PrimaryHeader<'a> {
			#[nom(Parse = "{ |i| take_c_string_as_str(i, 256) }")]
			version_string: &'a str,
			is_not_rtc: u32,
			#[nom(Verify = "*block_count <= 64")]
			block_count: u32,
			block_working_buffer_capacity_even: u32,
			block_working_buffer_capacity_odd: u32,
			padded_size: u32,
			version_patch: u32,
			version_minor: u32,
			#[nom(Count = "block_count", Parse = "BlockDescription::parse")]
			block_descriptions: Vec<BlockDescription>,
			#[nom(MoveAbs(0x720))]
			#[nom(Map = "|x| x * 2048")]
			pool_manifest_padded_size: u32,
			#[nom(Map = "|x| x * 2048")]
			pool_manifest_offset: u32,
			pool_manifest_unused0: u32,
			pool_manifest_unused1: u32,
			pool_object_decompression_buffer_capacity: u32,
			block_sector_padding_size: u32,
			pool_sector_padding_size: u32,
			file_size: u32,
			#[nom(Parse = "{ |i| take_c_string_as_str(i, 128) }")]
			incredi_builder_string: &'a str,
		}

		#[derive(NomLE,BinWrite,Clone,Copy,Debug,PartialEq,Eq)]
		#[binwrite(little)]
		struct ObjectHeader {
			data_size: u32,
			class_object_size: u32,
			decompressed_size: u32,
			compressed_size: u32,
			class_crc32: u32,
			crc32: u32,
		}

		#[derive(NomLE,Clone,Debug,PartialEq,Eq)]
		struct BlockObject {
			#[nom(Parse = "ObjectHeader::parse")]
			header: ObjectHeader,
			#[nom(Count((header.data_size) as usize))]
			data: Vec<u8>,
		}

		#[derive(NomLE,Clone,Debug,PartialEq,Eq)]
		struct PoolObject {
			#[nom(Parse = "ObjectHeader::parse")]
			header: ObjectHeader,
			#[nom(Count((header.data_size) as usize), AlignAfter(2048))]
			data: Vec<u8>,
		}

		#[derive(NomLE,Clone,Copy,Debug,PartialEq,Eq)]
		struct PoolManifestHeader
		{
			equals524288: u32,
			equals2048: u32,
			objects_crc32_count_sum: u32,
		}

		#[derive(NomLE,Clone,Copy,Debug,PartialEq,Eq)]
		struct ReferenceRecord {
			start_chunk_index: u32,
			end_chunk_index: u32,
			objects_crc32_starting_index: u32,
			#[nom(SkipBefore(2), SkipAfter(12))]
			objects_crc32_count: u16,
		}
		
		#[derive(NomLE,Clone,Debug,PartialEq,Eq)]
		struct PoolManifest {
			#[nom(Parse = "PoolManifestHeader::parse")]
			header : PoolManifestHeader,
			#[nom(Parse = "{ |i| length_count!(i, le_u32, le_u32) }")]
			objects_crc32s: Vec<u32>,
			#[nom(Parse = "{ |i| length_count!(i, le_u32, le_u32) }")]
			crc32s: Vec<u32>,
			#[nom(Parse = "{ |i| length_count!(i, le_u32, le_u32) }")]
			reference_counts: Vec<u32>,
			#[nom(Parse = "{ |i| length_count!(i, le_u32, le_u32) }")]
			object_padded_size: Vec<u32>,
			#[nom(Parse = "{ |i| length_count!(i, le_u32, le_u32) }")]
			reference_records_indices: Vec<u32>,
			#[nom(Parse = "{ |i| length_count!(i, le_u32, ReferenceRecord::parse) }")]
			reference_records: Vec<ReferenceRecord>,
		}
		
		let mut buffer = [0; 2048];
		input_file.read(&mut buffer)?;
		let header = match PrimaryHeader::parse(&buffer) {
			Ok((_, h)) => h,
			Err(error) => panic!("{}", error),
		};

		//println!("{:#?}", header);

		let mut object_count = 0;

		for block_description in header.block_descriptions.iter() {
			object_count += block_description.object_count;
		}

		let pb = ProgressBar::new(object_count as u64);
		if !self.options.is_quiet {

		}

		let mut crc32s = HashSet::new();

		let objects_path = output_path.as_ref().join("objects");
		fs::create_dir_all(&objects_path)?;

		for block_description in header.block_descriptions.iter() {

			let mut buff: Vec<u8> = vec![0; (block_description.padded_size) as usize];
			input_file.read(&mut buff)?;

			let objects = match count!(buff.as_bytes(), BlockObject::parse, block_description.object_count as usize) {
				Ok((_, h)) => h,
				Err(error) => panic!("{}", error),
			};

			for object in objects.iter() {
				if !crc32s.contains(&object.header.crc32) {
					let mut object_file = File::create(objects_path.join(format!("{}.{}", object.header.crc32, class_names.get_by_left(&object.header.class_crc32).unwrap())))?;
					object.header.write(&mut object_file)?;
					object_file.write(&object.data)?;
					crc32s.insert(object.header.crc32);
				}
				pb.inc(1);
			}

			//println!("{:#?}", objects);
		}
		// ...

		pb.finish_and_clear();

		let mut buf: Vec<u8> = vec![0; header.pool_manifest_padded_size as usize];
		input_file.read(&mut buf)?;

		let pool_manifest = match PoolManifest::parse(&buf) {
			Ok((_, h)) => h,
			Err(error) => panic!("{}", error),
		};

		//println!("{:#?}", pool_manifest);

		let cur = input_file.seek(SeekFrom::Current(0)).unwrap();
		let end = input_file.seek(SeekFrom::End(0)).unwrap();
		input_file.seek(SeekFrom::Start(cur))?;

		let mut bufff: Vec<u8> = vec![0; (end - cur) as usize];
		input_file.read(&mut bufff)?;

		let pool_objects = match count!(bufff.as_bytes(), PoolObject::parse, pool_manifest.objects_crc32s.len() as usize) {
			Ok((_, h)) => h,
			Err(error) => panic!("{}", error),
		};

		for pool_object in pool_objects.iter() {
			let mut object_file = std::fs::OpenOptions::new()
				.append(true)
				.open(objects_path.join(format!("{}.{}", pool_object.header.crc32, class_names.get_by_left(&pool_object.header.class_crc32).unwrap())))?;
			object_file.write(pool_object.data.as_bytes())?;
		}

		// ...

		let john = json!({
			"name": "John Doe",
			"age": 43,
			"phones": [
				"+44 1234567",
				"+44 2345678"
			]
		});

		manifest_file.write(john.to_string().as_bytes()).unwrap_or_else(|why| {
			panic!("Problem writing the manifest file: {:?}", why.kind());
		});


		Ok(())
	}

	fn create<P: AsRef<Path>>(&self, input_path: &P, output_path: &P) -> Result<()> {
		Ok(())
	}
}
