use std::cmp::max;
use std::collections::HashMap;
use std::collections::HashSet;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::fs;
use std::fs::File;
use std::fs::{metadata, OpenOptions};
use std::io::prelude::*;
use std::io::Cursor;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Read;
use std::io::Result;
use std::io::SeekFrom;
use std::io::Write;
use std::option::Option::Some;
use std::path::Path;
use std::path::PathBuf;

use binwrite::BinWrite;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use clap::{App, AppSettings, Arg};
use dialoguer::Select;
use indicatif::ProgressBar;
use itertools::Itertools;
use nom::number::complete::*;
use nom::*;
use nom_derive::{Nom, NomBE, Parse};
use serde::Deserialize;
use serde::Serialize;
use tempdir::TempDir;

use base_dpc::Options;
use base_dpc::DPC;

use crate::base_dpc;
use crate::fuel_fmt;
use crate::lz;

fn calculate_padded_size(unpadded_size: u32) -> u32 {
    return (unpadded_size + 0x7ff) & 0xfffff800;
}

fn calculate_padding_size(unpadded_size: u32) -> u32 {
    return calculate_padded_size(unpadded_size) - unpadded_size;
}

#[derive(Serialize, Deserialize)]
struct Header {
    version_string: String,
    version_minor: Option<u32>,
    version_patch: Option<u32>,
    block_type: Option<u32>,
    is_rtc: bool,
    pool_manifest_unused: u32,
    incredi_builder_string: String,
}

#[derive(Serialize, Deserialize)]
struct ObjectDescription {
    crc32: u32,
    compress: bool,
}

#[derive(Serialize, Deserialize)]
struct Block {
    offset: u32,
    objects: Vec<ObjectDescription>,
}

#[derive(Serialize, Deserialize)]
struct Manifest {
    header: Header,
    blocks: Vec<Block>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pool: Option<Pool>,
}

#[derive(Serialize, Deserialize)]
struct PoolObjectEntry {
    crc32: u32,
    reference_record_index: u32,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
struct JsonReferenceRecord {
    object_entries_starting_index: u32,
    object_entries_count: u16,
}

#[derive(Serialize, Deserialize)]
struct Pool {
    object_entry_indices: Vec<u32>,
    object_entries: Vec<PoolObjectEntry>,
    reference_records: Vec<JsonReferenceRecord>,
}

impl Manifest {
    fn new() -> Manifest {
        Manifest {
            header: Header {
                version_string: String::from(""),
                version_minor: None,
                version_patch: None,
                block_type: None,
                is_rtc: false,
                pool_manifest_unused: 0,
                incredi_builder_string: String::from(""),
            },
            blocks: vec![],
            pool: None,
        }
    }
}

#[derive(NomBE, BinWrite, Clone, Copy, Debug, PartialEq, Eq)]
#[binwrite(big)]
struct ObjectHeader {
    data_size: u32,
    class_object_size: u32,
    decompressed_size: u32,
    compressed_size: u32,
    class_crc32: u32,
    crc32: u32,
}

#[derive(Serialize, NomBE, BinWrite, Clone, Copy, Debug, PartialEq, Eq)]
#[binwrite(big)]
struct PoolManifestHeader {
    equals524288: u32,
    equals2048: u32,
    objects_crc32_count_sum: u32,
}

#[derive(Serialize, NomBE, BinWrite, Clone, Copy, Debug, PartialEq, Eq)]
#[binwrite(big)]
struct ReferenceRecord {
    start_chunk_index: u32,
    end_chunk_index: u32,
    objects_crc32_starting_index: u32,
    placeholder_dpc_index: u16,
    objects_crc32_count: u16,
    placeholder_times_referenced: u32,
    placeholder_current_references_shared: u32,
    placeholder_current_references_weak: u32,
}

#[derive(Serialize, NomBE, Clone, Debug, PartialEq, Eq)]
struct PoolManifest {
    #[nom(Parse = "PoolManifestHeader::parse")]
    header: PoolManifestHeader,
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

#[derive(Serialize, NomBE, BinWrite, Clone, Copy, Debug, PartialEq, Eq)]
#[binwrite(big)]
struct BlockDescription {
    block_type: u32,
    object_count: u32,
    padded_size: u32,
    data_size: u32,
    working_buffer_offset: u32,
    crc32: u32,
}

named_args!(take_c_string_as_str(size: usize)<&str>, do_parse!(
    s: take_str!(size) >>
    (s.trim_end_matches('\0'))
));

named!(take_nothing_as_str<&str>, do_parse!(("")));

#[derive(Serialize, NomBE, Clone, Debug, PartialEq, Eq)]
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
    #[nom(Map = "|x| if x != -1 { x * 2048 } else { 0 }")]
    pool_manifest_padded_size: i32,
    #[nom(Map = "|x| if x != -1 { x * 2048 } else { 0 }")]
    pool_manifest_offset: i32,
    pool_manifest_unused0: u32,
    pool_manifest_unused1: u32,
    pool_object_decompression_buffer_capacity: u32,
    block_sector_padding_size: u32,
    pool_sector_padding_size: u32,
    file_size: u32,
    #[nom(
        Parse = "{ |i| { if file_size != 0xFFFFFFFF { take_c_string_as_str(i, 128) } else { take_nothing_as_str(i) } } }"
    )]
    incredi_builder_string: &'a str,
}

pub struct FuelDPC {
    options: Options,
    unoptimized_pool: bool,
    no_pool: bool,
    version_lookup: HashMap<String, (u32, u32, u32)>,
    version: String,
}

impl DPC for FuelDPC {
    fn new(options: &Options, custom_args: &Vec<&OsStr>) -> FuelDPC {
        let matches = App::new("fuel dpc backend")
            .version("version 1.0.0")
            .author("widberg <https://github.com/widberg>")
            .about("FUEL")
            .arg(
                Arg::with_name("UNOPTIMIZED-POOL")
                    .short("p")
                    .long("unoptimized-pool")
                    .help("Don't minify the pool manifest"),
            )
            .arg(
                Arg::with_name("NO-POOL")
                    .short("n")
                    .long("no-pool")
                    .help("Don't use a pool"),
            )
            .settings(&[AppSettings::NoBinaryName])
            .get_matches_from(custom_args);

        let mut version_lookup: HashMap<String, (u32, u32, u32)> = HashMap::new();
        version_lookup.insert(
            String::from("v1.530.62.09 - Asobo Studio - Internal Cross Technology"),
            (290, 529, 150),
        );
        version_lookup.insert(
            String::from("v1.381.67.09 - Asobo Studio - Internal Cross Technology"),
            (272, 380, 253),
        );
        version_lookup.insert(
            String::from("v1.381.66.09 - Asobo Studio - Internal Cross Technology"),
            (272, 380, 252),
        );
        version_lookup.insert(
            String::from("v1.381.65.09 - Asobo Studio - Internal Cross Technology"),
            (271, 380, 249),
        );
        version_lookup.insert(
            String::from("v1.381.64.09 - Asobo Studio - Internal Cross Technology"),
            (271, 380, 249),
        );
        version_lookup.insert(
            String::from("v1.379.60.09 - Asobo Studio - Internal Cross Technology"),
            (269, 380, 211),
        );
        version_lookup.insert(
            String::from("v1.325.50.07 - Asobo Studio - Internal Cross Technology"),
            (262, 326, 146),
        );
        version_lookup.insert(
            String::from("v1.220.50.07 - Asobo Studio - Internal Cross Technology"),
            (262, 221, 144),
        );
        FuelDPC {
            options: *options,
            unoptimized_pool: matches.is_present("UNOPTIMIZED-POOL"),
            no_pool: matches.is_present("NO-POOL"),
            version_lookup: version_lookup,
            version: String::from("v1.381.67.09 - Asobo Studio - Internal Cross Technology"),
        }
    }

