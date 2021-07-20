use std::io::Result;
use std::io::Write;
use std::path::Path;

use nom_derive::{NomLE, Parse};
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::{Mat4f, ObjectZ, PascalArray};
use crate::File;

#[derive(Serialize, Deserialize, NomLE)]
struct WorldRefZUnknown7 {
    unknown0: u32,
    unknown1: u32,
}

#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
struct WorldRefZ {
    unknown11: u32,
    unknown12: u32,
    unknown13: u32,
    unknown14: u32,
    unknown15: u32,
    unknown16: u32,
    unknown17s: PascalArray<u32>,
    unknowns: PascalArray<u8>,
    mats: PascalArray<Mat4f>,
    unknown1: u32,
    unknown2: u32,
    unknown3: u32,
    unknown4: u32,
    unknown5: u32,
    unknown6: u32,
    unknown7s: PascalArray<WorldRefZUnknown7>,
    unknown8s: PascalArray<u8>,
    unknown9s: PascalArray<u32>,
    zero: u32,
}

#[derive(Serialize, Deserialize)]
struct WorldRefObject {
    object: ObjectZ,
    world_ref: WorldRefZ,
}

pub fn fuel_fmt_extract_world_ref_z(header: &[u8], data: &[u8], output_path: &Path) -> Result<()> {
    let json_path = output_path.join("object.json");
    let mut output_file = File::create(json_path)?;

    let object = match ObjectZ::parse(&header) {
        Ok((_, h)) => h,
        Err(error) => panic!("{}", error),
    };

    let world_ref = match WorldRefZ::parse(&data) {
        Ok((_, h)) => h,
        Err(error) => panic!("{}", error),
    };

    let object = WorldRefObject { object, world_ref };

    output_file.write(serde_json::to_string_pretty(&object)?.as_bytes())?;

    Ok(())
}
