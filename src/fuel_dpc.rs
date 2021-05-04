use crate::base_dpc;
use base_dpc::DPC;
use base_dpc::Options;
use std::path::Path;
use std::io::Result;
use std::fs;
use std::fs::File;
use indicatif::ProgressBar;
use std::io::Read;
use std::io::Write;
use nom::*;
use nom::number::complete::*;
use nom_derive::{NomLE, Parse};
use std::io::SeekFrom;
use std::io::prelude::*;
use std::collections::HashSet;
use std::collections::HashMap;
use binwrite::BinWrite;
use serde::Serialize;
use serde::Deserialize;
use std::cmp::max;

#[derive(Serialize, Deserialize)]
struct Header {
	version_string: String,
	is_rtc: bool,
	pool_manifest_unused: u32,
	incredi_builder_string: String,
}

#[derive(Serialize, Deserialize)]
struct ObjectDescription {
	crc32: u32,
	compress: bool,
	depends: Vec<u32>,
}

#[derive(Serialize, Deserialize)]
struct Block {
	block_type: u32,
	offset: u32,
	objects: Vec<ObjectDescription>,
}

#[derive(Serialize, Deserialize)]
struct Manifest {
	header: Header,
	blocks: Vec<Block>,
	pool: Vec<u32>,
}

impl Manifest {
	fn new() -> Manifest {
		Manifest {
			header: Header {
				version_string: String::from(""),
				is_rtc: false,
				pool_manifest_unused: 0,
				incredi_builder_string: String::from(""),
			},
			blocks: vec![],
			pool: vec![],
		}
	}
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

#[derive(NomLE,BinWrite,Clone,Copy,Debug,PartialEq,Eq)]
struct PoolManifestHeader
{
	equals524288: u32,
	equals2048: u32,
	objects_crc32_count_sum: u32,
}

#[derive(NomLE,BinWrite,Clone,Copy,Debug,PartialEq,Eq)]
struct ReferenceRecord {
	start_chunk_index: u32,
	end_chunk_index: u32,
	objects_crc32_starting_index: u32,
	#[nom(SkipBefore(2), SkipAfter(12))]
	objects_crc32_count: u16,
}


#[derive(NomLE,BinWrite,Clone,Copy,Debug,PartialEq,Eq)]
struct BlockDescription {
	block_type: u32,
	object_count: u32,
	padded_size: u32,
	data_size: u32,
	working_buffer_offset: u32,
	crc32: u32,
}

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
		let mut class_names: HashMap<u32, &str> = HashMap::new();
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

		let mut global_objects: HashMap<u32, ObjectDescription> = HashMap::new();
		let mut global_object_headers: HashMap<u32, ObjectHeader> = HashMap::new();
		
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

		let mut manifest_json = Manifest::new();

		named_args!(take_c_string_as_str(size: usize)<&str>, do_parse!(
			s: take_str!(size) >>
			(s.trim_end_matches('\0'))
		));

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

		manifest_json.header.version_string = String::from(header.version_string);
		manifest_json.header.is_rtc = header.is_not_rtc == 0;
		manifest_json.header.pool_manifest_unused = header.pool_manifest_unused0;
		if header.block_sector_padding_size != 0xFFFFFFFF {
			manifest_json.header.incredi_builder_string = String::from(header.incredi_builder_string);
		}

		println!("{:#?}", header);

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

			let mut v = vec![];

			let mut buff: Vec<u8> = vec![0; (block_description.padded_size) as usize];
			input_file.read(&mut buff)?;

			let objects = match count!(buff.as_bytes(), BlockObject::parse, block_description.object_count as usize) {
				Ok((_, h)) => h,
				Err(error) => panic!("{}", error),
			};

			for object in objects.iter() {
				v.push(ObjectDescription {
					crc32: object.header.crc32,
					compress: object.header.compressed_size != 0,
					depends: vec![],
				});

				if !crc32s.contains(&object.header.crc32) {
					let mut object_file = File::create(objects_path.join(format!("{}.{}", object.header.crc32, class_names.get(&object.header.class_crc32).unwrap())))?;
					object.header.write(&mut object_file)?;
					object_file.write(&object.data)?;
					crc32s.insert(object.header.crc32);

					global_object_headers.insert(object.header.crc32, object.header);
					
					global_objects.insert(object.header.crc32, ObjectDescription {
						crc32: object.header.crc32,
						compress: object.header.compressed_size != 0,
						depends: vec![],
					});
				}
				pb.inc(1);
			}