    fn extract<P: AsRef<Path>>(&mut self, input_path: &P, output_path: &P) -> Result<()> {
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
        class_names.insert(1175485833, "Animation_Z");
        class_names.insert(1387343541, "Mesh_Z");
        class_names.insert(1391959958, "UserDefine_Z");
        class_names.insert(1396791303, "Skin_Z");
        class_names.insert(1471281566, "Bitmap_Z");
        class_names.insert(1536002910, "Fonts_Z");
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
        class_names.insert(4117606081, "AnimFrame_Z");
        class_names.insert(3979333606, "CameraZone_Z");
        class_names.insert(72309972, "Occluder_Z");
        class_names.insert(1390918523, "Graph_Z");
        class_names.insert(1918499807, "Light_Z");
        class_names.insert(3210467954, "HFogData_Z");
        class_names.insert(2735949084, "HFog_Z");
        class_names.insert(2203168663, "Flare_Z");
        class_names.insert(1393846573, "FlareData_Z");

        let mut global_objects: HashMap<u32, ObjectDescription> = HashMap::new();
        let mut global_object_headers: HashMap<u32, ObjectHeader> = HashMap::new();
        let mut global_object_references: HashMap<u32, (Vec<u32>, Vec<u32>)> = HashMap::new();

        let mut input_file = File::open(input_path.as_ref()).unwrap_or_else(|why| {
            panic!("Problem opening the input file: {:?}", why.kind());
        });

        if output_path.as_ref().exists() && !self.options.is_force {
            println!("Output directory already exists. You can avoid this interaction by choosing a new output directory or run the program with the -f flag to overwrite the existing directory and avoid this prompt for all files. What would you like to do for {}", output_path.as_ref().to_str().unwrap());
            let selection = Select::new()
                .item("Exit")
                .item("Skip this file")
                .item("Overwrite this file")
                .default(0)
                .interact()?;

            match selection {
                0 => panic!("Aborting"),
                1 => return Ok(()),
                2 => (),
                _ => panic!("Invalid choice"),
            };
        }

        fs::create_dir_all(output_path.as_ref()).unwrap_or_else(|why| {
            panic!("Problem creating the output directory: {:?}", why.kind());
        });

        let manifest_path = output_path.as_ref().join("manifest.json");
        let mut manifest_file = File::create(manifest_path).unwrap_or_else(|why| {
            panic!("Problem creating the manifest file: {:?}", why.kind());
        });

        let mut manifest_json = Manifest::new();

        #[derive(NomBE, Clone, Debug, PartialEq, Eq)]
        struct BlockObject {
            #[nom(Parse = "ObjectHeader::parse")]
            header: ObjectHeader,
            #[nom(Count((header.class_object_size) as usize))]
            class_object: Vec<u8>,
            #[nom(Count((header.data_size - header.class_object_size) as usize))]
            data: Vec<u8>,
        }

        #[derive(NomBE, Clone, Debug, PartialEq, Eq)]
        struct PoolObject {
            #[nom(Parse = "ObjectHeader::parse")]
            header: ObjectHeader,
            #[nom(Count((header.data_size) as usize), AlignAfter(2048))]
            data: Vec<u8>,
        }

        let mut buffer = [0; 2048];
        input_file.read(&mut buffer)?;
        let header = match PrimaryHeader::parse(&buffer) {
            Ok((_, h)) => h,
            Err(error) => panic!("{}", error),
        };

        if !self.version_lookup.contains_key(header.version_string) && !self.options.is_unsafe {
            panic!("Invalid version string for fuel. Use -u/--unsafe to bypass this check and extract the dpc anyway.");
        }

        self.version = String::from(header.version_string);
        manifest_json.header.version_string = String::from(header.version_string);
        if !self.version_lookup.contains_key(header.version_string) {
            manifest_json.header.version_minor = Some(header.version_minor);
            manifest_json.header.version_patch = Some(header.version_patch);
            if header.block_descriptions.len() > 0 {
                manifest_json.header.block_type = Some(header.block_descriptions[0].block_type);
            }
        }
        manifest_json.header.is_rtc = header.is_not_rtc == 0;
        manifest_json.header.pool_manifest_unused = header.pool_manifest_unused0;
        if header.block_sector_padding_size != 0xFFFFFFFF {
            manifest_json.header.incredi_builder_string =
                String::from(header.incredi_builder_string);
        }

        //println!("{:#?}", header);

        let mut object_count = 0;

        for block_description in header.block_descriptions.iter() {
            object_count += block_description.object_count;
        }

        let pb = match self.options.is_quiet {
            false => ProgressBar::new(object_count as u64),
            true => ProgressBar::hidden(),
        };

        let mut crc32s = HashSet::new();

        let objects_path = output_path.as_ref().join("objects");
        fs::create_dir_all(&objects_path)?;
        let mut x = 1;

        for block_description in header.block_descriptions.iter() {
            pb.println(format!(
                "Processing block {}/{}",
                x,
                header.block_descriptions.len()
            ));

            let mut v = vec![];

            let mut buff: Vec<u8> = vec![0; (block_description.padded_size) as usize];
            input_file.read(&mut buff)?;

            let objects = match count!(
                buff.as_bytes(),
                BlockObject::parse,
                block_description.object_count as usize
            ) {
                Ok((_, h)) => h,
                Err(error) => panic!("{} on block {}", error, x),
            };

            // println!(
            //     "id = {}, data_size = {}, padded_size = {}, offset = {}",
            //     x,
            //     block_description.data_size,
            //     block_description.padded_size,
            //     block_description.working_buffer_offset
            // );

            x += 1;

            //let mut a = 0;

            for object in objects.iter() {
                v.push(ObjectDescription {
                    crc32: object.header.crc32,
                    compress: object.header.compressed_size != 0,
                });

                // println!(
                //     "\t{} {} {} {}",
                //     a,
                //     object.header.data_size + 24,
                //     class_names.get(&object.header.class_crc32).unwrap(),
                //     object.header.compressed_size != 0
                // );

                // a += object.header.data_size + 24;

                if !crc32s.contains(&object.header.crc32) {
                    let x: String;
                    if let Some(v) = class_names.get(&object.header.class_crc32) {
                        x = String::from(*v)
                    } else {
                        x = object.header.class_crc32.to_string().clone();
                    }
                    let object_file_path =
                        objects_path.join(format!("{}.{}", object.header.crc32, x.as_str()));
                    let mut object_file = File::create(&object_file_path)?;
                    let mut oh = object.header;
                    if self.options.is_lz && object.header.compressed_size != 0 {
                        pb.println(format!("Decompressing {}", object.header.crc32));
                        let mut decompressed_buffer = vec![0; oh.decompressed_size as usize];
                        lz::lzrs_decompress(
                            &object.data[8..],
                            oh.compressed_size as usize - 8,
                            &mut decompressed_buffer[..],
                            oh.decompressed_size as usize,
                            false,
                        )?;

                        oh.compressed_size = 0;
                        oh.data_size = oh.class_object_size + oh.decompressed_size;

                        oh.write(&mut object_file)?;
                        object_file.write(&object.class_object)?;
                        object_file.write(&decompressed_buffer)?;
                        assert_eq!(oh.data_size, oh.class_object_size + oh.decompressed_size);
                        assert_eq!(oh.data_size + 24, object_file.stream_position()? as u32);
                    } else {
                        pb.println(format!("Processing {}", object.header.crc32));
                        oh.write(&mut object_file)?;
                        object_file.write(&object.class_object)?;
                        object_file.write(&object.data)?;
                        assert_eq!(
                            oh.data_size,
                            oh.class_object_size
                                + if oh.compressed_size != 0 {
                                    oh.compressed_size
                                } else {
                                    oh.decompressed_size
                                }
                        );
                        assert_eq!(oh.data_size + 24, object_file.stream_position()? as u32);
                    }

                    if oh.data_size > oh.class_object_size && self.options.is_recursive {
                        pb.println(format!("Extracting {}", oh.crc32));
                        let mut t = OsString::new();
                        t.push(object_file_path.as_os_str());
                        t.push(".d");
                        match self.fmt_extract(&object_file_path, &PathBuf::from(&t)) {
                            Ok(x) => {
                                global_object_references.insert(oh.crc32, x);
                            }
                            Err(ref e) => {
                                if e.kind() != ErrorKind::Other {
                                    panic!("{}: {}", oh.crc32, e);
                                }
                            }
                        }
                    }

                    crc32s.insert(object.header.crc32);

                    global_object_headers.insert(object.header.crc32, object.header);

                    global_objects.insert(
                        object.header.crc32,
                        ObjectDescription {
                            crc32: object.header.crc32,
                            compress: object.header.compressed_size != 0,
                        },
                    );
                }
                pb.inc(1);
            }

            manifest_json.blocks.push(Block {
                offset: block_description.working_buffer_offset,
                objects: v,
            });

            //println!("{:#?}", objects);
        }
        // ...

        if header.pool_manifest_offset != 0 {
            let mut buf: Vec<u8> = vec![0; header.pool_manifest_padded_size as usize];
            input_file.read(&mut buf)?;

            let pool_manifest = match PoolManifest::parse(&buf) {
                Ok((_, h)) => h,
                Err(error) => panic!("{}", error),
            };

            let mut object_entries = vec![];
            for i in 0..pool_manifest.crc32s.len() {
                object_entries.push(PoolObjectEntry {
                    crc32: pool_manifest.crc32s[i],
                    reference_record_index: pool_manifest.reference_records_indices[i],
                })
            }

            let mut json_reference_records = vec![];
            for reference_record in pool_manifest.reference_records.iter() {
                json_reference_records.push(JsonReferenceRecord {
                    object_entries_starting_index: reference_record.objects_crc32_starting_index,
                    object_entries_count: reference_record.objects_crc32_count,
                })
            }

            manifest_json.pool = Some(Pool {
                object_entry_indices: pool_manifest.objects_crc32s.clone(),
                object_entries: object_entries,
                reference_records: json_reference_records,
            });

            //println!("{:#?}", pool_manifest);

            let cur = input_file.seek(SeekFrom::Current(0)).unwrap();
            let end = input_file.seek(SeekFrom::End(0)).unwrap();
            input_file.seek(SeekFrom::Start(cur))?;

            let mut bufff: Vec<u8> = vec![0; (end - cur) as usize];
            input_file.read(&mut bufff)?;

            let pool_objects = match count!(
                bufff.as_bytes(),
                PoolObject::parse,
                pool_manifest.objects_crc32s.len() as usize
            ) {
                Ok((_, h)) => h,
                Err(error) => panic!("{}", error),
            };

            pb.println("Processing pool");
            pb.set_position(0);
            pb.set_length(pool_objects.len() as u64);

            for pool_object in pool_objects.iter() {
                pb.println(format!("Processing {}", pool_object.header.crc32));

                let x: String;
                if let Some(v) = class_names.get(&pool_object.header.class_crc32) {
                    x = String::from(*v)
                } else {
                    x = pool_object.header.class_crc32.to_string().clone();
                }
                let object_file_path =
                    objects_path.join(format!("{}.{}", pool_object.header.crc32, x.as_str()));

                let mut object_file = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .open(&object_file_path)?;

                let mut oh = global_object_headers
                    .get(&pool_object.header.crc32)
                    .unwrap()
                    .clone();
                object_file.seek(SeekFrom::Start((oh.class_object_size + 24) as u64))?; // FIXME: if object in pool twice then skip this stuff
                if self.options.is_lz && (pool_object.header.compressed_size != 0) {
                    pb.println(format!("Decompressing {}", pool_object.header.crc32));
                    let mut data_cursor = Cursor::new(&pool_object.data);
                    let decompressed_buffer_len = data_cursor.read_u32::<BigEndian>()?;
                    let compressed_buffer_len = data_cursor.read_u32::<BigEndian>()? - 8;
                    let mut decompressed_buffer = vec![0; decompressed_buffer_len as usize];
                    lz::lzrs_decompress(
                        &pool_object.data[8..],
                        compressed_buffer_len as usize,
                        &mut decompressed_buffer[..],
                        decompressed_buffer_len as usize,
                        false,
                    )?;
                    object_file.write(&decompressed_buffer)?;
                    oh.data_size = oh.class_object_size + pool_object.header.decompressed_size;
                } else {
                    object_file.write(pool_object.data.as_bytes())?;
                    oh.data_size = oh.class_object_size
                        + if pool_object.header.compressed_size != 0 {
                            pool_object.header.compressed_size
                        } else {
                            pool_object.header.decompressed_size
                        };
                }
                global_objects
                    .get_mut(&pool_object.header.crc32)
                    .unwrap()
                    .compress = pool_object.header.compressed_size != 0;

                if self.options.is_lz && pool_object.header.compressed_size != 0 {
                    oh.compressed_size = 0;
                } else {
                    oh.compressed_size = pool_object.header.compressed_size;
                }
                oh.decompressed_size = pool_object.header.decompressed_size;
                assert_eq!(
                    oh.data_size,
                    oh.class_object_size
                        + if oh.compressed_size != 0 {
                            oh.compressed_size
                        } else {
                            oh.decompressed_size
                        }
                );
                assert_eq!(oh.data_size + 24, object_file.stream_position()? as u32);

                object_file.seek(SeekFrom::Start(0))?;
                oh.write(&mut object_file)?;

                if self.options.is_recursive {
                    pb.println(format!("Extracting {}", oh.crc32));
                    let mut t = OsString::new();
                    t.push(object_file_path.as_os_str());
                    t.push(".d");
                    let x = self.fmt_extract(&object_file_path, &PathBuf::from(&t))?;
                    global_object_references.insert(oh.crc32, x);
                }

                pb.inc(1);
            }
        }

        // ...

        pb.finish_and_clear();

        for block in manifest_json.blocks.iter_mut() {
            for object in block.objects.iter_mut() {
                let od: &ObjectDescription = global_objects.get(&object.crc32).unwrap();
                object.compress = od.compress;
            }
        }

        manifest_file
            .write(serde_json::to_string_pretty(&manifest_json)?.as_bytes())
            .unwrap_or_else(|why| {
                panic!("Problem writing the manifest file: {:?}", why.kind());
            });

        let mut references_file = File::create(output_path.as_ref().join("references.txt"))?;
        for (crc32, x) in &global_object_references {
            references_file.write(format!("{} > {:?} & {:?}\n", crc32, x.0, x.1).as_ref())?;
        }

        Ok(())
    }

