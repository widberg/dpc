use std::io::Result;
use std::io::Write;
use std::path::Path;

use nom::number::complete::*;
use nom_derive::{NomLE, Parse};
use serde::{Deserialize, Serialize};

use crate::File;
use crate::fuel_fmt::common::ResourceObjectZ;

#[derive(Serialize, Deserialize, NomLE)]
struct RtcZUnknown1Unknown2 {
    unknown0: u32,
    unknown1: u32,
    unknown2: u32,
}

#[derive(Serialize, Deserialize, NomLE)]
struct RtcZUnknown1Unknown3Unknown
{
    unknown0: u32,
    unknown1: u32,
}

#[derive(Serialize, Deserialize, NomLE)]
struct RtcZUnknown1Unknown3 {
    #[nom(Count(5))]
    unknowns: Vec<RtcZUnknown1Unknown3Unknown>,
}

#[derive(Serialize, Deserialize, NomLE)]
struct RtcZUnknown1Unknown5Unknown1
{
    unknown0: u32,
    unknown1: u32,
    unknown2: u32,
    unknown3: u32,
    unknown4: u32,
}

#[derive(Serialize, Deserialize, NomLE)]
struct RtcZUnknown1Unknown5 {
    unknown0: u32,
    #[nom(LengthCount(le_u32))]
    unknown1s: Vec<RtcZUnknown1Unknown5Unknown1>,
}

#[derive(Serialize, Deserialize, NomLE)]
struct RtcZUnknown1 {
    unknown_node_crc32: u32,
    unknown1: u16,
    #[nom(LengthCount(le_u32))]
    unknown2s: Vec<RtcZUnknown1Unknown2>,
    unknown3flag: u16,
    #[nom(LengthCount(le_u32))]
    unknown3s: Vec<RtcZUnknown1Unknown3>,
    unknown4flag: u16,
    #[nom(LengthCount(le_u32))]
    unknown4s: Vec<RtcZUnknown1Unknown3>,
    #[nom(LengthCount(le_u32))]
    unknown5s: Vec<RtcZUnknown1Unknown5>,
}

#[derive(Serialize, Deserialize, NomLE)]
struct RtcZUnknown2Unknown2 {
    unknown0: u32,
    unknown1: u32,
    unknown2: u32,
}

#[derive(Serialize, Deserialize, NomLE)]
struct RtcZUnknown2Unknown4 {
    unknown0: u32,
    unknown1: u32,
    unknown2: u32,
    unknown3: u32,
}

#[derive(Serialize, Deserialize, NomLE)]
struct RtcZUnknown2 {
    unknown0: u32,
    unknown1: u16,
    unknown2flag: u16,
    #[nom(LengthCount(le_u32))]
    unknown2s: Vec<RtcZUnknown2Unknown2>,
    unknown3flag: u16,
    #[nom(LengthCount(le_u32))]
    unknown3s: Vec<RtcZUnknown2Unknown2>,
    unknown4flag: u16,
    #[nom(LengthCount(le_u32))]
    unknown4s: Vec<RtcZUnknown2Unknown4>,
    unknown5flag: u16,
    #[nom(LengthCount(le_u32))]
    unknown5s: Vec<RtcZUnknown2Unknown2>,
}

#[derive(Serialize, Deserialize, NomLE)]
struct RtcZUnknown4RtcZUnknown5Unknown {
    unknown0: u32,
    unknown1: u32,
}

#[derive(Serialize, Deserialize, NomLE)]
struct RtcZUnknown4RtcZUnknown5 {
    #[nom(Count(3))]
    unknowns: Vec<RtcZUnknown4RtcZUnknown5Unknown>,
}

#[derive(Serialize, Deserialize, NomLE)]
struct RtcZUnknown4RtcZUnknown6 {
    unknown0: u32,
    unknown1: u32,
    unknown2: u32,
}

#[derive(Serialize, Deserialize, NomLE)]
struct RtcZUnknown4 {
    unknown0: u32,
    unknown1: u16,
    unknown5flag: u16,
    #[nom(LengthCount(le_u32))]
    unknown5s: Vec<RtcZUnknown4RtcZUnknown5>,
    unknown6flag: u16,
    #[nom(LengthCount(le_u32))]
    unknown6s: Vec<RtcZUnknown4RtcZUnknown6>,
    unknown7flag: u16,
    #[nom(LengthCount(le_u32))]
    unknown7s: Vec<RtcZUnknown4RtcZUnknown6>,
}

#[derive(Serialize, Deserialize, NomLE)]
struct RtcZUnknown8 {
    unknown0: u32,
    unknown1: u32,
    unknown2: u32,
    unknown3: u32,
    unknown4: u8,
    unknown5: u32,
    unknown6: u32,
}

#[derive(Serialize, Deserialize, NomLE)]
struct RtcZUnknown9 {
    unknown0: u32,
    unknown1: u32,
    unknown2: u32,
    unknown3: u32,
    unknown4: u32,
    unknown5: u32,
}

#[derive(Serialize, Deserialize, NomLE)]
struct RtcZUnknown12Unknown1 {
    unknown0: u32,
    unknown1: u32,
    unknown2: u32,
    unknown3: u32,
    unknown4: u32,
}

#[derive(Serialize, Deserialize, NomLE)]
struct RtcZUnknown12
{
    unknown0: u32,
    #[nom(LengthCount(le_u32))]
    unknown1s: Vec<RtcZUnknown12Unknown1>,
}

#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
struct RtcZ {
    unknown0: f32,
    #[nom(LengthCount(le_u32))]
    unknown1s: Vec<RtcZUnknown1>,
    #[nom(LengthCount(le_u32))]
    unknown2s: Vec<RtcZUnknown2>,
    #[nom(LengthCount(le_u32))]
    unknown3s: Vec<u32>,
    #[nom(LengthCount(le_u32))]
    unknown4s: Vec<RtcZUnknown4>,
    #[nom(LengthCount(le_u32))]
    unknown8s: Vec<RtcZUnknown8>,
    #[nom(LengthCount(le_u32))]
    unknown9s: Vec<RtcZUnknown9>,
    #[nom(LengthCount(le_u32))]
    unknown10s: Vec<u32>,
    #[nom(LengthCount(le_u32))]
    unknown11s: Vec<u32>,
    #[nom(LengthCount(le_u32))]
    unknown12s: Vec<RtcZUnknown12>,
}

#[derive(Serialize, Deserialize)]
struct RtcObject {
    resource_object: ResourceObjectZ,
    rtc: RtcZ,
}

pub fn fuel_fmt_extract_rtc_z(header: &[u8], data: &[u8], output_path: &Path) -> Result<()> {
    let json_path = output_path.join("object.json");
    let mut output_file = File::create(json_path)?;

    let resource_object = match ResourceObjectZ::parse(&header) {
        Ok((_, h)) => h,
        Err(error) => panic!("{}", error),
    };

    let rtc = match RtcZ::parse(&data) {
        Ok((_, h)) => h,
        Err(error) => panic!("{}", error),
    };

    let object = RtcObject {
        resource_object,
        rtc,
    };

    output_file.write(serde_json::to_string_pretty(&object)?.as_bytes())?;

    Ok(())
}
