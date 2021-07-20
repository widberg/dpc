use std::io::Result;
use std::io::Write;
use std::path::Path;

use nom_derive::{NomLE, Parse};
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::{FixedVec, PascalArray, ResourceObjectZ};
use crate::File;

#[derive(Serialize, Deserialize, NomLE)]
struct RtcZUnknown1Unknown2 {
    unknown0: u32,
    unknown1: u32,
    unknown2: u32,
}

#[derive(Serialize, Deserialize, NomLE)]
struct RtcZUnknown1Unknown3Unknown {
    unknown0: u32,
    unknown1: u32,
}

#[derive(Serialize, Deserialize, NomLE)]
struct RtcZUnknown1Unknown3 {
    unknowns: FixedVec<RtcZUnknown1Unknown3Unknown, 5>,
}

#[derive(Serialize, Deserialize, NomLE)]
struct RtcZUnknown1Unknown5Unknown1 {
    unknown0: u32,
    unknown1: u32,
    unknown2: u32,
    unknown3: u32,
    unknown4: u32,
}

#[derive(Serialize, Deserialize, NomLE)]
struct RtcZUnknown1Unknown5 {
    unknown0: u32,
    unknown1s: PascalArray<RtcZUnknown1Unknown5Unknown1>,
}

#[derive(Serialize, Deserialize, NomLE)]
struct RtcZUnknown1 {
    unknown_node_crc32: u32,
    unknown1: u16,
    unknown2s: PascalArray<RtcZUnknown1Unknown2>,
    unknown3flag: u16,
    unknown3s: PascalArray<RtcZUnknown1Unknown3>,
    unknown4flag: u16,
    unknown4s: PascalArray<RtcZUnknown1Unknown3>,
    unknown5s: PascalArray<RtcZUnknown1Unknown5>,
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
    unknown2s: PascalArray<RtcZUnknown2Unknown2>,
    unknown3flag: u16,
    unknown3s: PascalArray<RtcZUnknown2Unknown2>,
    unknown4flag: u16,
    unknown4s: PascalArray<RtcZUnknown2Unknown4>,
    unknown5flag: u16,
    unknown5s: PascalArray<RtcZUnknown2Unknown2>,
}

#[derive(Serialize, Deserialize, NomLE)]
struct RtcZUnknown4RtcZUnknown5Unknown {
    unknown0: u32,
    unknown1: u32,
}

#[derive(Serialize, Deserialize, NomLE)]
struct RtcZUnknown4RtcZUnknown5 {
    unknowns: FixedVec<RtcZUnknown4RtcZUnknown5Unknown, 3>,
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
    unknown5s: PascalArray<RtcZUnknown4RtcZUnknown5>,
    unknown6flag: u16,
    unknown6s: PascalArray<RtcZUnknown4RtcZUnknown6>,
    unknown7flag: u16,
    unknown7s: PascalArray<RtcZUnknown4RtcZUnknown6>,
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
struct RtcZUnknown12 {
    unknown0: u32,
    unknown1s: PascalArray<RtcZUnknown12Unknown1>,
}

#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
struct RtcZ {
    unknown0: f32,
    unknown1s: PascalArray<RtcZUnknown1>,
    unknown2s: PascalArray<RtcZUnknown2>,
    unknown3s: PascalArray<u32>,
    unknown4s: PascalArray<RtcZUnknown4>,
    unknown8s: PascalArray<RtcZUnknown8>,
    unknown9s: PascalArray<RtcZUnknown9>,
    unknown10s: PascalArray<u32>,
    unknown11s: PascalArray<u32>,
    unknown12s: PascalArray<RtcZUnknown12>,
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
