use nom_derive::{NomLE, Parse};
use serde::{Serialize, Deserialize};
use std::path::Path;
use crate::File;
use std::io::Result;
use std::io::Write;
use nom::number::complete::*;

use crate::fuel_fmt::common::{ResourceObjectZ, Mat4f};

#[derive(Serialize, Deserialize, NomLE)]
struct SkelZBone {
    unknown0: u32,
    #[nom(Count(136))]
    data0: Vec<u8>,
    transformation: Mat4f,
    unknown1: u32,
    parent_index: u32,
    #[nom(Count(16))]
    data1: Vec<u8>,
    unknown2: u32,
    unknown3: u32,
    unknown4: u32,
    unknown5: u32,
    unknown6: u32,
}

#[derive(Serialize, Deserialize, NomLE)]
struct SkelZUnknown4 {
    unknown0: u32,
    unknown1: u32,
    unknown2: u32,
    unknown3: u32,
    unknown4: u32,
    unknown5: u32,
    unknown6: u32,
}

#[derive(Serialize, Deserialize, NomLE)]
struct SkelZUnknown2 {
    mat: Mat4f,
    unknown0: u32,
    unknown1: u32,
    unknown2: u32,
}

#[derive(Serialize, Deserialize, NomLE)]
struct SkelZUnknown5 {
    #[nom(LengthCount(le_u32))]
    data: Vec<u32>,
}

#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
struct SkelZ {
    u0: u32,
    u1: f32,
    u2: f32,
    u3: f32,
    u4: f32,
    #[nom(LengthCount(le_u32))]
    bones: Vec<SkelZBone>,
    #[nom(LengthCount(le_u32))]
    material_crc32s: Vec<u32>,
    #[nom(LengthCount(le_u32))]
    mesh_data_crc32s: Vec<u32>,
    #[nom(LengthCount(le_u32))]
    unknown5s: Vec<SkelZUnknown5>,
    #[nom(LengthCount(le_u32))]
    unknown3: Vec<u32>,
    #[nom(LengthCount(le_u32))]
    unknown4s: Vec<SkelZUnknown4>,
    #[nom(LengthCount(le_u32))]
    unknown1s: Vec<SkelZUnknown4>,
    #[nom(LengthCount(le_u32))]
    unknown2s: Vec<SkelZUnknown2>,
}

#[derive(Serialize, Deserialize)]
struct SkelObject {
    resource_object: ResourceObjectZ,
    skel: SkelZ,
}

pub fn fuel_fmt_extract_skel_z(header: &[u8], data: &[u8], output_path: &Path) -> Result<()> {
    let json_path = output_path.join("object.json");
    let mut output_file = File::create(json_path)?;

    let resource_object = match ResourceObjectZ::parse(&header) {
        Ok((_, h)) => h,
        Err(error) => panic!("{}", error),
    };

    let skel = match SkelZ::parse(&data) {
        Ok((_, h)) => h,
        Err(error) => panic!("{}", error),
    };

    let object = SkelObject {
        resource_object,
        skel,
    };

    output_file.write(serde_json::to_string_pretty(&object)?.as_bytes())?;

    Ok(())
}