			manifest_json.blocks.push(Block {
				block_type: block_description.block_type,
				offset: block_description.working_buffer_offset,
				objects: v,
			});

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

		for i in pool_manifest.objects_crc32s.iter() {
			let crc32 = pool_manifest.crc32s[*i as usize];
			let reference_record = pool_manifest.reference_records[pool_manifest.reference_records_indices[*i as usize] as usize - 1];
			let depends_indices = &pool_manifest.objects_crc32s[(reference_record.objects_crc32_starting_index as usize)..(reference_record.objects_crc32_starting_index as usize + reference_record.objects_crc32_count as usize)];
			let mut depends = vec![];
			for index in depends_indices.iter() {
				depends.push(pool_manifest.crc32s[*index as usize]);
			}

			let od = global_objects.get_mut(&crc32).unwrap();
			od.depends = depends;
		}

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
				.read(true)
				.write(true)
				.open(objects_path.join(format!("{}.{}", pool_object.header.crc32, class_names.get(&pool_object.header.class_crc32).unwrap())))?;
			
			object_file.seek(SeekFrom::End(0))?;
			object_file.write(pool_object.data.as_bytes())?;
			global_objects.get_mut(&pool_object.header.crc32).unwrap().compress = pool_object.header.compressed_size != 0;

			let mut oh = global_object_headers.get(&pool_object.header.crc32).unwrap().clone();
			oh.data_size += pool_object.header.data_size;
			oh.compressed_size = pool_object.header.compressed_size;
			oh.decompressed_size = pool_object.header.decompressed_size;

			object_file.seek(SeekFrom::Start(0))?;
			oh.write(&mut object_file)?;

