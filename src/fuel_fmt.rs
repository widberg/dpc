use std::path::Path;
use std::fs::File;
use std::io::Result;
use serde::Deserialize;
use serde::Serialize;
use nom_derive::{NomLE, Parse};
use std::io::Write;
use nom::number::complete::*;
use nom::*;
use hound;
use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt};

#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
struct ResourceObjectZ {
	friendly_name_crc32: u32,
	#[nom(Cond = "i.len() != 0")]
	#[nom(Count = "{ if i.len() != 0 { le_u32(i)?.1 } else { 0 } }")]
    #[serde(skip_serializing_if = "Option::is_none")]
    crc32s: Option<Vec<u32>>,
}

#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
struct ObjectZ {
	friendly_name_crc32: u32,
	#[nom(Cond = "i.len() == 94")]
    #[serde(skip_serializing_if = "Option::is_none")]
	crc32_or_zero: Option<u32>,
	#[nom(Cond = "i.len() > 94")]
	#[nom(Parse = "|i| length_count!(i, le_u32, le_u32)")]
    #[serde(skip_serializing_if = "Option::is_none")]
	crc32s: Option<Vec<u32>>,
	#[nom(Count = "22")]
	floats: Vec<f32>,
	short: u16,
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

	let object = MaterialObject {
		resource_object,
		material,
	};

	output_file.write(serde_json::to_string_pretty(&object)?.as_bytes())?;

	Ok(())
}

#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
struct UserDefineZ {
	#[nom(Map = "|x: &str| String::from(x)", Parse = "|i| take_str!(i, le_u32(i)?.1 as usize)")]
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

	let object = UserDefineObject {
		resource_object,
		user_define,
	};

	output_file.write(serde_json::to_string_pretty(&object)?.as_bytes())?;

	Ok(())
}

#[derive(Serialize, Deserialize, NomLE)]
struct GameObjZChild {
	#[nom(Map = "|x: &str| String::from(x)", Parse = "|i| take_str!(i, le_u32(i)?.1 as usize)")]
	str: String,
	is_in_world: u32,
	#[nom(Parse = "{ |i| length_count!(i, le_u32, le_u32) }")]
	crc32s: Vec<u32>,
}

#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
struct GameObjZ {
	#[nom(Parse = "{ |i| length_count!(i, le_u32, GameObjZChild::parse) }")]
	children: Vec<GameObjZChild>,
}

#[derive(Serialize, Deserialize)]
struct GameObjObject {
	resource_object: ResourceObjectZ,
	game_obj: GameObjZ,
}

pub fn fuel_fmt_extract_game_obj_z(header: &[u8], data: &[u8], output_path: &Path) -> Result<()> {
	let json_path = output_path.join("object.json");
	let mut output_file = File::create(json_path)?;

	let resource_object = match ResourceObjectZ::parse(&header) {
		Ok((_, h)) => h,
		Err(error) => panic!("{}", error),
	};

	let game_obj = match GameObjZ::parse(&data) {
		Ok((_, h)) => h,
		Err(error) => panic!("{}", error),
	};

	let object = GameObjObject {
		resource_object,
		game_obj,
	};

	output_file.write(serde_json::to_string_pretty(&object)?.as_bytes())?;

	Ok(())
}


#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
struct SurfaceDatasZ {
	one: u32,
}

#[derive(Serialize, Deserialize)]
struct SurfaceDatasObject {
	resource_object: ResourceObjectZ,
	surface_datas: SurfaceDatasZ,
}

pub fn fuel_fmt_extract_surface_datas_z(header: &[u8], data: &[u8], output_path: &Path) -> Result<()> {
	let json_path = output_path.join("object.json");
	let mut output_file = File::create(json_path)?;

	let resource_object = match ResourceObjectZ::parse(&header) {
		Ok((_, h)) => h,
		Err(error) => panic!("{}", error),
	};

	let surface_datas = match SurfaceDatasZ::parse(&data) {
		Ok((_, h)) => h,
		Err(error) => panic!("{}", error),
	};

	let object = SurfaceDatasObject {
		resource_object,
		surface_datas,
	};

	output_file.write(serde_json::to_string_pretty(&object)?.as_bytes())?;

	Ok(())
}

#[derive(Serialize, Deserialize, NomLE)]
struct FontZCharacter {
	id: u32,
	material_index: u32,
	point: f32,
	height: f32,
	y: f32,
	x: f32,
	width: f32,
}

