use nom_derive::NomLE;
use serde::{Deserialize, Serialize};
use binwrite::BinWrite;

use crate::fuel_fmt::common::{FUELObjectFormat, Mat4f, ObjectZ, PascalArray, write_option};

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct LodZUnknown0 {
    a: f32,
    b: f32,
    c: f32,
    d: f32,
    e: u32,
    f: f32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct LodZUnknown1 {
    transformation: Mat4f,
    q: u32,
    r: f32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct LodZSoundEntry {
    id: u32,
    sound_crc32: u32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct LodZUnknown4 {
    a: u32,
    b: u32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct LodZ {
    unknown0s: PascalArray<LodZUnknown0>,
    unknown1s: PascalArray<LodZUnknown1>,
    unknown2: u32,
    unknown3: u32,
    u0: f32,
    skin_crc32s: PascalArray<u32>,
    u1: u32,
    #[serde(skip_serializing)]
    #[allow(dead_code)]
    sound_entries_option: u32,
    #[nom(Cond(sound_entries_option != 0))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[binwrite(with(write_option))]
    sound_entries: Option<PascalArray<LodZSoundEntry>>,
    #[serde(skip_serializing)]
    #[allow(dead_code)]
    unknown4_option: u32,
    #[nom(Cond(unknown4_option != 0))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[binwrite(with(write_option))]
    unknown4s: Option<PascalArray<LodZUnknown4>>,
    unknown5: u32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct LodZAlt {
    x: u32,
    #[nom(Cond(x != 0))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[binwrite(with(write_option))]
    unused0: Option<u32>,
    #[nom(Cond(x != 0))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[binwrite(with(write_option))]
    unknown0_optional: Option<LodZUnknown0>,
    unknown0s: PascalArray<LodZUnknown0>,
    unknown1s: PascalArray<LodZUnknown1>,
    unknown2: u32,
    unknown3: u32,
    unknown4: u32,
    u0: f32,
    skin_crc32s: PascalArray<u32>,
    u1: u32,
    #[serde(skip_serializing)]
    #[allow(dead_code)]
    sound_entries_option: u8,
    #[nom(Cond(sound_entries_option != 0))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[binwrite(with(write_option))]
    sound_entries: Option<PascalArray<LodZSoundEntry>>,
    #[serde(skip_serializing)]
    #[allow(dead_code)]
    unknown4_option: u8,
    #[nom(Cond(unknown4_option != 0))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[binwrite(with(write_option))]
    unknown4s: Option<PascalArray<LodZUnknown4>>,
    unknown5: u32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct LodZAltAlt {
    x: u32,
    #[nom(Cond(x != 0))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[binwrite(with(write_option))]
    unused0: Option<u32>,
    #[nom(Cond(x != 0))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[binwrite(with(write_option))]
    unknown0_optional: Option<LodZUnknown0>,
    unknown0s: PascalArray<LodZUnknown0>,
    unknown1s: PascalArray<LodZUnknown1>,
    unknown2: u32,
    unknown3: u32,
    unknown4: u32,
    u0: f32,
    skin_crc32s: PascalArray<u32>,
    u1: u32,
    #[serde(skip_serializing)]
    #[allow(dead_code)]
    sound_entries_option: u32,
    #[nom(Cond(sound_entries_option != 0))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[binwrite(with(write_option))]
    sound_entries: Option<PascalArray<LodZSoundEntry>>,
    #[serde(skip_serializing)]
    #[allow(dead_code)]
    unknown4_option: u32,
    #[nom(Cond(unknown4_option != 0))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[binwrite(with(write_option))]
    unknown4s: Option<PascalArray<LodZUnknown4>>,
    unknown5: u32,
}

pub type LodObjectFormat = FUELObjectFormat<ObjectZ, LodZ>;
pub type LodObjectFormatAlt = FUELObjectFormat<ObjectZ, LodZAlt>;
pub type LodObjectFormatAltAlt = FUELObjectFormat<ObjectZ, LodZAltAlt>;
