use std::io::Result;
use std::io::Write;
use std::path::Path;

use nom_derive::{NomLE, Parse};
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::{ObjectZ, PascalArray, Vec2f, Vec3f};
use crate::File;

#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
struct RotShapeZ {
    vertices: PascalArray<Vec3f>,
    unknown1: f32,
    ints: PascalArray<u32>,
    sizes: PascalArray<Vec3f>,
    texcoords: PascalArray<Vec2f>,
    material_crc32s: PascalArray<u32>,
    scale: f32,
    billboard_mode: u16,
}

#[derive(Serialize, Deserialize)]
struct RotShapeObject {
    object: ObjectZ,
    rot_shape: RotShapeZ,
}

pub fn fuel_fmt_extract_rot_shape_z(header: &[u8], data: &[u8], output_path: &Path) -> Result<()> {
    let json_path = output_path.join("object.json");
    let mut output_file = File::create(json_path)?;

    let object = match ObjectZ::parse(&header) {
        Ok((_, h)) => h,
        Err(error) => panic!("{}", error),
    };

    let rot_shape = match RotShapeZ::parse(&data) {
        Ok((_, h)) => h,
        Err(error) => panic!("{}", error),
    };

    let object = RotShapeObject { object, rot_shape };

    output_file.write(serde_json::to_string_pretty(&object)?.as_bytes())?;

    Ok(())
}
