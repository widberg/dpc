use std::io::Result;
use std::io::Write;
use std::path::Path;

use nom::number::complete::*;
use nom_derive::{NomLE, Parse};
use serde::{Deserialize, Serialize};

use crate::File;
use crate::fuel_fmt::common::{ObjectZ, Quat, Vec3f};

#[derive(Serialize, Deserialize, NomLE)]
struct SurfaceZUnknown2 {
    #[nom(Count(32))]
    data: Vec<u8>,
}

#[derive(Serialize, Deserialize, NomLE)]
struct SurfaceZUnknown7 {
    unknown0: u32,
    unknown1: u32,
}

#[derive(Serialize, Deserialize, NomLE)]
struct SurfaceZUnknown8 {
    unknown0: u32,
    unknown1: u32,
    unknown2: u32,
}

#[derive(Serialize, Deserialize, NomLE)]
struct SurfaceZUnknown4 {
    #[nom(Count(43))]
    data: Vec<u32>,
    unknown: u32,
}

#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
struct SurfaceZ {
    #[nom(LengthCount(le_u32))]
    vertices: Vec<Vec3f>,
    #[nom(LengthCount(le_u32))]
    unknown1s: Vec<Quat>,
    #[nom(LengthCount(le_u32))]
    unknown2s: Vec<SurfaceZUnknown2>,
    #[nom(LengthCount(le_u32))]
    unknown3s: Vec<SurfaceZUnknown2>,
    #[nom(LengthCount(le_u32))]
    unknown4s: Vec<SurfaceZUnknown4>,
    #[nom(LengthCount(le_u32))]
    unknown7s: Vec<SurfaceZUnknown7>,
    #[nom(LengthCount(le_u32))]
    unknown8s: Vec<SurfaceZUnknown8>,
    #[nom(LengthCount(le_u32))]
    unknown9s: Vec<SurfaceZUnknown8>,
    #[nom(LengthCount(le_u32))]
    unknown10s: Vec<SurfaceZUnknown7>,
    #[nom(LengthCount(le_u32))]
    unknown11s: Vec<u16>,
    #[nom(LengthCount(le_u32))]
    unknown12s: Vec<SurfaceZUnknown2>,
    #[serde(skip_serializing)]
    #[allow(dead_code)]
    opt: u8,
    #[nom(Cond = "opt != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[nom(LengthCount(le_u32))]
    unknown13s: Option<Vec<u32>>,
    #[nom(Cond = "opt != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[nom(LengthCount(le_u32))]
    unknown14s: Option<Vec<u16>>,
    #[nom(Cond = "opt != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[nom(Count(52))]
    unknown15: Option<Vec<u32>>,
    #[nom(Cond = "opt != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    unknown16_count: Option<u32>,
}

#[derive(Serialize, Deserialize)]
struct SurfaceObject {
    object: ObjectZ,
    surface: SurfaceZ,
}

pub fn fuel_fmt_extract_surface_z(header: &[u8], data: &[u8], output_path: &Path) -> Result<()> {
    let json_path = output_path.join("object.json");
    let mut output_file = File::create(json_path)?;

    let object = match ObjectZ::parse(&header) {
        Ok((_, h)) => h,
        Err(error) => panic!("{}", error),
    };

    let surface = match SurfaceZ::parse(&data) {
        Ok((_, h)) => h,
        Err(error) => panic!("{}", error),
    };

    let object = SurfaceObject {
        object,
        surface,
    };

    output_file.write(serde_json::to_string_pretty(&object)?.as_bytes())?;

    Ok(())
}
