use std::io::Result;
use std::io::Write;
use std::path::Path;

use nom_derive::{NomLE, Parse};
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::{PascalArray, ResourceObjectZ};
use crate::File;

#[derive(Serialize, Deserialize, NomLE)]
struct MaterialObjZEntry {
    array_name_crc32: u32,
    material_anim_crc32s: PascalArray<u32>,
}

#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
struct MaterialObjZ {
    entries: PascalArray<MaterialObjZEntry>,
}

#[derive(Serialize, Deserialize)]
struct MaterialObjObject {
    resource_object: ResourceObjectZ,
    material_obj: MaterialObjZ,
}

pub fn fuel_fmt_extract_material_obj_z(
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
