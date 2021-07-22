use binwrite::BinWrite;
use nom_derive::NomLE;
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::{FUELObjectFormat, PascalArray, ResourceObjectZ};

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct MaterialAnimZUnknown0 {
    unknown0: f32,
    unknown1: f32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct MaterialAnimZUnknown23 {
    unknown0: f32,
    unknown1: f32,
    unknown2: f32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct MaterialAnimZUnknown56 {
    unknown0: f32,
    unknown1: f32,
    unknown2: f32,
    unknown3: f32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct MaterialAnimZUnknown89 {
    unknown0: f32,
    unknown1: f32,
    unknown2: f32,
    unknown3: f32,
    unknown4: f32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct MaterialAnimZUnknown1011 {
    unknown0: f32,
    unknown1: f32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct MaterialAnimZColor {
    unknown: f32,
    rgba: u32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct MaterialAnimZ {
    unknown0s: PascalArray<MaterialAnimZUnknown0>,
    unknown2flag: u16,
    unknown2s: PascalArray<MaterialAnimZUnknown23>,
    unknown3flag: u16,
    unknown3s: PascalArray<MaterialAnimZUnknown23>,
    unknown4flag: u16,
    unknown4s: PascalArray<MaterialAnimZColor>,
    unknown5flag: u16,
    unknown5s: PascalArray<MaterialAnimZUnknown56>,
    unknown6flag: u16,
    unknown6s: PascalArray<MaterialAnimZUnknown56>,
    colorsflag: u16,
    colors: PascalArray<MaterialAnimZColor>,
    unknown8flag: u16,
    unknown8s: PascalArray<MaterialAnimZUnknown89>,
    unknown9flag: u16,
    unknown9s: PascalArray<MaterialAnimZUnknown89>,
    unknown10s: PascalArray<MaterialAnimZUnknown1011>,
    unknown11s: PascalArray<MaterialAnimZUnknown1011>,
    material_crc32: u32,
    unknown_float: f32,
    unknown15: u8,
}

pub type MaterialAnimObjectFormat = FUELObjectFormat<ResourceObjectZ, MaterialAnimZ>;
