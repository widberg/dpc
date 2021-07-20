use std::io::Result;
use std::io::Write;
use std::path::Path;

use nom_derive::{NomLE, Parse};
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::{FixedVec, PascalArray, ResourceObjectZ, Vec3f};
use crate::File;

#[derive(Serialize, Deserialize, NomLE)]
struct AnimationZUnknown0 {
    data: FixedVec<u8, 40>,
}

#[derive(Serialize, Deserialize, NomLE)]
struct AnimationZUnknown {
    unknown0: u32,
    unknown1: u32,
}

#[derive(Serialize, Deserialize, NomLE)]
struct AnimationZUnknown2 {
    unknowns: FixedVec<AnimationZUnknown, 3>,
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
    unknown1s: PascalArray<AnimationZUnknown1>,
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
