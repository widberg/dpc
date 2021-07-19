use std::io::Result;
use std::io::Write;
use std::path::Path;

use nom::number::complete::*;
use nom_derive::{NomLE, Parse};
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::{ObjectZ, Vec2f, Vec3f};
use crate::File;

#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
struct RotShapeZ {
    #[nom(LengthCount(le_u32))]
    vertices: Vec<Vec3f>,
    unknown1: f32,
    #[nom(LengthCount(le_u32))]
    ints: Vec<u32>,
    #[nom(LengthCount(le_u32))]
    sizes: Vec<Vec3f>,
    #[nom(LengthCount(le_u32))]
    texcoords: Vec<Vec2f>,
    #[nom(LengthCount(le_u32))]
    material_crc32s: Vec<u32>,
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
