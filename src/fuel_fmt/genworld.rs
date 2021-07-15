use nom_derive::{NomLE, Parse};
use serde::{Serialize, Deserialize};
use std::path::Path;
use crate::File;
use std::io::Result;
use std::io::Write;
use nom::number::complete::*;

use crate::fuel_fmt::common::{ObjectZ, Mat4f};

#[derive(Serialize, Deserialize, NomLE)]
struct GenWorldZUnknown7Unknown {
    #[nom(LengthCount(le_u32))]
    unknown0s: Vec<u32>,
}

#[derive(Serialize, Deserialize, NomLE)]
struct GenWorldZUnknown7 {
    #[nom(LengthCount(le_u32))]
    unknown0s: Vec<u8>,
    #[nom(LengthCount(le_u32))]
    unknown1s: Vec<GenWorldZUnknown7Unknown>,
}

#[derive(Serialize, Deserialize, NomLE)]
struct GenWorldZUnknown8 {
    unknown0: u32,
    #[nom(Count(127))]
    data: Vec<u8>,
}

#[derive(Serialize, Deserialize, NomLE)]
struct GenWorldZUnknown10 {
    unknown0: u32,
    #[nom(Count(8))]
    unknown1s: Vec<u32>,
    unknown2: u32,
    unknown3: u32,
    unknown4: u32,
}

#[derive(Serialize, Deserialize, NomLE)]
struct GenWorldZUnknown11 {
    unknown0: u32,
    unknown1: u32,
}

#[derive(Serialize, Deserialize, NomLE)]
struct GenWorldZUnknown12
{
    unknown0: u32,
    unknown1: u32,
}

#[derive(Serialize, Deserialize, NomLE)]
struct GenWorldZUnknown13
{
    #[nom(Count(8))]
    unknown0s: Vec<u32>,
    #[nom(LengthCount(le_u32))]
    unknown1s: Vec<u32>,
}

#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
struct GenWorldZ {
    unknown0: u32,
    unknown1: u32,
    unknown2: u32,
    #[nom(LengthCount(le_u32))]
    unknown3s: Vec<u32>,
    #[nom(LengthCount(le_u32))]
    unknown4s: Vec<u32>,
    #[nom(LengthCount(le_u32))]
    unknown5s: Vec<u32>,
    unknown6: u32,
    #[nom(LengthCount(le_u32))]
    unknown7s: Vec<GenWorldZUnknown7>,
    #[nom(LengthCount(le_u32))]
    unknown8s: Vec<GenWorldZUnknown8>,
    #[nom(LengthCount(le_u32))]
    mats: Vec<Mat4f>,
    #[nom(LengthCount(le_u32))]
    unknown10s: Vec<GenWorldZUnknown10>,
    #[nom(LengthCount(le_u32))]
    unknown11s: Vec<GenWorldZUnknown11>,
    #[nom(LengthCount(le_u32))]
    unknown12s: Vec<GenWorldZUnknown12>,
    #[nom(LengthCount(le_u32))]
    unknown13s: Vec<GenWorldZUnknown13>,
}

#[derive(Serialize, Deserialize)]
struct GenWorldObject {
    object: ObjectZ,
    gen_world: GenWorldZ,
}

pub fn fuel_fmt_extract_gen_world_z(header: &[u8], data: &[u8], output_path: &Path) -> Result<()> {
    let json_path = output_path.join("object.json");
    let mut output_file = File::create(json_path)?;

    let object = match ObjectZ::parse(&header) {
        Ok((_, h)) => h,
        Err(error) => panic!("{}", error),
    };

    let gen_world = match GenWorldZ::parse(&data) {
        Ok((_, h)) => h,
        Err(error) => panic!("{}", error),
    };

    let object = GenWorldObject {
        object,
        gen_world,
    };

    output_file.write(serde_json::to_string_pretty(&object)?.as_bytes())?;

    Ok(())
}
