use std::path::Path;
use std::fs::File;
use std::io::Result;
use serde::Deserialize;
use serde::Serialize;
use nom_derive::{NomLE, Parse};
use std::io::Write;
use nom::number::complete::*;

#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
struct ResourceObjectZ {
	friendly_name_crc32: u32,
	#[nom(Cond = "i.len() != 0")]
	#[nom(Count = "{ if i.len() != 0 { le_u32(i)?.1 } else { 0 } }")]
    #[serde(skip_serializing_if = "Option::is_none")]
    crc32s: Option<Vec<u32>>,
}

static mut MATERIAL_BITMAP_CRC32S_COUNT: usize = 0;

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
	#[nom(Cond = "i.len() != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
	unknown0: Option<u8>,
	#[nom(Count = "unsafe { MATERIAL_BITMAP_CRC32S_COUNT }")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
	bitmap_crc32s: Vec<u32>,
}

#[derive(Serialize, Deserialize)]
struct MaterialObject {
	resource_object: ResourceObjectZ,
	material: MaterialZ,
}

pub fn fuel_fmt_extract_material_z(header: &[u8], data: &[u8], output_path: &Path) -> Result<()> {
	let json_path = output_path.join("object.json");
	let mut output_file = File::create(json_path)?;

	let resource_object = match ResourceObjectZ::parse(&header) {
		Ok((_, h)) => h,
		Err(error) => panic!("{}", error),
	};

	unsafe { MATERIAL_BITMAP_CRC32S_COUNT = if let Some(crc32s) = resource_object.crc32s.clone() { crc32s.len() } else { 0 } };

	let material = match MaterialZ::parse(&data) {
		Ok((_, h)) => h,
		Err(error) => panic!("{}", error),
	};

	let material_object = MaterialObject {
		resource_object,
		material,
	};

	output_file.write(serde_json::to_string_pretty(&material_object)?.as_bytes())?;

	Ok(())
}

#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
struct UserDefineZ {
	#[nom(Map = "|x: &[u8]| String::from_utf8(x.to_vec()).unwrap()", Take = "le_u32(i)?.1 as usize")]
	data: String,
}

#[derive(Serialize, Deserialize)]
struct UserDefineObject {
	resource_object: ResourceObjectZ,
	user_define: UserDefineZ,
}

pub fn fuel_fmt_extract_user_define_z(header: &[u8], data: &[u8], output_path: &Path) -> Result<()> {
	let json_path = output_path.join("object.json");
	let mut output_file = File::create(json_path)?;

	let resource_object = match ResourceObjectZ::parse(&header) {
		Ok((_, h)) => h,
		Err(error) => panic!("{}", error),
	};

	let user_define = match UserDefineZ::parse(&data) {
		Ok((_, h)) => h,
		Err(error) => panic!("{}", error),
	};

	let material_object = UserDefineObject {
		resource_object,
		user_define,
	};

	output_file.write(serde_json::to_string_pretty(&material_object)?.as_bytes())?;

	Ok(())
}
