use nom_derive::NomLE;
use serde::{Deserialize, Serialize};
use binwrite::BinWrite;

use crate::fuel_fmt::common::{FixedVec, FUELObjectFormat, PascalArray, ResourceObjectZ, Vec3f};

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct AnimationZUnknown0 {
    data: FixedVec<u8, 40>,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct AnimationZUnknown {
    unknown0: u32,
    unknown1: u32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct AnimationZUnknown2 {
    unknowns: FixedVec<AnimationZUnknown, 3>,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct AnimationZUnknown1 {
    unknown0: u32,
    unknown1: u32,
    unknown2: u32,
    unknown3: u32,
    unknown4: u32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct AnimationZUnknown4 {
    unknown0: u32,
    unknown1s: PascalArray<AnimationZUnknown1>,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct AnimationZUnknown5 {
    unknown0: u32,
    unknown1: u32,
    unknown2: u32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct AnimationZUnknown12 {
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
#[nom(Exact)]
pub struct AnimationZ {
    a: f32,
    b: f32,
    c: u16,
    d: u16,
    vectors: PascalArray<Vec3f>,
    unknown0s: PascalArray<AnimationZUnknown0>,
    unknown2flag: u16,
    unknown2s: PascalArray<AnimationZUnknown2>,
    unknown3flag: u16,
    unknown3s: PascalArray<AnimationZUnknown2>,
    unknown4s: PascalArray<AnimationZUnknown4>,
    unknown5flag: u16,
    unknown5s: PascalArray<AnimationZUnknown5>,
    unknown6flag: u16,
    unknown6s: PascalArray<AnimationZUnknown5>,
    unknown7flag: u16,
    unknown7s: PascalArray<AnimationZUnknown2>,
    unknown8flag: u16,
    unknown8s: PascalArray<AnimationZUnknown2>,
    unknown9flag: u16,
    unknown9s: PascalArray<AnimationZUnknown5>,
    unknown10flag: u16,
    unknown10s: PascalArray<AnimationZUnknown5>,
    unknown11flag: u16,
    unknown11s: PascalArray<AnimationZUnknown5>,
    unknown12s: PascalArray<AnimationZUnknown12>,
    unknown13s: PascalArray<AnimationZUnknown12>,
    unknown14s: PascalArray<AnimationZUnknown5>,
    unknown15s: PascalArray<AnimationZUnknown5>,
}

pub type AnimationObjectFormat = FUELObjectFormat<ResourceObjectZ, AnimationZ>;
