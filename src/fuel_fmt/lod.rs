use nom_derive::{NomLE, Parse};
use serde::{Serialize, Deserialize};
use std::path::Path;
use crate::File;
use std::io::Result;
use std::io::Write;
use nom::number::complete::*;

use crate::fuel_fmt::common::{ObjectZ, Mat4f};

#[derive(Serialize, Deserialize, NomLE)]
struct LodZUnknown0 {
    a: f32,
    b: f32,
    c: f32,
    d: f32,
    e: u32,
    f: f32,
}

#[derive(Serialize, Deserialize, NomLE)]
struct LodZUnknown1 {
    transformation: Mat4f,
    q: u32,
    r: f32,
}

#[derive(Serialize, Deserialize, NomLE)]
struct LodZSoundEntry
{
    id: u32,
    sound_crc32: u32,
}

#[derive(Serialize, Deserialize, NomLE)]
struct LodZUnknown2 {
    #[nom(LengthCount(le_u32))]
    sound_entries: Vec<LodZSoundEntry>,
}

#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
struct LodZ {
    #[nom(LengthCount(le_u32))]
    unknown0s: Vec<LodZUnknown0>,
    #[nom(LengthCount(le_u32))]
    unknown1s: Vec<LodZUnknown1>,
    unknown2_count: u32,
    unknown3_count: u32,
    u0: f32,
    #[nom(LengthCount(le_u32))]
    skin_crc32s: Vec<u32>,
    u1: u32,
    u3: u32,
    #[nom(LengthCount(le_u32))]
    unknown2s: Vec<LodZUnknown2>,
    zero: u32,
}

#[derive(Serialize, Deserialize)]
struct LodObject {
    object: ObjectZ,
    lod: LodZ,
}

pub fn fuel_fmt_extract_lod_z(header: &[u8], data: &[u8], output_path: &Path) -> Result<()> {
    let json_path = output_path.join("object.json");
    let mut output_file = File::create(json_path)?;

    let object = match ObjectZ::parse(&header) {
        Ok((_, h)) => h,
        Err(error) => panic!("{}", error),
    };

    let lod = match LodZ::parse(&data) {
        Ok((_, h)) => h,
        Err(error) => panic!("{}", error),
    };

    let object = LodObject {
        object,
        lod,
    };

    output_file.write(serde_json::to_string_pretty(&object)?.as_bytes())?;

    Ok(())
}
