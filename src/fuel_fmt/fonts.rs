use std::io::Result;
use std::io::Write;
use std::path::Path;

use nom_derive::{NomLE, Parse};
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::{PascalArray, ResourceObjectZ};
use crate::File;

#[derive(Serialize, Deserialize, NomLE)]
struct FontsZCharacter {
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
struct FontsZ {
    characters: PascalArray<FontsZCharacter>,
    material_crc32s: PascalArray<u32>,
}

#[derive(Serialize, Deserialize)]
struct FontsObject {
    resource_object: ResourceObjectZ,
    fonts: FontsZ,
}

pub fn fuel_fmt_extract_fonts_z(header: &[u8], data: &[u8], output_path: &Path) -> Result<()> {
    let json_path = output_path.join("object.json");
    let mut output_file = File::create(json_path)?;

    let resource_object = match ResourceObjectZ::parse(&header) {
        Ok((_, h)) => h,
        Err(error) => panic!("{}", error),
    };

    let fonts = match FontsZ::parse(&data) {
        Ok((_, h)) => h,
        Err(error) => panic!("{}", error),
    };

    let object = FontsObject {
        resource_object,
        fonts,
    };

    output_file.write(serde_json::to_string_pretty(&object)?.as_bytes())?;

    Ok(())
}
