use binwrite::BinWrite;
use nom_derive::NomLE;
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::{FUELObjectFormat, FixedVec, Mat4f, PascalArray, ResourceObjectZ};

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct SkelZBone {
    unknown0: u32,
    data0: FixedVec<u8, 136>,
    transformation: Mat4f,
    unknown1: u32,
    parent_index: u32,
    data1: FixedVec<u8, 16>,
    unknown2: u32,
    unknown3: u32,
    unknown4: u32,
    unknown5: u32,
    unknown6: u32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct SkelZUnknown4 {
    unknown0: u32,
    unknown1: u32,
    unknown2: u32,
    unknown3: u32,
    unknown4: u32,
    unknown5: u32,
    unknown6: u32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct SkelZUnknown2 {
    mat: Mat4f,
    unknown0: u32,
    unknown1: u32,
    unknown2: u32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct SkelZ {
    u0: u32,
    u1: f32,
    u2: f32,
    u3: f32,
    u4: f32,
    bones: PascalArray<SkelZBone>,
    material_crc32s: PascalArray<u32>,
    mesh_data_crc32s: PascalArray<u32>,
    unknown5s: PascalArray<PascalArray<u32>>,
    unknown3: PascalArray<u32>,
    unknown4s: PascalArray<SkelZUnknown4>,
    unknown1s: PascalArray<SkelZUnknown4>,
    unknown2s: PascalArray<SkelZUnknown2>,
}

pub type SkelObjectFormat = FUELObjectFormat<ResourceObjectZ, SkelZ>;
