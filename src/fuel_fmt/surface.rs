use std::io::Result;
use std::io::Write;
use std::path::Path;

use nom_derive::{NomLE, Parse};
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::{FixedVec, ObjectZ, PascalArray, Quat, Vec3f};
use crate::File;

#[derive(Serialize, Deserialize, NomLE)]
struct SurfaceZUnknown2 {
    data: FixedVec<u8, 32>,
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
    data: FixedVec<u32, 43>,
    unknown: u32,
}

#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
struct SurfaceZ {
    vertices: PascalArray<Vec3f>,
    unknown1s: PascalArray<Quat>,
    unknown2s: PascalArray<SurfaceZUnknown2>,
    unknown3s: PascalArray<SurfaceZUnknown2>,
    unknown4s: PascalArray<SurfaceZUnknown4>,
    unknown7s: PascalArray<SurfaceZUnknown7>,
    unknown8s: PascalArray<SurfaceZUnknown8>,
    unknown9s: PascalArray<SurfaceZUnknown8>,
    unknown10s: PascalArray<SurfaceZUnknown7>,
    unknown11s: PascalArray<u16>,
    unknown12s: PascalArray<SurfaceZUnknown2>,
    #[serde(skip_serializing)]
    #[allow(dead_code)]
    opt: u8,
    #[nom(Cond = "opt != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    unknown13s: Option<PascalArray<u32>>,
    #[nom(Cond = "opt != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    unknown14s: Option<PascalArray<u16>>,
    #[nom(Cond = "opt != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    unknown15: Option<FixedVec<u32, 52>>,
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

    let object = SurfaceObject { object, surface };

    output_file.write(serde_json::to_string_pretty(&object)?.as_bytes())?;

    Ok(())
}
