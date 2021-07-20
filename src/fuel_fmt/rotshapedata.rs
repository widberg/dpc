use std::io::Result;
use std::io::Write;
use std::path::Path;

use nom_derive::{NomLE, Parse};
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::{PascalArray, ResourceObjectZ};
use crate::File;

#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
struct RotShapeDataZ {
    one: u32,
    shorts: PascalArray<u16>,
    #[nom(Map = "|x: &[u8]| x.to_vec()", Take = "shorts.len() * 28")]
    padding: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
struct RotShapeDataObject {
    resource_object: ResourceObjectZ,
    rot_shape_data: RotShapeDataZ,
}

pub fn fuel_fmt_extract_rot_shape_data_z(
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
