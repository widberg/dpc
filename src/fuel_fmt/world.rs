use std::io::Result;
use std::io::Write;
use std::path::Path;

use nom::number::complete::*;
use nom_derive::{NomLE, Parse};
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::{Mat4f, ResourceObjectZ};
use crate::File;

#[derive(Serialize, Deserialize, NomLE)]
struct WorldZUnknown2 {
    unknown0: u32,
    unknown1: u32,
    unknown2: u32,
    unknown3: u32,
    unknown4: u32,
    unknown5: u32,
}

#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
struct WorldZ {
    unknown11: u32,
    unknown12: u32,
    unknown13: u32,
    unknown14: u32,
    unknown15: u32,
    unknown16: u32,
    #[nom(LengthCount(le_u32))]
    unknown17s: Vec<u32>,
    #[nom(LengthCount(le_u32))]
    unknowns: Vec<u8>,
    unknown0: Mat4f,
    #[nom(LengthCount(le_u32))]
    unknown1s: Vec<u32>,
    #[nom(LengthCount(le_u32))]
    unknown2s: Vec<WorldZUnknown2>,
    unknown3: Mat4f,
    #[nom(LengthCount(le_u32))]
    unknown4s: Vec<u32>,
    #[nom(LengthCount(le_u32))]
    unknown5s: Vec<WorldZUnknown2>,
    #[nom(LengthCount(le_u32))]
    unknown6s: Vec<u32>,
    #[nom(LengthCount(le_u32))]
    unknown7s: Vec<u32>,
    #[nom(LengthCount(le_u32))]
    unknown8s: Vec<u32>,
    #[nom(LengthCount(le_u32))]
    unknown9s: Vec<u32>,
    #[nom(LengthCount(le_u32))]
    unknown10s: Vec<u32>,
    #[nom(LengthCount(le_u32))]
    unknown11s: Vec<u32>,
    #[nom(LengthCount(le_u32))]
    unknown12s: Vec<u32>,
    #[nom(LengthCount(le_u32))]
    unknown13s: Vec<u32>,
}

#[derive(Serialize, Deserialize)]
struct WorldObject {
    resource_object: ResourceObjectZ,
    world: WorldZ,
}

pub fn fuel_fmt_extract_world_z(header: &[u8], data: &[u8], output_path: &Path) -> Result<()> {
    let json_path = output_path.join("object.json");
    let mut output_file = File::create(json_path)?;

    let resource_object = match ResourceObjectZ::parse(&header) {
        Ok((_, h)) => h,
        Err(error) => panic!("{}", error),
    };

    let world = match WorldZ::parse(&data) {
        Ok((_, h)) => h,
        Err(error) => panic!("{}", error),
    };

    let object = WorldObject {
        resource_object,
        world,
    };

    output_file.write(serde_json::to_string_pretty(&object)?.as_bytes())?;

    Ok(())
}
