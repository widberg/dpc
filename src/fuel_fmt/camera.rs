use std::io::Result;
use std::io::Write;
use std::path::Path;

use nom_derive::{NomLE, Parse};
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::ObjectZ;
use crate::File;

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

    let object = CameraObject { object, camera };

    output_file.write(serde_json::to_string_pretty(&object)?.as_bytes())?;

    Ok(())
}
