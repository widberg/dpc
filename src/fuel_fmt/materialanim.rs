use std::io::Result;
use std::io::Write;
use std::path::Path;

use nom_derive::{NomLE, Parse};
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::{PascalArray, ResourceObjectZ};
use crate::File;

#[derive(Serialize, Deserialize, NomLE)]
struct MaterialAnimZUnknown0 {
    unknown0: f32,
    unknown1: f32,
}

#[derive(Serialize, Deserialize, NomLE)]
struct MaterialAnimZUnknown23 {
    unknown0: f32,
    unknown1: f32,
    unknown2: f32,
}

#[derive(Serialize, Deserialize, NomLE)]
struct MaterialAnimZUnknown56 {
    unknown0: f32,
    unknown1: f32,
    unknown2: f32,
    unknown3: f32,
}

#[derive(Serialize, Deserialize, NomLE)]
struct MaterialAnimZUnknown89 {
    unknown0: f32,
    unknown1: f32,
    unknown2: f32,
    unknown3: f32,
    unknown4: f32,
}

#[derive(Serialize, Deserialize, NomLE)]
struct MaterialAnimZUnknown1011 {
    unknown0: f32,
    unknown1: f32,
}

#[derive(Serialize, Deserialize, NomLE)]
struct MaterialAnimZColor {
    unknown: f32,
    rgba: u32,
}

#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
struct MaterialAnimZ {
    unknown0s: PascalArray<MaterialAnimZUnknown0>,
    unknown2flag: u16,
    unknown2s: PascalArray<MaterialAnimZUnknown23>,
    unknown3flag: u16,
    unknown3s: PascalArray<MaterialAnimZUnknown23>,
    unknown4flag: u16,
    unknown4s: PascalArray<MaterialAnimZColor>,
    unknown5flag: u16,
    unknown5s: PascalArray<MaterialAnimZUnknown56>,
    unknown6flag: u16,
    unknown6s: PascalArray<MaterialAnimZUnknown56>,
    colorsflag: u16,
    colors: PascalArray<MaterialAnimZColor>,
    unknown8flag: u16,
    unknown8s: PascalArray<MaterialAnimZUnknown89>,
    unknown9flag: u16,
    unknown9s: PascalArray<MaterialAnimZUnknown89>,
    unknown10s: PascalArray<MaterialAnimZUnknown1011>,
    unknown11s: PascalArray<MaterialAnimZUnknown1011>,
    material_crc32: u32,
    unknown_float: f32,
    unknown15: u8,
}

#[derive(Serialize, Deserialize)]
struct MaterialAnimObject {
    resource_object: ResourceObjectZ,
    material_anim: MaterialAnimZ,
}

pub fn fuel_fmt_extract_material_anim_z(
    header: &[u8],
    data: &[u8],
    output_path: &Path,
) -> Result<()> {
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
