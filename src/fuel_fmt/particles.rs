use binwrite::BinWrite;
use nom_derive::NomLE;
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::{FUELObjectFormat, FixedVec, Mat4f, ObjectZ, PascalArray};

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct ParticlesZUnknown1 {
    unknown0: u32,
    unknown1: u32,
    unknown2: u32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct ParticlesZUnknown2 {
    unknown0: u32,
    unknown1: u32,
    unknown2: u32,
    unknown3: u32,
    unknown4: u32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct ParticlesZUnknown4 {
    unknown0: u32,
    unknown1: u32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct ParticlesZUnknown5 {
    unknown0: u32,
    unknown1: u32,
    unknown2: u32,
    unknown3: u32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct ParticlesZUnknown0 {
    data: FixedVec<u32, 19>,
    unknown1flag: u16,
    unknown1s: PascalArray<ParticlesZUnknown1>,
    unknown2flag: u16,
    unknown2s: PascalArray<ParticlesZUnknown2>,
    unknown3flag: u16,
    unknown3s: PascalArray<ParticlesZUnknown2>,
    unknown4flag: u16,
    unknown4s: PascalArray<ParticlesZUnknown4>,
    unknown5flag: u16,
    unknown5s: PascalArray<ParticlesZUnknown5>,
    unknown6flag: u16,
    unknown6s: PascalArray<ParticlesZUnknown5>,
    unknown7flag: u16,
    unknown7s: PascalArray<ParticlesZUnknown4>,
    unknown8: u32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct ParticlesZ {
    unknown0s: PascalArray<ParticlesZUnknown0>,
    mats: PascalArray<Mat4f>,
    unknown2: u32,
    unknown3: u16,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct ParticlesZUnknown0Alt {
    data: FixedVec<u32, 19>,
    unknown1flag: u16,
    unknown1s: PascalArray<FixedVec<u32, 2>>,
    unknown2flag: u16,
    unknown2s: PascalArray<FixedVec<u32, 3>>,
    unknown3flag: u16,
    unknown3s: PascalArray<FixedVec<u32, 3>>,
    unknown4flag: u16,
    unknown4s: PascalArray<FixedVec<u32, 2>>,
    unknown5flag: u16,
    unknown5s: PascalArray<FixedVec<u32, 4>>,
    unknown6flag: u16,
    unknown6s: PascalArray<FixedVec<u32, 4>>,
    unknown7flag: u16,
    unknown7s: PascalArray<FixedVec<u32, 2>>,
    unknown8: u32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct ParticlesZAlt {
    unknown0s: PascalArray<ParticlesZUnknown0Alt>,
    mats: PascalArray<FixedVec<u32, 16>>,
    unknown2: u32,
    unknown3: u16,
}

pub type ParticlesObjectFormat = FUELObjectFormat<ObjectZ, ParticlesZ>;
pub type ParticlesObjectFormatAlt = FUELObjectFormat<ObjectZ, ParticlesZAlt>;
