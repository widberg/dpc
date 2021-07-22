use nom_derive::NomLE;
use serde::{Deserialize, Serialize};
use binwrite::BinWrite;

use crate::fuel_fmt::common::{FixedVec, FUELObjectFormat, PascalArray, ResourceObjectZ};

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct RtcZUnknown1Unknown2 {
    unknown0: u32,
    unknown1: u32,
    unknown2: u32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct RtcZUnknown1Unknown3Unknown {
    unknown0: u32,
    unknown1: u32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct RtcZUnknown1Unknown3 {
    unknowns: FixedVec<RtcZUnknown1Unknown3Unknown, 5>,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct RtcZUnknown1Unknown5Unknown1 {
    unknown0: u32,
    unknown1: u32,
    unknown2: u32,
    unknown3: u32,
    unknown4: u32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct RtcZUnknown1Unknown5 {
    unknown0: u32,
    unknown1s: PascalArray<RtcZUnknown1Unknown5Unknown1>,
}

#[derive(BinWrite)]
#[binwrite(little)]
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

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct RtcZUnknown2Unknown2 {
    unknown0: u32,
    unknown1: u32,
    unknown2: u32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct RtcZUnknown2Unknown4 {
    unknown0: u32,
    unknown1: u32,
    unknown2: u32,
    unknown3: u32,
}

#[derive(BinWrite)]
#[binwrite(little)]
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

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct RtcZUnknown4RtcZUnknown5Unknown {
    unknown0: u32,
    unknown1: u32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct RtcZUnknown4RtcZUnknown5 {
    unknowns: FixedVec<RtcZUnknown4RtcZUnknown5Unknown, 3>,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct RtcZUnknown4RtcZUnknown6 {
    unknown0: u32,
    unknown1: u32,
    unknown2: u32,
}

#[derive(BinWrite)]
#[binwrite(little)]
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

#[derive(BinWrite)]
#[binwrite(little)]
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

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct RtcZUnknown9 {
    unknown0: u32,
    unknown1: u32,
    unknown2: u32,
    unknown3: u32,
    unknown4: u32,
    unknown5: u32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct RtcZUnknown12Unknown1 {
    unknown0: u32,
    unknown1: u32,
    unknown2: u32,
    unknown3: u32,
    unknown4: u32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct RtcZUnknown12 {
    unknown0: u32,
    unknown1s: PascalArray<RtcZUnknown12Unknown1>,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct RtcZ {
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

pub type RtcObjectFormat = FUELObjectFormat<ResourceObjectZ, RtcZ>;