#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
struct FontZ {
	#[nom(Parse = "{ |i| length_count!(i, le_u32, FontZCharacter::parse) }")]
	characters: Vec<FontZCharacter>,
	#[nom(Parse = "{ |i| length_count!(i, le_u32, le_u32) }")]
	material_crc32s: Vec<u32>,
}

#[derive(Serialize, Deserialize)]
struct FontObject {
	resource_object: ResourceObjectZ,
	font: FontZ,
}

pub fn fuel_fmt_extract_font_z(header: &[u8], data: &[u8], output_path: &Path) -> Result<()> {
	let json_path = output_path.join("object.json");
	let mut output_file = File::create(json_path)?;

	let resource_object = match ResourceObjectZ::parse(&header) {
		Ok((_, h)) => h,
		Err(error) => panic!("{}", error),
	};

	let font = match FontZ::parse(&data) {
		Ok((_, h)) => h,
		Err(error) => panic!("{}", error),
	};

	let object = FontObject {
		resource_object,
		font,
	};

	output_file.write(serde_json::to_string_pretty(&object)?.as_bytes())?;

	Ok(())
}

#[derive(Serialize, Deserialize, NomLE)]
struct MaterialObjZEntry {
	array_name_crc32: u32,
	#[nom(Parse = "{ |i| length_count!(i, le_u32, le_u32) }")]
	material_anim_crc32s: Vec<u32>,
}

#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
struct MaterialObjZ {
	#[nom(Parse = "{ |i| length_count!(i, le_u32, MaterialObjZEntry::parse) }")]
	entries: Vec<MaterialObjZEntry>,
}

#[derive(Serialize, Deserialize)]
struct MaterialObjObject {
	resource_object: ResourceObjectZ,
	material_obj: MaterialObjZ,
}

pub fn fuel_fmt_extract_material_obj_z(header: &[u8], data: &[u8], output_path: &Path) -> Result<()> {
	let json_path = output_path.join("object.json");
	let mut output_file = File::create(json_path)?;

	let resource_object = match ResourceObjectZ::parse(&header) {
		Ok((_, h)) => h,
		Err(error) => panic!("{}", error),
	};

	let material_obj = match MaterialObjZ::parse(&data) {
		Ok((_, h)) => h,
		Err(error) => panic!("{}", error),
	};

	let object = MaterialObjObject {
		resource_object,
		material_obj,
	};

	output_file.write(serde_json::to_string_pretty(&object)?.as_bytes())?;

	Ok(())
}

#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
struct MaterialAnimZ {
	unknown0: u32,
    unknown1: u32,
    unknown2: u32,
    unknown3: u32,
    unknown4: u32,
    unknown5: u32,
    unknown6: u32,
    unknown7: u32,
    unknown8: u32,
    unknown9: u32,
    unknown10: u32,
    unknown11: u32,
    unknown12: u32,
    unknown13: u32,
    unknown14: u32,
    material_crc32: u32,
    unknown15: u32,
    unknown16: u8,
}

#[derive(Serialize, Deserialize)]
struct MaterialAnimObject {
	resource_object: ResourceObjectZ,
	material_anim: MaterialAnimZ,
}

pub fn fuel_fmt_extract_material_anim_z(header: &[u8], data: &[u8], output_path: &Path) -> Result<()> {
	let json_path = output_path.join("object.json");
	let mut output_file = File::create(json_path)?;

	let resource_object = match ResourceObjectZ::parse(&header) {
		Ok((_, h)) => h,
		Err(error) => panic!("{}", error),
	};

	let material_anim = match MaterialAnimZ::parse(&data) {
		Ok((_, h)) => h,
		Err(error) => panic!("{}", error),
	};

	let object = MaterialAnimObject {
		resource_object,
		material_anim,
	};

	output_file.write(serde_json::to_string_pretty(&object)?.as_bytes())?;

	Ok(())
}

#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
struct MeshDataZ {
	not_traffic_tm_or_p_moto: u32,
    zero0: u32,
    zero1: u32,
    zero2: u32,
    zero3: u32,
}

#[derive(Serialize, Deserialize)]
struct MeshDataObject {
	resource_object: ResourceObjectZ,
	mesh_data: MeshDataZ,
}

pub fn fuel_fmt_extract_mesh_data_z(header: &[u8], data: &[u8], output_path: &Path) -> Result<()> {
	let json_path = output_path.join("object.json");
	let mut output_file = File::create(json_path)?;

	let resource_object = match ResourceObjectZ::parse(&header) {
		Ok((_, h)) => h,
		Err(error) => panic!("{}", error),
	};

	let mesh_data = match MeshDataZ::parse(&data) {
		Ok((_, h)) => h,
		Err(error) => panic!("{}", error),
	};

	let object = MeshDataObject {
		resource_object,
		mesh_data,
	};

	output_file.write(serde_json::to_string_pretty(&object)?.as_bytes())?;

	Ok(())
}