    //
    // CREATE
    //

    fn create<P: AsRef<Path>>(&mut self, input_path: &P, output_path: &P) -> Result<()> {
        let manifest_file =
            File::open(input_path.as_ref().join("manifest.json")).unwrap_or_else(|why| {
                panic!("Problem opening the input file: {:?}", why.kind());
            });

        if output_path.as_ref().exists() && !self.options.is_force {
            println!("Output DPC already exists. You can avoid this interaction by choosing a new output DPC path or run the program with the -f flag to overwrite the existing DPC and avoid this prompt for all files. What would you like to do for {}", output_path.as_ref().to_str().unwrap());
            let selection = Select::new()
                .item("Exit")
                .item("Skip this file")
                .item("Overwrite this file")
                .default(0)
                .interact()?;

            match selection {
                0 => panic!("Aborting"),
                1 => return Ok(()),
                2 => (),
                _ => panic!("Invalid choice"),
            };
        }

        let mut manifest_json: Manifest = serde_json::from_reader(manifest_file)?;

        self.version = manifest_json.header.version_string.clone();

        if self.no_pool {
            manifest_json.header.pool_manifest_unused = 0;
            manifest_json.pool = None;
        }

        let mut dpc_file = File::create(output_path.as_ref())?;

        let mut index: HashMap<u32, std::path::PathBuf> = HashMap::new();

        if self.options.is_recursive {
            for entry in fs::read_dir(input_path.as_ref().join("objects"))? {
                let entry = entry.unwrap();
                let path = entry.path();
                if path.is_dir() {
                    let res = self.fmt_create(&path, &path.with_extension(""));
                    if res.is_err() {
                        if !self.options.is_unsafe {
                            panic!(
                                "Object parser failed. run again with -u/--unsafe to skip errors."
                            );
                        }

                        if !self.options.is_quiet {
                            println!("Warn: object parser failed");
                        }
                    }
                }
            }
        }

        for path in fs::read_dir(input_path.as_ref().join("objects"))? {
            let actual_os_path = path.unwrap().path();
            let actual_path: &Path = actual_os_path.as_path();
            if metadata(actual_path).unwrap().is_file() {
                let crc32: u32 = match Path::new(actual_path.file_name().unwrap())
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string()
                    .parse::<u32>(){
                    Err(_) => continue,
                    Ok(x) => x
                };
                if index.contains_key(&crc32) {
                    panic!("Ambiguous files for crc32 = {}", crc32);
                }

                index.insert(crc32, actual_os_path);
            }
        }

        dpc_file.seek(SeekFrom::Start(2048))?;

        let mut block_sector_padding_size: u32 = 0;

        let mut block_descriptions: Vec<BlockDescription> = Vec::new();

        let (version_patch, version_minor, mut block_type) = self
            .version_lookup
            .get(&manifest_json.header.version_string)
            .unwrap_or(
                &(manifest_json.header.version_patch.unwrap_or(0),
                  manifest_json.header.version_minor.unwrap_or(0),
                  manifest_json.header.block_type.unwrap_or(0))
            ).clone();

        if version_patch == 0 && !self.options.is_unsafe {
            panic!("Invalid version string for fuel. Use -u/--unsafe to bypass this check and use the invalid string.");
        }

        let mut pool_object_crc32s: HashSet<u32> = HashSet::new();

        if let Some(pool) = &manifest_json.pool {
            for entry in pool.object_entries.iter() {
                pool_object_crc32s.insert(entry.crc32);
            }
        }

        let mut object_padded_size_map: HashMap<u32, u32> = HashMap::new();
        let mut pool_object_compress_map: HashMap<u32, bool> = HashMap::new();

        let tmp_dir = TempDir::new("dpc").expect("Failed to create temp_dir");

        let mut object_count = 0;
        for block in manifest_json.blocks.iter() {
            object_count += block.objects.len();
        }

        let pb = match self.options.is_quiet {
            false => ProgressBar::new(object_count as u64),
            true => ProgressBar::hidden(),
        };

        let mut x = 1;

        for block in manifest_json.blocks.iter() {
            pb.println(format!(
                "Processing block {}/{}",
                x,
                manifest_json.blocks.len()
            ));
            x += 1;

            let start_pos = dpc_file.stream_position()?;

            for object in block.objects.iter() {
                let mut object_file = File::open(index.get(&object.crc32).unwrap().as_path())?;
                let mut buffer: [u8; 24] = [0; 24];
                object_file.read(&mut buffer)?;

                let (_, mut oh) = ObjectHeader::parse(&buffer).unwrap();

                pb.println(format!("Processing {}", oh.crc32));
                if !pool_object_crc32s.contains(&oh.crc32) {
                    if object.compress
                        && oh.compressed_size == 0
                        && self.options.is_lz
                        && self.options.is_optimization
                    {
                        pb.println(format!("Compressing {}", oh.crc32));
                        let mut class_object_data = vec![0; oh.class_object_size as usize];
                        object_file.read(&mut class_object_data)?;
                        let mut data = vec![0; oh.decompressed_size as usize];
                        object_file.read(&mut data)?;
                        let mut compressed_buffer = vec![0; oh.decompressed_size as usize * 2];

                        let compressed_buffer_len = lz::lzrs_compress_optimized(
                            &data[..],
                            oh.decompressed_size as usize,
                            &mut compressed_buffer[..],
                            oh.decompressed_size as usize * 2,
                        )?;
                        oh.compressed_size = compressed_buffer_len as u32 + 8;
                        oh.data_size = oh.class_object_size + oh.compressed_size;

                        oh.write(&mut dpc_file)?;
                        dpc_file.write(&class_object_data)?;
                        dpc_file.write_u32::<BigEndian>(oh.decompressed_size)?;
                        dpc_file.write_u32::<BigEndian>(oh.compressed_size)?;
                        dpc_file.write(&compressed_buffer[0..compressed_buffer_len])?;
                    } else if object.compress
                        && oh.compressed_size == 0
                        && self.options.is_lz
                        && !self.options.is_optimization
                    {
                        pb.println(format!("Compressing {}", oh.crc32));
                        let mut class_object_data = vec![0; oh.class_object_size as usize];
                        object_file.read(&mut class_object_data)?;
                        let mut data = vec![0; oh.decompressed_size as usize + 2];
                        object_file.read(&mut data)?;
                        let mut compressed_buffer = vec![0; oh.decompressed_size as usize * 2];

                        unsafe {
                            let compressed_buffer_len = lz::lzrs_compress(
                                &data[..],
                                oh.decompressed_size as usize,
                                &mut compressed_buffer[..],
                                oh.decompressed_size as usize * 2,
                            )?;
                            oh.compressed_size = compressed_buffer_len as u32 + 8;
                            oh.data_size = oh.class_object_size + oh.compressed_size;

                            oh.write(&mut dpc_file)?;
                            dpc_file.write(&class_object_data)?;
                            dpc_file.write_u32::<BigEndian>(oh.decompressed_size)?;
                            dpc_file.write_u32::<BigEndian>(oh.compressed_size)?;
                            dpc_file.write(&compressed_buffer[0..compressed_buffer_len])?;
                        }
                    } else {
                        oh.write(&mut dpc_file)?;
                        let mut data = vec![0; oh.data_size as usize];
                        object_file.read(&mut data)?;
                        dpc_file.write(&data)?;
                    }
                } else {
                    let mut class_object_data = vec![0; oh.class_object_size as usize];
                    object_file.read(&mut class_object_data)?;

                    if let Some(v) = pool_object_compress_map.get(&oh.crc32) {
                        assert_eq!(
                            *v, object.compress,
                            "Inconsistent compress values for crc32 {}",
                            oh.crc32
                        );
                    } else {
                        pool_object_compress_map.insert(oh.crc32, object.compress);

                        if object.compress
                            && oh.compressed_size == 0
                            && self.options.is_lz
                            && self.options.is_optimization
                        {
                            pb.println(format!("Compressing {}", oh.crc32));
                            let compressed_path = tmp_dir.path().join(oh.crc32.to_string());
                            let mut compressed_file = File::create(compressed_path)?;

                            let mut decompressed_buffer = vec![0; oh.decompressed_size as usize];
                            let mut compressed_buffer = vec![0; oh.decompressed_size as usize * 2];

                            object_file.read(&mut decompressed_buffer)?;

                            let compressed_buffer_len = lz::lzrs_compress_optimized(
                                &decompressed_buffer[..],
                                oh.decompressed_size as usize,
                                &mut compressed_buffer[..],
                                oh.decompressed_size as usize * 2,
                            )?;

                            oh.compressed_size = compressed_buffer_len as u32 + 8;
                            oh.data_size = oh.compressed_size;
                            oh.class_object_size = 0;

                            oh.write(&mut compressed_file)?;
                            compressed_file.write_u32::<BigEndian>(oh.decompressed_size)?;
                            compressed_file.write_u32::<BigEndian>(oh.compressed_size)?;
                            compressed_file.write(&compressed_buffer[0..compressed_buffer_len])?;
                        } else if object.compress
                            && oh.compressed_size == 0
                            && self.options.is_lz
                            && !self.options.is_optimization
                        {
                            pb.println(format!("Compressing {}", oh.crc32));
                            let compressed_path = tmp_dir.path().join(oh.crc32.to_string());
                            let mut compressed_file = File::create(compressed_path)?;

                            let mut decompressed_buffer =
                                vec![0; oh.decompressed_size as usize + 2];
                            let mut compressed_buffer = vec![0; oh.decompressed_size as usize * 2];

                            object_file.read(&mut decompressed_buffer)?;

                            unsafe {
                                let compressed_buffer_len = lz::lzrs_compress(
                                    &decompressed_buffer[..],
                                    oh.decompressed_size as usize,
                                    &mut compressed_buffer[..],
                                    oh.decompressed_size as usize * 2,
                                )?;

                                oh.compressed_size = compressed_buffer_len as u32 + 8;
                                oh.data_size = oh.compressed_size;
                                oh.class_object_size = 0;

                                oh.write(&mut compressed_file)?;
                                compressed_file.write_u32::<BigEndian>(oh.decompressed_size)?;
                                compressed_file.write_u32::<BigEndian>(oh.compressed_size)?;
                                compressed_file
                                    .write(&compressed_buffer[0..compressed_buffer_len])?;
                            }
                        }
                    }

                    if !object_padded_size_map.contains_key(&oh.crc32) {
                        object_padded_size_map.insert(
                            oh.crc32,
                            calculate_padded_size(24 + oh.data_size - oh.class_object_size) >> 11,
                        );
                    }

                    oh.class_object_size = class_object_data.len() as u32;
                    oh.data_size = oh.class_object_size;
                    oh.compressed_size = 0;
                    oh.decompressed_size = 0;
                    oh.write(&mut dpc_file)?;
                    dpc_file.write(&class_object_data)?;
                }

                pb.inc(1);
            }

            let end_pos = dpc_file.stream_position()?;
            let len = (end_pos - start_pos) as u32;

            let block_description = BlockDescription {
                block_type: block_type,
                object_count: block.objects.len() as u32,
                crc32: block.objects[0].crc32,
                data_size: len,
                padded_size: calculate_padded_size(len),
                working_buffer_offset: block.offset,
            };

            block_type = 0;

            block_descriptions.push(block_description);

            let pos = dpc_file.stream_position()?;
            let lll = vec![0x00; calculate_padding_size(pos as u32) as usize];
            dpc_file.write(&lll)?;

            block_sector_padding_size += lll.len() as u32;
        }

        let blocks_padded_size = dpc_file.stream_position()? as u32 - 2048;

        let mut pool_manifest_offset: u32 = 0;
        let mut pool_manifest_padded_size: u32 = 0;
        let mut pool_sector_padding_size: u32 = 0;
        let mut max_pool_decompressed_size = 0;

        if let Some(pool) = &mut manifest_json.pool {
            pb.println("Processing pool");
            pb.set_position(0);
            pb.set_length(pool.object_entry_indices.len() as u64);

            if self.options.is_optimization && !self.unoptimized_pool {
                pb.println("Optimizing the pool");
                let vec_new_reference_records: Vec<JsonReferenceRecord> = pool
                    .reference_records
                    .clone()
                    .into_iter()
                    .unique()
                    .collect();

                for entry in pool.object_entries.iter_mut() {
                    let record = pool.reference_records[entry.reference_record_index as usize - 1];
                    let index = vec_new_reference_records
                        .iter()
                        .position(|&r| r == record)
                        .unwrap();
                    entry.reference_record_index = index as u32 + 1;
                }

                pool.reference_records = vec_new_reference_records;
            }

            pool_manifest_offset = dpc_file.stream_position()? as u32;

            let mut objects_crc32_count_sum: u32 = 0;

            for record in pool.reference_records.iter() {
                objects_crc32_count_sum += record.object_entries_count as u32;
            }

            let pool_header = PoolManifestHeader {
                equals524288: 524288,
                equals2048: 2048,
                objects_crc32_count_sum: objects_crc32_count_sum,
            };

            pool_header.write(&mut dpc_file)?;

            //
            // Pool Manifest
            //

            #[derive(BinWrite)]
            #[binwrite(big)]
            struct PascalArrayU32 {
                len: u32,
                data: Vec<u32>,
            }

            let object_crc32s = PascalArrayU32 {
                len: pool.object_entry_indices.len() as u32,
                data: pool.object_entry_indices.clone(),
            };

            object_crc32s.write(&mut dpc_file)?;

            let mut reference_count_map: HashMap<u32, u32> = HashMap::new();
            for i in object_crc32s.data.iter() {
                reference_count_map.insert(
                    pool.object_entries[*i as usize].crc32,
                    match reference_count_map.get(&pool.object_entries[*i as usize].crc32) {
                        None => 1,
                        Some(x) => *x + 1,
                    },
                );
            }

            let mut vec_crc32s = vec![];
            let mut vec_reference_records_indices = vec![];
            let mut vec_reference_count: Vec<u32> = vec![];
            let mut vec_object_padded_size: Vec<u32> = vec![];
            for entry in pool.object_entries.iter() {
                vec_crc32s.push(entry.crc32);
                vec_reference_records_indices.push(entry.reference_record_index);
                vec_reference_count.push(reference_count_map.get(&entry.crc32).unwrap().clone());
                vec_object_padded_size
                    .push(object_padded_size_map.get(&entry.crc32).unwrap().clone());
            }

            let crc32s = PascalArrayU32 {
                len: vec_crc32s.len() as u32,
                data: vec_crc32s,
            };

            crc32s.write(&mut dpc_file)?;

            let reference_count = PascalArrayU32 {
                len: vec_reference_count.len() as u32,
                data: vec_reference_count,
            };
            reference_count.write(&mut dpc_file)?;

            let object_padded_size = PascalArrayU32 {
                len: vec_object_padded_size.len() as u32,
                data: vec_object_padded_size,
            };

            object_padded_size.write(&mut dpc_file)?;

            let reference_records_indices = PascalArrayU32 {
                len: vec_reference_records_indices.len() as u32,
                data: vec_reference_records_indices,
            };

            reference_records_indices.write(&mut dpc_file)?;

            #[derive(BinWrite)]
            #[binwrite(big)]
            struct PascalArrayReferenceRecord {
                len: u32,
                data: Vec<ReferenceRecord>,
            }

            let mut vec_reference_records = vec![];

            let pos = dpc_file.stream_position()?;
            let end_of_pool_manifest =
                calculate_padded_size(pos as u32 + 28 * pool.reference_records.len() as u32 + 28);

            for record in pool.reference_records.iter() {
                let mut start_chunk_index: u32 = end_of_pool_manifest / 2048;
                for i in 0..record.object_entries_starting_index {
                    let object_entry_index: u32 = pool.object_entry_indices[i as usize];
                    start_chunk_index += object_padded_size_map
                        .get(&pool.object_entries[object_entry_index as usize].crc32)
                        .unwrap();
                }

                let mut end_chunk_index = start_chunk_index;
                for i in record.object_entries_starting_index
                    ..(record.object_entries_starting_index + record.object_entries_count as u32)
                {
                    let object_entry_index: u32 = pool.object_entry_indices[i as usize];
                    end_chunk_index += object_padded_size_map
                        .get(&pool.object_entries[object_entry_index as usize].crc32)
                        .unwrap();
                }

                vec_reference_records.push(ReferenceRecord {
                    start_chunk_index: start_chunk_index,
                    end_chunk_index: end_chunk_index,
                    objects_crc32_starting_index: record.object_entries_starting_index,
                    placeholder_dpc_index: 0,
                    objects_crc32_count: record.object_entries_count,
                    placeholder_times_referenced: 0xFFFFFFFF,
                    placeholder_current_references_shared: 0xFFFFFFFF,
                    placeholder_current_references_weak: 0xFFFFFFFF,
                })
            }

            let reference_records = PascalArrayReferenceRecord {
                len: vec_reference_records.len() as u32,
                data: vec_reference_records,
            };

            reference_records.write(&mut dpc_file)?;

            // terminal
            let llll = ReferenceRecord {
                start_chunk_index: 0,
                end_chunk_index: 0,
                objects_crc32_starting_index: 0,
                placeholder_dpc_index: 0,
                objects_crc32_count: 0,
                placeholder_times_referenced: 0xFFFFFFFF,
                placeholder_current_references_shared: 0xFFFFFFFF,
                placeholder_current_references_weak: 0xFFFFFFFF,
            };
            llll.write(&mut dpc_file)?;

            let pos: i64 = dpc_file.stream_position()? as i64;
            let lll = vec![0xff; calculate_padding_size(pos as u32) as usize];
            dpc_file.write(&lll)?;

            pool_manifest_padded_size = dpc_file.stream_position()? as u32 - pool_manifest_offset;

            for i in pool.object_entry_indices.iter() {
                let crc32 = pool.object_entries[*i as usize].crc32;
                pb.println(format!("Processing {}", crc32));

                let mut object_file =
                    match *pool_object_compress_map.get(&crc32).unwrap() && self.options.is_lz {
                        true => File::open(tmp_dir.path().join(crc32.to_string()))?,
                        false => File::open(index.get(&crc32).unwrap().as_path())?,
                    };

                let mut buffer: [u8; 24] = [0; 24];
                object_file.read(&mut buffer)?;

                let (_, mut oh) = ObjectHeader::parse(&buffer).unwrap();

                max_pool_decompressed_size = max(
                    max_pool_decompressed_size,
                    (oh.decompressed_size + 2047) / 2048,
                );

                oh.data_size = oh.data_size - oh.class_object_size;
                let mut buffer = vec![0; oh.data_size as usize];

                object_file.seek(SeekFrom::Current(oh.class_object_size as i64))?;
                object_file.read(&mut buffer)?;

                oh.class_object_size = 0;
                oh.write(&mut dpc_file)?;
                dpc_file.write(&buffer)?;

                let pos: i64 = dpc_file.stream_position()? as i64;
                let lll = vec![0xff; calculate_padding_size(pos as u32) as usize];
                dpc_file.write(&lll)?;

                pool_sector_padding_size += lll.len() as u32;

                pb.inc(1);
            }
        }

        tmp_dir.close().expect("Failed to delete temp_dir");

        let mut file_padded_size = dpc_file.stream_position()? as u32;

        // HEADER

        dpc_file.seek(SeekFrom::Start(0))?;

        #[derive(BinWrite, Clone, Debug, PartialEq, Eq)]
        #[binwrite(big)]
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

        let mut block_working_buffer_capacity_even = 0;
        let mut block_working_buffer_capacity_odd = 0;

        for i in 0..manifest_json.blocks.len() {
            let block_working_buffer_capacity =
                block_descriptions[i].padded_size + block_descriptions[i].working_buffer_offset;
            if i % 2 == 0 {
                block_working_buffer_capacity_even = max(
                    block_working_buffer_capacity_even,
                    block_working_buffer_capacity,
                );
            } else {
                block_working_buffer_capacity_odd = max(
                    block_working_buffer_capacity_odd,
                    block_working_buffer_capacity,
                );
            }
        }

        let phpa = PrimaryHeaderPartA {
            is_not_rtc: !manifest_json.header.is_rtc as u32,
            block_count: manifest_json.blocks.len() as u32,
            block_working_buffer_capacity_even: block_working_buffer_capacity_even,
            block_working_buffer_capacity_odd: block_working_buffer_capacity_odd,
            padded_size: blocks_padded_size,
            version_patch: version_patch,
            version_minor: version_minor,
        };

        phpa.write(&mut dpc_file)?;

        for block_description in block_descriptions.iter() {
            block_description.write(&mut dpc_file)?;
        }

        dpc_file.seek(SeekFrom::Start(0x720))?;

        #[derive(BinWrite, Clone, Debug, PartialEq, Eq)]
        #[binwrite(big)]
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

        if manifest_json.header.incredi_builder_string.len() == 0 {
            block_sector_padding_size = 0xFFFFFFFF;
            pool_sector_padding_size = 0xFFFFFFFF;
            file_padded_size = 0xFFFFFFFF;
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

        if manifest_json.header.incredi_builder_string.len() != 0 {
            dpc_file.write(manifest_json.header.incredi_builder_string.as_bytes())?;
        } else {
            dpc_file.write(&vec![0xFF; 128])?;
        }

        dpc_file.seek(SeekFrom::Start(0x7c0))?;

        let padding = [0xff; 64];

        dpc_file.write(&padding)?;

        pb.finish_and_clear();

        Ok(())
    }

    fn validate<P: AsRef<Path>>(&self, input_path: &P, output_path: &P) -> Result<()> {
        let mut dpc_file = File::open(input_path.as_ref())?;

        if output_path.as_ref().exists() && !self.options.is_force {
            println!("Output json already exists. You can avoid this interaction by choosing a new output json path or run the program with the -f flag to overwrite the existing json and avoid this prompt for all files. What would you like to do for {}", output_path.as_ref().to_str().unwrap());
            let selection = Select::new()
                .item("Exit")
                .item("Skip this file")
                .item("Overwrite this file")
                .default(0)
                .interact()?;

            match selection {
                0 => panic!("Aborting"),
                1 => return Ok(()),
                2 => (),
                _ => panic!("Invalid choice"),
            };
        }

        let mut primary_header_buffer = Vec::new();
        dpc_file.read_to_end(&mut primary_header_buffer)?;

        #[derive(Serialize, NomBE, Clone, Debug, PartialEq, Eq)]
        struct DPCObjectHeader {
            data_size: u32,
            class_object_size: u32,
            decompressed_size: u32,
            #[nom(
                Verify = "if *compressed_size != 0 { data_size == class_object_size + *compressed_size } else { data_size == class_object_size + decompressed_size }"
            )]
            compressed_size: u32,
            class_crc32: u32,
            #[nom(SkipAfter(data_size))]
            crc32: u32,
        }

