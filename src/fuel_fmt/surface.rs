use binwrite::BinWrite;
use nom_derive::NomLE;
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::{
    write_option, FUELObjectFormat, FixedVec, ObjectZ, PascalArray, Quat, Vec3f,
};

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct SurfaceZUnknown2 {
    data: FixedVec<u8, 32>,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct SurfaceZUnknown7 {
    unknown0: u32,
    unknown1: u32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct SurfaceZUnknown8 {
    unknown0: u32,
    unknown1: u32,
    unknown2: u32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct SurfaceZUnknown4 {
    data: FixedVec<u32, 43>,
    unknown: u32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct SurfaceZ {
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
    #[serde(skip_deserializing)]
    #[allow(dead_code)]
    #[binwrite(ignore)]
    opt: u8,
    #[binwrite(postprocessor(|x: Vec<u8>| -> (u8, Vec<u8>) { if x.len() != 0 { (1u8, x) } else { (0u8, x) } }))]
    #[nom(Cond = "opt != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[binwrite(with(write_option))]
    unknown13s: Option<PascalArray<u32>>,
    #[nom(Cond = "opt != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[binwrite(with(write_option))]
    unknown14s: Option<PascalArray<u16>>,
    #[nom(Cond = "opt != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[binwrite(with(write_option))]
    unknown15: Option<FixedVec<u32, 52>>,
    #[nom(Cond = "opt != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[binwrite(with(write_option))]
    unknown16_count: Option<u32>,
}

pub type SurfaceObjectFormat = FUELObjectFormat<ObjectZ, SurfaceZ>;
