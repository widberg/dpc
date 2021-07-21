use nom_derive::NomLE;
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::{FixedVec, Mat4f, ObjectZ, PascalArray, FUELObjectFormat, Vec2f, Vec3f, Vec4f};

#[derive(Serialize, Deserialize, NomLE)]
struct ParticlesZUnknown1 {
    unknown0: u32,
    unknown1: u32,
    unknown2: u32,
}

#[derive(Serialize, Deserialize, NomLE)]
struct ParticlesZUnknown2 {
    unknown0: u32,
    unknown1: u32,
    unknown2: u32,
    unknown3: u32,
    unknown4: u32,
}

#[derive(Serialize, Deserialize, NomLE)]
struct ParticlesZUnknown4 {
    unknown0: u32,
    unknown1: u32,
}

#[derive(Serialize, Deserialize, NomLE)]
struct ParticlesZUnknown5 {
    unknown0: u32,
    unknown1: u32,
    unknown2: u32,
    unknown3: u32,
}

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

#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct ParticlesZ {
    unknown0s: PascalArray<ParticlesZUnknown0>,
    mats: PascalArray<Mat4f>,
    unknown2: u32,
    unknown3: u16,
}

#[derive(Serialize, Deserialize, NomLE)]
struct ParticlesZUnknown0Alt {
    data: FixedVec<u32, 19>,
    unknown1flag: u16,
    unknown1s: PascalArray<Vec2f>,
    unknown2flag: u16,
    unknown2s: PascalArray<Vec3f>,
    unknown3flag: u16,
    unknown3s: PascalArray<Vec3f>,
    unknown4flag: u16,
    unknown4s: PascalArray<Vec2f>,
    unknown5flag: u16,
    unknown5s: PascalArray<Vec4f>,
    unknown6flag: u16,
    unknown6s: PascalArray<Vec4f>,
    unknown7flag: u16,
    unknown7s: PascalArray<Vec2f>,
    unknown8: u32,
}

#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct ParticlesZAlt {
    unknown0s: PascalArray<ParticlesZUnknown0Alt>,
    mats: PascalArray<Mat4f>,
    unknown2: u32,
    unknown3: u16,
}

pub type ParticlesObjectFormat = FUELObjectFormat<ObjectZ, ParticlesZ>;
pub type ParticlesObjectFormatAlt = FUELObjectFormat<ObjectZ, ParticlesZAlt>;
