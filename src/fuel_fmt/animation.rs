use nom_derive::{NomLE, Parse};
use serde::{Serialize, Deserialize};
use std::path::Path;
use crate::File;
use std::io::Result;
use std::io::Write;
use nom::number::complete::*;

use crate::fuel_fmt::common::{ResourceObjectZ, Vec3f};

#[derive(Serialize, Deserialize, NomLE)]
struct AnimationZUnknown0 {
    #[nom(Count(40))]
    data: Vec<u8>,
}

#[derive(Serialize, Deserialize, NomLE)]
struct AnimationZUnknown
{
    unknown0: u32,
    unknown1: u32,
}

#[derive(Serialize, Deserialize, NomLE)]
struct AnimationZUnknown2 {
    #[nom(Count(3))]
    unknowns: Vec<AnimationZUnknown>,
}

#[derive(Serialize, Deserialize, NomLE)]
struct AnimationZUnknown1 {
    unknown0: u32,
    unknown1: u32,
    unknown2: u32,
    unknown3: u32,
    unknown4: u32,
}

#[derive(Serialize, Deserialize, NomLE)]
struct AnimationZUnknown4 {
    unknown0: u32,
    #[nom(LengthCount(le_u32))]
    unknown1s: Vec<AnimationZUnknown1>
}

#[derive(Serialize, Deserialize, NomLE)]
struct AnimationZUnknown5 {
    unknown0: u32,
    unknown1: u32,
    unknown2: u32,
}

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

#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
struct AnimationZ {
    a: f32,
    b: f32,
    c: u16,
    d: u16,
    #[nom(LengthCount(le_u32))]
    vectors: Vec<Vec3f>,
    #[nom(LengthCount(le_u32))]
    unknown0s: Vec<AnimationZUnknown0>,
    unknown2flag: u16,
    #[nom(LengthCount(le_u32))]
    unknown2s: Vec<AnimationZUnknown2>,
    unknown3flag: u16,
    #[nom(LengthCount(le_u32))]
    unknown3s: Vec<AnimationZUnknown2>,
    #[nom(LengthCount(le_u32))]
    unknown4s: Vec<AnimationZUnknown4>,
    unknown5flag: u16,
    #[nom(LengthCount(le_u32))]
    unknown5s: Vec<AnimationZUnknown5>,
    unknown6flag: u16,
    #[nom(LengthCount(le_u32))]
    unknown6s: Vec<AnimationZUnknown5>,
    unknown7flag: u16,
    #[nom(LengthCount(le_u32))]
    unknown7s: Vec<AnimationZUnknown2>,
    unknown8flag: u16,
    #[nom(LengthCount(le_u32))]
    unknown8s: Vec<AnimationZUnknown2>,
    unknown9flag: u16,
    #[nom(LengthCount(le_u32))]
    unknown9s: Vec<AnimationZUnknown5>,
    unknown10flag: u16,
    #[nom(LengthCount(le_u32))]
    unknown10s: Vec<AnimationZUnknown5>,
    unknown11flag: u16,
    #[nom(LengthCount(le_u32))]
    unknown11s: Vec<AnimationZUnknown5>,
    #[nom(LengthCount(le_u32))]
    unknown12s: Vec<AnimationZUnknown12>,
    #[nom(LengthCount(le_u32))]
    unknown13s: Vec<AnimationZUnknown12>,
    #[nom(LengthCount(le_u32))]
    unknown14s: Vec<AnimationZUnknown5>,
    #[nom(LengthCount(le_u32))]
    unknown15s: Vec<AnimationZUnknown5>,
}

#[derive(Serialize, Deserialize)]
struct AnimationObject {
    resource_object: ResourceObjectZ,
    animation: AnimationZ,
}

pub fn fuel_fmt_extract_animation_z(header: &[u8], data: &[u8], output_path: &Path) -> Result<()> {
    let json_path = output_path.join("object.json");
    let mut output_file = File::create(json_path)?;

    let resource_object = match ResourceObjectZ::parse(&header) {
        Ok((_, h)) => h,
        Err(error) => panic!("{}", error),
    };

    let animation = match AnimationZ::parse(&data) {
        Ok((_, h)) => h,
        Err(error) => panic!("{}", error),
    };

    let object = AnimationObject {
        resource_object,
        animation,
    };

    output_file.write(serde_json::to_string_pretty(&object)?.as_bytes())?;

    Ok(())
}
