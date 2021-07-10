use nom_derive::{NomLE, Parse};
use serde::{Serialize, Deserialize};
use nom::number::complete::*;
use nom::*;
use std::path::Path;
use crate::File;
use std::io::Result;
use std::io::Write;

use crate::fuel_fmt::common::ResourceObjectZ;


#[derive(Serialize, Deserialize, NomLE)]
struct MaterialObjZEntry {
    array_name_crc32: u32,
    #[nom(LengthCount = "le_u32")]
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