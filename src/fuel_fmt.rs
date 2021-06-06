use std::path::Path;
use std::fs::File;
use std::io::Result;
use serde::Deserialize;
use serde::Serialize;
use nom_derive::{NomLE, Parse};
use std::io::Write;

#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
struct ResourceObjectZ {
	friendly_name_crc32: u32,
	crc32s_count: u32,
	#[nom(Cond = "crc32s_count != 0")]
	#[nom(Count = "crc32s_count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    crc32s: Option<Vec<u32>>,
}

#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
struct MaterialZ {
	#[nom(Count = "34")]
	vertex_shader_constant_fs: Vec<f32>,
	diffuse_bitmap_crc32: u32,
	unknown_bitmap_crc320: u32,
	metal_bitmap_crc32: u32,
	unknown_bitmap_crc321: u32,
	grey_bitmap_crc32: u32,
	normal_bitmap_crc32: u32,
	dirt_bitmap_crc32: u32,
	unknown_bitmap_crc322: u32,
	unknown_bitmap_crc323: u32,
	unknown0: u8,
	#[nom(Count = "0")]
	bitmap_crc32s: Vec<u32>,
}

#[derive(Serialize, Deserialize, NomLE)]
struct MaterialObject {
    #[nom(Parse = "ResourceObjectZ::parse")]
	resource_object: ResourceObjectZ,
    #[nom(Parse = "MaterialZ::parse")]
	material: MaterialZ,
}

fn fuel_fmt_extract_material_z<P: AsRef<Path>>(header: &[u8], data: &[u8], output_path: &P) -> Result<()> {
	let json_path = output_path.as_ref().join("object.json");
	let mut output_file = File::create(json_path)?;

	let resource_object = match ResourceObjectZ::parse(&header) {
		Ok((_, h)) => h,
		Err(error) => panic!("{}", error),
	};

	let material = match MaterialZ::parse(&data) {
		Ok((_, h)) => h,
		Err(error) => panic!("{}", error),
	};

	let materialObject = MaterialObject {
		resource_object,
		material,
	};

	output_file.write(serde_json::to_string_pretty(&materialObject)?.as_bytes())?;

	Ok(())
}