#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
struct RotShapeDataZ {
    one: u32,
	#[nom(Parse = "{ |i| length_count!(i, le_u32, le_u16) }")]
	shorts: Vec<u16>,
	#[nom(Map = "|x: &[u8]| x.to_vec()", Take = "shorts.len() * 28")]
    padding: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
struct RotShapeDataObject {
	resource_object: ResourceObjectZ,
	rot_shape_data: RotShapeDataZ,
}

pub fn fuel_fmt_extract_rot_shape_data_z(header: &[u8], data: &[u8], output_path: &Path) -> Result<()> {
	let json_path = output_path.join("object.json");
	let mut output_file = File::create(json_path)?;

	let resource_object = match ResourceObjectZ::parse(&header) {
		Ok((_, h)) => h,
		Err(error) => panic!("{}", error),
	};

	let rot_shape_data = match RotShapeDataZ::parse(&data) {
		Ok((_, h)) => h,
		Err(error) => panic!("{}", error),
	};

	let object = RotShapeDataObject {
		resource_object,
		rot_shape_data,
	};

	output_file.write(serde_json::to_string_pretty(&object)?.as_bytes())?;

	Ok(())
}

#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
struct ParticlesDataZ {
    equals257: u32,
    position_x: f32,
    position_y: f32,
    position_z: f32,
    velocity_x: f32,
    velocity_y: f32,
    velocity_z: f32,
	#[nom(Parse = "{ |i| length_count!(i, le_u32, le_u16) }")]
	shorts: Vec<u16>,
	zero: u32,
}

#[derive(Serialize, Deserialize)]
struct ParticlesDataObject {
	resource_object: ResourceObjectZ,
	particles_data: ParticlesDataZ,
}

pub fn fuel_fmt_extract_particles_data_z(header: &[u8], data: &[u8], output_path: &Path) -> Result<()> {
	let json_path = output_path.join("object.json");
	let mut output_file = File::create(json_path)?;

	let resource_object = match ResourceObjectZ::parse(&header) {
		Ok((_, h)) => h,
		Err(error) => panic!("{}", error),
	};

	let particles_data = match ParticlesDataZ::parse(&data) {
		Ok((_, h)) => h,
		Err(error) => panic!("{}", error),
	};

	let object = ParticlesDataObject {
		resource_object,
		particles_data,
	};

	output_file.write(serde_json::to_string_pretty(&object)?.as_bytes())?;

	Ok(())
}

#[derive(Serialize, Deserialize)]
struct BinaryObject {
	resource_object: ResourceObjectZ,
}

pub fn fuel_fmt_extract_binary_z(header: &[u8], data: &[u8], output_path: &Path) -> Result<()> {
	let json_path = output_path.join("object.json");
	let mut output_file = File::create(json_path)?;

	let bin_path = output_path.join("data.bin");
	let mut output_bin_file = File::create(bin_path)?;

	let resource_object = match ResourceObjectZ::parse(&header) {
		Ok((_, h)) => h,
		Err(error) => panic!("{}", error),
	};

	let object = BinaryObject {
		resource_object,
	};

	output_file.write(serde_json::to_string_pretty(&object)?.as_bytes())?;

	output_bin_file.write(&data)?;

	Ok(())
}

#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
struct CameraZ {
    angle_of_view: f32,
    zero: f32,
	node_crc32: u32,
}

#[derive(Serialize, Deserialize)]
struct CameraObject {
	object: ObjectZ,
	camera: CameraZ,
}

pub fn fuel_fmt_extract_camera_z(header: &[u8], data: &[u8], output_path: &Path) -> Result<()> {
	let json_path = output_path.join("object.json");
	let mut output_file = File::create(json_path)?;

	let object = match ObjectZ::parse(&header) {
		Ok((_, h)) => h,
		Err(error) => panic!("{}", error),
	};

	let camera = match CameraZ::parse(&data) {
		Ok((_, h)) => h,
		Err(error) => panic!("{}", error),
	};

	let object = CameraObject {
		object,
		camera,
	};

	output_file.write(serde_json::to_string_pretty(&object)?.as_bytes())?;

	Ok(())
}

#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
struct WarpZ {
	some: u32,
    u0: f32,        
    u1: f32,
    u2: f32,
    u3: f32,
    u4: f32,
    u5: f32,
    u6: f32,
    u7: f32,
    u8: f32,
    u9: f32,
    u10: f32,
    u11: f32,
    u12: f32,
    u13: f32,
    u14: f32,
    u15: f32,
    u16: f32,
    u17: f32,
    u18: f32,
    u19: f32,
    u20: f32,
    u21: f32,
    u22: f32,
    u23: f32,
    u24: f32,
    u25: f32,
    u26: f32,
    u27: f32,
    u28: f32,
    u29: f32,
    u30: f32,
    u31: f32,
    u32: f32,
    u33: f32,
    u34: f32,
}

#[derive(Serialize, Deserialize)]
struct WarpObject {
	resource_object: ResourceObjectZ,
	warp: WarpZ,
}

pub fn fuel_fmt_extract_warp_z(header: &[u8], data: &[u8], output_path: &Path) -> Result<()> {
	let json_path = output_path.join("object.json");
	let mut output_file = File::create(json_path)?;

	let resource_object = match ResourceObjectZ::parse(&header) {
		Ok((_, h)) => h,
		Err(error) => panic!("{}", error),
	};

	let warp = match WarpZ::parse(&data) {
		Ok((_, h)) => h,
		Err(error) => panic!("{}", error),
	};

	let object = WarpObject {
		resource_object,
		warp,
	};

	output_file.write(serde_json::to_string_pretty(&object)?.as_bytes())?;

	Ok(())
}

#[derive(Serialize, Deserialize, NomLE)]
struct SplineZUnknown0 {
	x: f32,
	y: f32,
	z: f32,
}

#[derive(Serialize, Deserialize, NomLE)]
struct SplineZUnknown1 {
	#[nom(Count = "240")]
	data: Vec<u8>,
}

#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
struct SplineZ {
	#[nom(Parse = "{ |i| length_count!(i, le_u32, SplineZUnknown0::parse) }")]
    unknown0s: Vec<SplineZUnknown0>,
	#[nom(Parse = "{ |i| length_count!(i, le_u32, SplineZUnknown1::parse) }")]
    unknown1s: Vec<SplineZUnknown1>,
    unknown2: f32,
    unknown3: f32,
    unknown4: f32,
    unknown5: f32,
    unknown6: f32,
}

#[derive(Serialize, Deserialize)]
struct SplineObject {
	object: ObjectZ,
	spline: SplineZ,
}

pub fn fuel_fmt_extract_spline_z(header: &[u8], data: &[u8], output_path: &Path) -> Result<()> {
	let json_path = output_path.join("object.json");
	let mut output_file = File::create(json_path)?;

	let object = match ObjectZ::parse(&header) {
		Ok((_, h)) => h,
		Err(error) => panic!("{}", error),
	};

	let spline = match SplineZ::parse(&data) {
		Ok((_, h)) => h,
		Err(error) => panic!("{}", error),
	};

	let object = SplineObject {
		object,
		spline,
	};

	output_file.write(serde_json::to_string_pretty(&object)?.as_bytes())?;

	Ok(())
}

#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
struct SoundZHeader {
    friendly_name_crc32: u32,
	#[serde(skip_serializing)]
    sample_rate: u32,
	#[serde(skip_serializing)]
    data_size: u32,
    sound_type: u32,
}

#[derive(Serialize, Deserialize)]
struct SoundObject {
	sound_header: SoundZHeader,
}

pub fn fuel_fmt_extract_sound_z(header: &[u8], data: &[u8], output_path: &Path) -> Result<()> {
	let json_path = output_path.join("object.json");
	let mut output_file = File::create(json_path)?;

	let wav_path = output_path.join("data.wav");

	let sound_header = match SoundZHeader::parse(&header) {
		Ok((_, h)) => h,
		Err(error) => panic!("{}", error),
	};

	let spec = hound::WavSpec {
		channels: 1,
		sample_rate: sound_header.sample_rate,
		bits_per_sample: 16,
		sample_format: hound::SampleFormat::Int,
	};

	let number_of_samples = sound_header.data_size / (spec.bits_per_sample / 8) as u32;

	let mut parent_writer = hound::WavWriter::create(wav_path, spec).unwrap();
	let mut writer = parent_writer.get_i16_writer(number_of_samples);

	let mut data_cursor = Cursor::new(&data);

	for _ in 0..number_of_samples {
		writer.write_sample(data_cursor.read_i16::<LittleEndian>()?);
	}

	let object = SoundObject {
		sound_header,
	};

	output_file.write(serde_json::to_string_pretty(&object)?.as_bytes())?;

	Ok(())
}