			manifest_json.pool.push(pool_object.header.crc32);
		}

		// ...

		for block in manifest_json.blocks.iter_mut() {
			for object in block.objects.iter_mut() {
				let od: &ObjectDescription = global_objects.get(&object.crc32).unwrap();
				object.compress = od.compress;
				object.depends = od.depends.clone();
			}
		}

		manifest_file.write(serde_json::to_string_pretty(&manifest_json)?.as_bytes()).unwrap_or_else(|why| {
			panic!("Problem writing the manifest file: {:?}", why.kind());
		});


		Ok(())
	}

	fn create<P: AsRef<Path>>(&self, input_path: &P, output_path: &P) -> Result<()> {
		let manifest_file = File::open(input_path.as_ref().join("manifest.json")).unwrap_or_else(|why| {
			panic!("Problem opening the input file: {:?}", why.kind());
		});

		let manifest_json: Manifest = serde_json::from_reader(manifest_file)?;

		let mut dpc_file = File::create(output_path.as_ref())?;

		let mut index: HashMap<u32, std::path::PathBuf> = HashMap::new();

		for path in fs::read_dir(input_path.as_ref().join("objects"))? {
			let actual_os_path = path.unwrap().path();
			let actual_path: &Path = actual_os_path.as_path();
			let crc32: u32 = Path::new(actual_path.file_name().unwrap()).file_stem().unwrap().to_str().unwrap().to_string().parse::<u32>().unwrap();
			if index.contains_key(&crc32) {
				panic!("Ambiguous files for crc32 = {}", crc32);
			}
			
			index.insert(crc32, actual_os_path);
		}

		dpc_file.seek(SeekFrom::Start(2048))?;

		let mut block_sector_padding_size: u32 = 0;

		let mut block_descriptions: Vec<BlockDescription> = Vec::new();

		for block in manifest_json.blocks.iter() {
			let start_pos = dpc_file.stream_position()?;


			for object in block.objects.iter() {
				let mut object_file = File::open(index.get(&object.crc32).unwrap().as_path())?;
				let mut buffer: [u8; 24] = [0; 24];
				object_file.read(&mut buffer)?;

				
				let (_, mut oh) = ObjectHeader::parse(&buffer).unwrap();

				if object.depends.len() == 0 {
					oh.write(&mut dpc_file)?;
					let mut data = vec![0; oh.data_size as usize];
					object_file.read(&mut data)?;
					dpc_file.write(&data)?;
				} else {
					oh.data_size = oh.class_object_size;
					oh.compressed_size = 0;
					oh.decompressed_size = 0;
					oh.write(&mut dpc_file)?;
					let mut data = vec![0; oh.class_object_size as usize];
					object_file.read(&mut data)?;
					dpc_file.write(&data)?;
				}
			}

			let end_pos = dpc_file.stream_position()?;
			let len = (end_pos - start_pos) as u32;

			let block_description = BlockDescription {
				block_type: block.block_type,
				object_count: block.objects.len() as u32,
				crc32: block.objects[0].crc32,
				data_size: len,
				padded_size: len + (2048 - len % 2048),
				working_buffer_offset: block.offset,
			};

			block_descriptions.push(block_description);

			let pos: i64 = dpc_file.stream_position()? as i64;
			block_sector_padding_size += (2048 - pos % 2048) as u32;
			dpc_file.seek(SeekFrom::Current(2048 - pos % 2048))?;
		}

		let pool_manifest_offset: u32 = dpc_file.stream_position()? as u32;

		let pool_header = PoolManifestHeader {
			equals524288: 524288,
			equals2048: 2048,
			objects_crc32_count_sum: 0,
		};

		pool_header.write(&mut dpc_file)?;

		let pos: i64 = dpc_file.stream_position()? as i64;
		let mut pool_sector_padding_size: u32 = 0;
		let lll = vec![0xff;(2048 - pos % 2048) as usize];
		dpc_file.write(&lll)?;
		pool_sector_padding_size += lll.len() as u32;

		let pool_manifest_padded_size: u32 = dpc_file.stream_position()? as u32 - pool_manifest_offset;

		let mut max_pool_decompressed_size = 0;

		for crc32 in manifest_json.pool.iter() {
			let mut object_file = File::open(index.get(&crc32).unwrap().as_path())?;
			let mut buffer: [u8; 24] = [0; 24];
			object_file.read(&mut buffer)?;

			let (_, mut oh) = ObjectHeader::parse(&buffer).unwrap();

			if oh.compressed_size != 0 {
				max_pool_decompressed_size = max(max_pool_decompressed_size, (oh.decompressed_size + 2047) / 2048);
			}

			oh.data_size = oh.data_size - oh.class_object_size;
			let mut buffer = vec![0;oh.data_size as usize];

			object_file.seek(SeekFrom::Current(oh.class_object_size as i64))?;
			object_file.read(&mut buffer)?;

			oh.class_object_size = 0;
			oh.write(&mut dpc_file)?;
			dpc_file.write(&buffer)?;
			
			let pos: i64 = dpc_file.stream_position()? as i64;
			let lll = vec![0xff;(2048 - pos % 2048) as usize];
			dpc_file.write(&lll)?;

			pool_sector_padding_size += lll.len() as u32;
		}

		let file_padded_size = dpc_file.stream_position()? as u32;

		// HEADER

		dpc_file.seek(SeekFrom::Start(0))?;

		#[derive(BinWrite,Clone,Debug,PartialEq,Eq)]
		struct PrimaryHeaderPartA {
			is_not_rtc: u32,
			block_count: u32,
			block_working_buffer_capacity_even: u32,
			block_working_buffer_capacity_odd: u32,
			padded_size: u32,
			version_patch: u32,
			version_minor: u32,
		}

		dpc_file.write(manifest_json.header.version_string.as_bytes())?;

		dpc_file.seek(SeekFrom::Start(256))?;


		let mut version_lookup: HashMap<String, (u32, u32)> = HashMap::new();
		version_lookup.insert(String::from("v1.381.67.09 - Asobo Studio - Internal Cross Technology"), (272, 380));
		version_lookup.insert(String::from("v1.381.66.09 - Asobo Studio - Internal Cross Technology"), (272, 380));
		version_lookup.insert(String::from("v1.381.65.09 - Asobo Studio - Internal Cross Technology"), (271, 380));
		version_lookup.insert(String::from("v1.381.64.09 - Asobo Studio - Internal Cross Technology"), (271, 380));
		version_lookup.insert(String::from("v1.379.60.09 - Asobo Studio - Internal Cross Technology"), (269, 380));
		version_lookup.insert(String::from("v1.325.50.07 - Asobo Studio - Internal Cross Technology"), (262, 326));
		version_lookup.insert(String::from("v1.220.50.07 - Asobo Studio - Internal Cross Technology"), (262, 221));

		let (version_patch, version_minor) = version_lookup.get(&manifest_json.header.version_string).unwrap();


		let mut block_working_buffer_capacity_even = 0;
		let mut block_working_buffer_capacity_odd = 0;

		for i in 0..manifest_json.blocks.len() {
			let block_working_buffer_capacity = block_descriptions[i].padded_size + block_descriptions[i].working_buffer_offset;
			if i % 2 == 0 {
				block_working_buffer_capacity_even = max(block_working_buffer_capacity_even, block_working_buffer_capacity);
			} else {
				block_working_buffer_capacity_odd = max(block_working_buffer_capacity_odd, block_working_buffer_capacity);
			}
		}

		let phpa = PrimaryHeaderPartA {
			is_not_rtc: !manifest_json.header.is_rtc as u32,
			block_count: manifest_json.blocks.len() as u32,
			block_working_buffer_capacity_even: block_working_buffer_capacity_even,
			block_working_buffer_capacity_odd: block_working_buffer_capacity_odd,
			padded_size: pool_manifest_offset - 2048,
			version_patch: *version_patch,
			version_minor: *version_minor,
		};

		phpa.write(&mut dpc_file)?;

		for block_description in block_descriptions.iter() {
			block_description.write(&mut dpc_file)?;
		}

		dpc_file.seek(SeekFrom::Start(0x720))?;

		#[derive(BinWrite,Clone,Debug,PartialEq,Eq)]
		struct PrimaryHeaderPartB {
			pool_manifest_padded_size: u32,
			pool_manifest_offset: u32,
			pool_manifest_unused0: u32,
			pool_manifest_unused1: u32,
			pool_object_decompression_buffer_capacity: u32,
			block_sector_padding_size: u32,
			pool_sector_padding_size: u32,
			file_size: u32,
		}

		let phpb = PrimaryHeaderPartB {
			pool_manifest_padded_size: (pool_manifest_padded_size + 2047) / 2048,
			pool_manifest_offset: (pool_manifest_offset + 2047) / 2048,
			pool_manifest_unused0: manifest_json.header.pool_manifest_unused,
			pool_manifest_unused1: manifest_json.header.pool_manifest_unused,
			pool_object_decompression_buffer_capacity: max_pool_decompressed_size,
			block_sector_padding_size: block_sector_padding_size,
			pool_sector_padding_size: pool_sector_padding_size,
			file_size: file_padded_size,
		};

		phpb.write(&mut dpc_file)?;

		dpc_file.write(manifest_json.header.incredi_builder_string.as_bytes())?;

		dpc_file.seek(SeekFrom::Start(0x7c0))?;

		let padding = [0xff;64];

		dpc_file.write(&padding)?;

		Ok(())
	}
}

#[cfg(test)]
mod test {
	use crate::base_dpc::Options;
	use crate::base_dpc::DPC;
	use crate::fuel_dpc::FuelDPC;
	use checksums::hash_file;
	use checksums::Algorithm;
	use std::path::Path;
	use test_generator::test_resources;

    #[test_resources("data/*.DPC")]
    fn test_fuel_dpc(path: &str) {
		let dpc = FuelDPC::new(&Options {
			is_quiet: true,
			is_force: true,
			is_unsafe: false,
			is_lz: false,
		});

		let dpc_file = Path::new(path);
		let dpc_file_2 = Path::new("data/test/TEMP.DPC");
		let dpc_directory = Path::new("data/test/TEMP");

		dpc.extract(&dpc_file, &dpc_directory).unwrap();
		dpc.create(&dpc_directory, &dpc_file_2).unwrap();
		assert_eq!(hash_file(dpc_file, Algorithm::SHA1), hash_file(dpc_file_2, Algorithm::SHA1));
    }
}