        #[derive(Serialize, NomBE, Clone, Debug, PartialEq, Eq)]
        struct DPCPoolObjectHeader {
            data_size: u32,
            class_object_size: u32,
            decompressed_size: u32,
            #[nom(
                Verify = "if *compressed_size != 0 { data_size == class_object_size + *compressed_size } else { data_size == class_object_size + decompressed_size }"
            )]
            compressed_size: u32,
            class_crc32: u32,
            #[nom(SkipAfter(calculate_padded_size(data_size + 24) as usize - 24))]
            crc32: u32,
        }

        #[derive(Serialize, Clone, Debug, PartialEq, Eq)]
        struct DPCBlock {
            objects: Vec<DPCObjectHeader>,
        }

        named_args!(parse_dpcblock(padding_size: usize, object_count: usize)<DPCBlock>, do_parse!(
            objects: count!(DPCObjectHeader::parse, object_count) >>
            take!(padding_size) >>
            (DPCBlock { objects: objects })
        ));

        #[derive(Serialize, Nom, Clone, Debug, PartialEq, Eq)]
        struct DPCPool {
            #[nom(AlignAfter(2048))]
            manifest: PoolManifest,
            #[nom(Count = "manifest.objects_crc32s.len()")]
            #[nom(AlignAfter(2048))]
            objects: Vec<DPCPoolObjectHeader>,
        }

        #[derive(Serialize, Nom, Clone, Debug, PartialEq, Eq)]
        #[nom(Exact)]
        struct DPCFile<'a> {
            #[nom(AlignAfter(2048))]
            primary_header: PrimaryHeader<'a>,
            #[nom(
                PreExec = "let mut x = 0;",
                Count = "primary_header.block_count",
                Parse = "|i| { let res = parse_dpcblock(i, calculate_padding_size(primary_header.block_descriptions[x].data_size) as usize, primary_header.block_descriptions[x].object_count as usize); x += 1; res }"
            )]
            blocks: Vec<DPCBlock>,
            #[nom(Cond = "primary_header.pool_manifest_offset != 0")]
            #[nom(AlignAfter(2048))]
            #[serde(skip_serializing_if = "Option::is_none")]
            pool: Option<DPCPool>,
        }

        let dpc_json = match DPCFile::parse(&primary_header_buffer[..]) {
            Ok((_, h)) => h,
            Err(error) => panic!("{}", error),
        };

        let mut output_file = File::create(output_path.as_ref())?;
        output_file.write(serde_json::to_string_pretty(&dpc_json)?.as_bytes())?;

        Ok(())
    }

    fn compress_object<P: AsRef<Path>>(&self, input_path: &P, output_path: &P) -> Result<()> {
        let mut input_file = File::open(input_path)?;
        let mut output_file = File::create(output_path)?;

        let mut object_header_buffer = [0; 24];
        input_file.read(&mut object_header_buffer)?;

        let mut object_header = match ObjectHeader::parse(&object_header_buffer) {
            Ok((_, h)) => h,
            Err(error) => panic!("{}", error),
        };

        if object_header.compressed_size != 0 {
            panic!("Already compressed");
        }

        let mut class_object_data = vec![0; object_header.class_object_size as usize];
        input_file.read(&mut class_object_data)?;

        let mut decompressed_buffer = vec![0; object_header.decompressed_size as usize];
        input_file.read(&mut decompressed_buffer)?;

        let mut compressed_buffer = vec![0; object_header.decompressed_size as usize * 2];

        let compressed_len = lz::lzrs_compress_optimized(
            &decompressed_buffer[..],
            object_header.decompressed_size as usize,
            &mut compressed_buffer[..],
            object_header.decompressed_size as usize * 2,
        )?;
        compressed_buffer.resize(compressed_len, 0);

        object_header.compressed_size = compressed_len as u32 + 8;
        object_header.data_size = object_header.class_object_size + object_header.compressed_size;

        object_header.write(&mut output_file)?;
        output_file.write(&class_object_data)?;
        output_file.write_u32::<BigEndian>(object_header.decompressed_size)?;
        output_file.write_u32::<BigEndian>(object_header.compressed_size)?;
        output_file.write(&compressed_buffer)?;

        Ok(())
    }

    fn decompress_object<P: AsRef<Path>>(&self, input_path: &P, output_path: &P) -> Result<()> {
        let mut input_file = File::open(input_path)?;
        let mut output_file = File::create(output_path)?;

        let mut object_header_buffer = [0; 24];
        input_file.read(&mut object_header_buffer)?;

        let mut object_header = match ObjectHeader::parse(&object_header_buffer) {
            Ok((_, h)) => h,
            Err(error) => panic!("{}", error),
        };

        if object_header.compressed_size == 0 {
            panic!("Already decompressed");
        }

        let mut class_object_data = vec![0; object_header.class_object_size as usize];
        input_file.read(&mut class_object_data)?;

        let mut decompressed_buffer = vec![0; object_header.decompressed_size as usize];
        let mut compressed_buffer = vec![0; object_header.compressed_size as usize];
        input_file.seek(SeekFrom::Current(8))?;
        input_file.read(&mut compressed_buffer)?;

        lz::lzrs_decompress(
            &compressed_buffer[..],
            object_header.compressed_size as usize,
            &mut decompressed_buffer[..],
            object_header.decompressed_size as usize,
            false,
        )?;

        object_header.compressed_size = 0;
        object_header.data_size = object_header.class_object_size + object_header.decompressed_size;

        object_header.write(&mut output_file)?;
        output_file.write(&class_object_data)?;
        output_file.write(&decompressed_buffer)?;

        Ok(())
    }

    fn split_object<P: AsRef<Path>>(&self, input_path: &P, output_path: &P) -> Result<()> {
        let mut header_path = output_path.as_ref().to_path_buf();
        header_path.push(".header");
        let mut header_file = File::create(header_path)?;

        let mut data_path = output_path.as_ref().to_path_buf();
        data_path.push(".data");
        let mut data_file = File::create(data_path)?;

        let mut input_file = File::open(input_path)?;

        let mut object_header_buffer = [0; 24];
        input_file.read(&mut object_header_buffer)?;

        let object_header = match ObjectHeader::parse(&object_header_buffer) {
            Ok((_, h)) => h,
            Err(error) => panic!("{}", error),
        };

        let mut header_buffer = vec![0; object_header.class_object_size as usize];
        input_file.read(&mut header_buffer)?;
        header_file.write(&header_buffer)?;

        let mut data_buffer =
            vec![0; object_header.data_size as usize - object_header.class_object_size as usize];
        input_file.read(&mut data_buffer)?;
        data_file.write(&data_buffer)?;

        Ok(())
    }

    fn fmt_extract<P: AsRef<Path>>(
        &self,
        input_path: &P,
        output_path: &P,
    ) -> Result<(Vec<u32>, Vec<u32>)> {
        fs::create_dir_all(output_path)?;

        let mut input_file = File::open(input_path)?;

        let mut object_header_buffer = [0; 24];
        input_file.read(&mut object_header_buffer)?;

        let object_header = match ObjectHeader::parse(&object_header_buffer) {
            Ok((_, h)) => h,
            Err(error) => panic!("{}", error),
        };

        println!("{}", &object_header.crc32);

        if let Some(fuel_object_format) =
            fuel_fmt::get_formats(&self.version).get(&object_header.class_crc32)
        {
            let mut header = vec![0; object_header.class_object_size as usize];
            input_file.read(&mut header)?;

            let mut data = vec![0; object_header.decompressed_size as usize];

            if object_header.compressed_size != 0 {
                let mut compresssed_data = vec![0; object_header.compressed_size as usize];
                input_file.read(&mut compresssed_data)?;
                lz::lzrs_decompress(
                    &compresssed_data[..],
                    object_header.compressed_size as usize,
                    &mut data[..],
                    object_header.decompressed_size as usize,
                    false,
                )?;
            } else {
                input_file.read(&mut data)?;
            }

            return fuel_object_format.unpack(&header[..], &data[..], output_path.as_ref());
        }

        return Err(Error::new(ErrorKind::Other, "unsupported format"));
    }

    fn fmt_create<P: AsRef<Path>>(&self, input_path: &P, output_path: &P) -> Result<()> {
        let mut output_file = File::create(output_path)?;

        let p = Path::new(input_path.as_ref().file_stem().unwrap());
        let crc32: u32 = p
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .parse::<u32>()
            .unwrap();
        let class_name = p.extension().unwrap().to_str().unwrap();

        let mut class_names: HashMap<&str, u32> = HashMap::new();
        class_names.insert("Omni_Z", 549480509);
        class_names.insert("Rtc_Z", 705810152);
        class_names.insert("GenWorld_Z", 838505646);
        class_names.insert("LightData_Z", 848525546);
        class_names.insert("Sound_Z", 849267944);
        class_names.insert("MaterialObj_Z", 849861735);
        class_names.insert("RotShape_Z", 866453734);
        class_names.insert("ParticlesData_Z", 954499543);
        class_names.insert("World_Z", 968261323);
        class_names.insert("Warp_Z", 1114947943);
        class_names.insert("Spline_Z", 1135194223);
        class_names.insert("Animation_Z", 1175485833);
        class_names.insert("Mesh_Z", 1387343541);
        class_names.insert("UserDefine_Z", 1391959958);
        class_names.insert("Skin_Z", 1396791303);
        class_names.insert("Bitmap_Z", 1471281566);
        class_names.insert("Fonts_Z", 1536002910);
        class_names.insert("RotShapeData_Z", 1625945536);
        class_names.insert("Surface_Z", 1706265229);
        class_names.insert("SplineGraph_Z", 1910554652);
        class_names.insert("Lod_Z", 1943824915);
        class_names.insert("Material_Z", 2204276779);
        class_names.insert("Node_Z", 2245010728);
        class_names.insert("Binary_Z", 2259852416);
        class_names.insert("CollisionVol_Z", 2398393906);
        class_names.insert("WorldRef_Z", 2906362741);
        class_names.insert("Particles_Z", 3312018398);
        class_names.insert("LodData_Z", 3412401859);
        class_names.insert("Skel_Z", 3611002348);
        class_names.insert("MeshData_Z", 3626109572);
        class_names.insert("SurfaceDatas_Z", 3747817665);
        class_names.insert("MaterialAnim_Z", 3834418854);
        class_names.insert("GwRoad_Z", 3845834591);
        class_names.insert("GameObj_Z", 4096629181);
        class_names.insert("Camera_Z", 4240844041);
        class_names.insert("AnimFrame_Z", 4117606081);
        class_names.insert("CameraZone_Z", 3979333606);
        class_names.insert("Occluder_Z", 72309972);
        class_names.insert("Graph_Z", 1390918523);
        class_names.insert("Light_Z", 1918499807);
        class_names.insert("HFogData_Z", 3210467954);
        class_names.insert("HFog_Z", 2735949084);
        class_names.insert("Flare_Z", 2203168663);
        class_names.insert("FlareData_Z", 1393846573);

        let class_crc32: u32;
        if let Some(v) = class_names.get(class_name) {
            class_crc32 = *v
        } else {
            class_crc32 = class_name.parse::<u32>().unwrap();
        }

        if let Some(fuel_object_format) = fuel_fmt::get_formats(&self.version).get(&class_crc32) {
            let mut header: Vec<u8> = Vec::new();
            let mut body: Vec<u8> = Vec::new();
            let res = fuel_object_format.pack(input_path.as_ref(), &mut header, &mut body);
            if res.is_err() {
                if !self.options.is_unsafe {
                    panic!(
                        "Object parser failed. run again with -u/--unsafe to skip errors. {}",
                        crc32
                    );
                }

                if !self.options.is_quiet {
                    println!("Warn: object parser failed {}", crc32);
                }
            }

            let object_header = ObjectHeader {
                data_size: body.len() as u32 + header.len() as u32,
                class_object_size: header.len() as u32,
                decompressed_size: body.len() as u32,
                compressed_size: 0,
                class_crc32: class_crc32,
                crc32: crc32,
            };

            object_header.write(&mut output_file)?;
            header.write(&mut output_file)?;
            body.write(&mut output_file)?;
        } else {
            return Err(Error::new(ErrorKind::Other, "unsupported format"));
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::ffi::OsStr;
    use std::path::Path;

    use checksumdir::checksumdir;
    use checksums::hash_file;
    use checksums::Algorithm;
    use tempdir::TempDir;
    use test_generator::test_resources;

    use crate::base_dpc::Options;
    use crate::base_dpc::DPC;
    use crate::fuel_dpc::{FuelDPC, ObjectHeader};
    use nom_derive::Parse;
    use std::fs;
    use std::fs::File;
    use std::io::{Read, Seek, SeekFrom};

    #[test_resources("D:/SteamLibrary/steamapps/common/FUEL/**/*.DPC")]
    fn test_fuel_dpc_validate(path: &str) {
        let dpc = FuelDPC::new(
            &Options {
                is_quiet: true,
                is_force: true,
                is_unsafe: false,
                is_lz: false,
                is_optimization: false,
                is_recursive: false,
            },
            &vec![],
        );

        let tmp_dir = TempDir::new("dpc").expect("Failed to create temp_dir");

        let dpc_file = Path::new(path);
        let dpc_json = tmp_dir.path().join("temp.json");

        dpc.validate(&dpc_file, &dpc_json.as_path()).unwrap();

        tmp_dir.close().expect("Failed to delete temp_dir");
    }

    #[test_resources("D:/SteamLibrary/steamapps/common/FUEL/**/*.DPC")]
    fn test_fuel_dpc_normal(path: &str) {
        let mut dpc = FuelDPC::new(
            &Options {
                is_quiet: true,
                is_force: true,
                is_unsafe: true,
                is_lz: false,
                is_optimization: false,
                is_recursive: false,
            },
            &vec![],
        );

        let tmp_dir = TempDir::new("dpc").expect("Failed to create temp_dir");

        let dpc_file = Path::new(path);
        let dpc_file_2 = tmp_dir.path().join("TEMP.DPC");
        let dpc_directory = tmp_dir.path().join("TEMP");

        dpc.extract(&dpc_file, &dpc_directory.as_path()).unwrap();
        dpc.create(&dpc_directory, &dpc_file_2).unwrap();
        assert_eq!(
            hash_file(dpc_file, Algorithm::SHA1),
            hash_file(dpc_file_2.as_path(), Algorithm::SHA1)
        );

        tmp_dir.close().expect("Failed to delete temp_dir");
    }

    #[test_resources("D:/SteamLibrary/steamapps/common/FUEL/**/*.DPC")]
    // #[test_resources("D:/SteamLibrary/steamapps/common/FUEL/WORLD/USA1.DPC")]
    // #[test_resources("D:/SteamLibrary/steamapps/common/FUEL/RTC/TELE_01.DPC")]
    // #[test_resources("D:/SteamLibrary/steamapps/common/FUEL/DATAS/VEH.DPC")]
    fn test_fuel_dpc_recursive(path: &str) {
        let mut dpc = FuelDPC::new(
            &Options {
                is_quiet: true,
                is_force: true,
                is_unsafe: false,
                is_lz: true,
                is_optimization: false,
                is_recursive: true,
            },
            &vec![],
        );

        let tmp_dir = TempDir::new("dpc").expect("Failed to create temp_dir");

        let dpc_file = Path::new(path);
        let dpc_file_2 = tmp_dir.path().join("TEMP.DPC");
        let dpc_directory = tmp_dir.path().join("TEMP");

        dpc.extract(&dpc_file, &dpc_directory.as_path()).unwrap();
        dpc.create(&dpc_directory.as_path(), &dpc_file_2.as_path())
            .unwrap();
        assert_eq!(
            hash_file(dpc_file, Algorithm::SHA1),
            hash_file(dpc_file_2.as_path(), Algorithm::SHA1)
        );

        tmp_dir.close().expect("Failed to delete temp_dir");
    }

    #[test_resources("D:/SteamLibrary/steamapps/common/FUEL/**/*.DPC")]
    fn test_fuel_dpc_extract(path: &str) {
        let mut dpc = FuelDPC::new(
            &Options {
                is_quiet: true,
                is_force: true,
                is_unsafe: false,
                is_lz: true,
                is_optimization: false,
                is_recursive: false,
            },
            &vec![],
        );

        let tmp_dir = TempDir::new("dpc").expect("Failed to create temp_dir");

        let dpc_file = Path::new(path);
        let dpc_directory = tmp_dir.path().join("TEMP");

        dpc.extract(&dpc_file, &dpc_directory.as_path()).unwrap();

        for entry in fs::read_dir(dpc_directory.join("objects")).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if !path.is_dir() {
                let mut f = File::open(path).unwrap();

                let mut object_header_buffer = [0; 24];
                f.read(&mut object_header_buffer).unwrap();

                let oh = match ObjectHeader::parse(&object_header_buffer) {
                    Ok((_, h)) => h,
                    Err(error) => panic!("{}", error),
                };

                f.seek(SeekFrom::End(0)).unwrap();

                assert_eq!(
                    oh.data_size,
                    oh.class_object_size
                        + if oh.compressed_size != 0 {
                            oh.compressed_size
                        } else {
                            oh.decompressed_size
                        }
                );
                assert_eq!(oh.data_size + 24, f.stream_position().unwrap() as u32);
            }
        }

        tmp_dir.close().expect("Failed to delete temp_dir");
    }

    #[test_resources("D:/SteamLibrary/steamapps/common/FUEL/**/*.DPC")]
    fn test_fuel_dpc_optimized(path: &str) {
        let mut dpc = FuelDPC::new(
            &Options {
                is_quiet: true,
                is_force: true,
                is_unsafe: false,
                is_lz: true,
                is_optimization: true,
                is_recursive: false,
            },
            &vec![&OsStr::new("--unoptimized-pool")],
        );

        let tmp_dir = TempDir::new("dpc").expect("Failed to create temp_dir");

        let dpc_file = Path::new(path);
        let dpc_file_2 = tmp_dir.path().join("TEMP.DPC");
        let dpc_directory = tmp_dir.path().join("TEMP");

        dpc.extract(&dpc_file, &dpc_directory.as_path()).unwrap();
        let cx = checksumdir(dpc_directory.as_os_str().to_str().unwrap()).unwrap();

        dpc.create(&dpc_directory, &dpc_file_2).unwrap();

        dpc.extract(&dpc_file_2, &dpc_directory).unwrap();
        let cy = checksumdir(dpc_directory.as_os_str().to_str().unwrap()).unwrap();

        assert_eq!(cx, cy);

        tmp_dir.close().expect("Failed to delete temp_dir");
    }

    #[test_resources("D:/SteamLibrary/steamapps/common/FUEL/**/*.DPC")]
    fn test_fuel_dpc_mixed_full(path: &str) {
        let mut dpc = FuelDPC::new(
            &Options {
                is_quiet: true,
                is_force: true,
                is_unsafe: true,
                is_lz: true,
                is_optimization: false,
                is_recursive: false,
            },
            &vec![],
        );

        let tmp_dir = TempDir::new("dpc").expect("Failed to create temp_dir");

        let dpc_file = Path::new(path);
        let dpc_file_2 = tmp_dir.path().join("TEMP.DPC");
        let dpc_directory = tmp_dir.path().join("TEMP");

        dpc.extract(&dpc_file, &dpc_directory.as_path()).unwrap();

        dpc.create(&dpc_directory, &dpc_file_2).unwrap();

        assert_eq!(
            hash_file(dpc_file, Algorithm::SHA1),
            hash_file(dpc_file_2.as_path(), Algorithm::SHA1)
        );

        tmp_dir.close().expect("Failed to delete temp_dir");
    }

    #[test_resources("D:/SteamLibrary/steamapps/common/FUEL/**/*.DPC")]
    fn test_fuel_dpc_mixed_lazy(path: &str) {
        let mut dpc_extract = FuelDPC::new(
            &Options {
                is_quiet: true,
                is_force: true,
                is_unsafe: false,
                is_lz: true,
                is_optimization: false,
                is_recursive: false,
            },
            &vec![],
        );

        let mut dpc_create = FuelDPC::new(
            &Options {
                is_quiet: true,
                is_force: true,
                is_unsafe: false,
                is_lz: false,
                is_optimization: false,
                is_recursive: false,
            },
            &vec![],
        );

        let tmp_dir = TempDir::new("dpc").expect("Failed to create temp_dir");

        let dpc_file = Path::new(path);
        let dpc_file_2 = tmp_dir.path().join("TEMP.DPC");
        let dpc_directory = tmp_dir.path().join("TEMP");

        dpc_extract
            .extract(&dpc_file, &dpc_directory.as_path())
            .unwrap();
        let cx = checksumdir(dpc_directory.join("objects").as_os_str().to_str().unwrap()).unwrap();

        dpc_create.create(&dpc_directory, &dpc_file_2).unwrap();

        dpc_extract.extract(&dpc_file_2, &dpc_directory).unwrap();
        let cy = checksumdir(dpc_directory.join("objects").as_os_str().to_str().unwrap()).unwrap();

        assert_eq!(cx, cy);

        tmp_dir.close().expect("Failed to delete temp_dir");
    }

    #[test_resources("D:/programming/widberg/dpc/data/CT.DGC")]
    fn test_fuel_dpc_mixed_lazy_big_endian(path: &str) {
        let mut dpc_extract = FuelDPC::new(
            &Options {
                is_quiet: true,
                is_force: true,
                is_unsafe: true,
                is_lz: true,
                is_optimization: false,
                is_recursive: false,
            },
            &vec![],
        );

        let mut dpc_create = FuelDPC::new(
            &Options {
                is_quiet: true,
                is_force: true,
                is_unsafe: true,
                is_lz: false,
                is_optimization: false,
                is_recursive: false,
            },
            &vec![],
        );

        let tmp_dir = TempDir::new("dpc").expect("Failed to create temp_dir");

        let dpc_file = Path::new(path);
        let dpc_file_2 = tmp_dir.path().join("TEMP.DPC");
        let dpc_directory = tmp_dir.path().join("TEMP");

        dpc_extract
            .extract(&dpc_file, &dpc_directory.as_path())
            .unwrap();
        let cx = checksumdir(dpc_directory.join("objects").as_os_str().to_str().unwrap()).unwrap();

        dpc_create.create(&dpc_directory, &dpc_file_2).unwrap();

        dpc_extract.extract(&dpc_file_2, &dpc_directory).unwrap();
        let cy = checksumdir(dpc_directory.join("objects").as_os_str().to_str().unwrap()).unwrap();

        assert_eq!(cx, cy);

        tmp_dir.close().expect("Failed to delete temp_dir");
    }
}
