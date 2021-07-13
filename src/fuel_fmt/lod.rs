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
struct LodZUnknown4 {
    a: u32,
    b: u32,
}

#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
struct LodZ {
    #[nom(LengthCount(le_u32))]
    unknown0s: Vec<LodZUnknown0>,
    #[nom(LengthCount(le_u32))]
    unknown1s: Vec<LodZUnknown1>,
    unknown2: u32,
    unknown3: u32,
    u0: f32,
    #[nom(LengthCount(le_u32))]
    skin_crc32s: Vec<u32>,
    u1: u32,
    #[serde(skip_serializing)]
    #[allow(dead_code)]
    sound_entries_option: u32,
    #[nom(Cond(sound_entries_option != 0))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[nom(LengthCount(le_u32))]
    sound_entries: Option<Vec<LodZSoundEntry>>,
    #[serde(skip_serializing)]
    #[allow(dead_code)]
    unknown4_option: u32,
    #[nom(Cond(unknown4_option != 0))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[nom(LengthCount(le_u32))]
    unknown4s: Option<Vec<LodZUnknown4>>,
    unknown5: u32,
}

#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
struct LodZAlt {
    #[nom(Count(i.len()))]
    data: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
struct LodObject {
    object: ObjectZ,
    lod: LodZ,
}

#[derive(Serialize, Deserialize)]
struct LodObjectAlt {
    object: ObjectZ,
    lod: LodZAlt,
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
        Err(_) => match LodZAlt::parse(&data) {
            Ok((_, lod)) => {
                let object = LodObjectAlt {
                    object,
                    lod,
                };

                output_file.write(serde_json::to_string_pretty(&object)?.as_bytes())?;

                return Ok(());
            },
            Err(error) => panic!("{}", error),
        },
    };

    let object = LodObject {
        object,
        lod,
    };

    output_file.write(serde_json::to_string_pretty(&object)?.as_bytes())?;

    Ok(())
}
