use nom_derive::{NomLE, Parse};
use serde::{Serialize, Deserialize};
use std::path::Path;
use crate::File;
use std::io::Result;
use std::io::Write;

use crate::fuel_fmt::common::{ObjectZ, Mat4f};

#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
struct CollisionVolZ {
    unknown0: u32,
    local_transform: Mat4f,
    local_transform_inverse: Mat4f,
    #[nom(Count(28))]
    zeros: Vec<u32>,
    volume_type: u32,
    unknown1: u32,
}

#[derive(Serialize, Deserialize)]
struct CollisionVolObject {
    object: ObjectZ,
    collision_vol: CollisionVolZ,
}

pub fn fuel_fmt_extract_collision_vol_z(header: &[u8], data: &[u8], output_path: &Path) -> Result<()> {
    let json_path = output_path.join("object.json");
    let mut output_file = File::create(json_path)?;

    let object = match ObjectZ::parse(&header) {
        Ok((_, h)) => h,
        Err(error) => panic!("{}", error),
    };

    let collision_vol = match CollisionVolZ::parse(&data) {
        Ok((_, h)) => h,
        Err(error) => panic!("{}", error),
    };

    let object = CollisionVolObject {
        object,
        collision_vol,
    };

    output_file.write(serde_json::to_string_pretty(&object)?.as_bytes())?;

    Ok(())
}