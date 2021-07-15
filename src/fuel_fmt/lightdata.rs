use std::io::Result;
use std::io::Write;
use std::path::Path;

use nom_derive::{NomLE, Parse};
use serde::{Deserialize, Serialize};

use crate::File;
use crate::fuel_fmt::common::{ResourceObjectZ, Vec3f, Vec3i32};

#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
struct LightDataZ {
    unknown0: u32,
    color: Vec3f,
    unknown1: Vec3f,
    unknown2: Vec3i32,
    unknown_flag: u32,
    unknown3: Vec3f,
}

#[derive(Serialize, Deserialize)]
struct LightDataObject {
    resource_object: ResourceObjectZ,
    light_data: LightDataZ,
}

pub fn fuel_fmt_extract_light_data_z(header: &[u8], data: &[u8], output_path: &Path) -> Result<()> {
    let json_path = output_path.join("object.json");
    let mut output_file = File::create(json_path)?;

    let resource_object = match ResourceObjectZ::parse(&header) {
        Ok((_, h)) => h,
        Err(error) => panic!("{}", error),
    };

    let light_data = match LightDataZ::parse(&data) {
        Ok((_, h)) => h,
        Err(error) => panic!("{}", error),
    };

    let object = LightDataObject {
        resource_object,
        light_data,
    };

    output_file.write(serde_json::to_string_pretty(&object)?.as_bytes())?;

    Ok(())
}
