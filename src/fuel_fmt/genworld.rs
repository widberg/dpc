use std::io::Result;
use std::io::Write;
use std::path::Path;

use nom_derive::{NomLE, Parse};
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::{FixedVec, Mat4f, ObjectZ, PascalArray};
use crate::File;

#[derive(Serialize, Deserialize, NomLE)]
struct GenWorldZUnknown7 {
    unknown0s: PascalArray<u8>,
    unknown1s: PascalArray<PascalArray<u32>>,
}

#[derive(Serialize, Deserialize, NomLE)]
struct GenWorldZUnknown8 {
    unknown0: u32,
    data: FixedVec<u8, 127>,
}

#[derive(Serialize, Deserialize, NomLE)]
struct GenWorldZUnknown10 {
    unknown0: u32,
    unknown1s: FixedVec<u32, 8>,
    unknown2: u32,
    unknown3: u32,
    unknown4: u32,
}

#[derive(Serialize, Deserialize, NomLE)]
struct GenWorldZUnknown11 {
    unknown0: u32,
    unknown1: u32,
}

#[derive(Serialize, Deserialize, NomLE)]
struct GenWorldZUnknown12 {
    unknown0: u32,
    unknown1: u32,
}

#[derive(Serialize, Deserialize, NomLE)]
struct GenWorldZUnknown13 {
    unknown0s: FixedVec<u32, 8>,
    unknown1s: PascalArray<u32>,
}

#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
struct GenWorldZ {
    unknown0: u32,
    unknown1: u32,
    unknown2: u32,
    unknown3s: PascalArray<u32>,
    unknown4s: PascalArray<u32>,
    unknown5s: PascalArray<u32>,
    unknown6: u32,
    unknown7s: PascalArray<GenWorldZUnknown7>,
    unknown8s: PascalArray<GenWorldZUnknown8>,
    mats: PascalArray<Mat4f>,
    unknown10s: PascalArray<GenWorldZUnknown10>,
    unknown11s: PascalArray<GenWorldZUnknown11>,
    unknown12s: PascalArray<GenWorldZUnknown12>,
    unknown13s: PascalArray<GenWorldZUnknown13>,
}

#[derive(Serialize, Deserialize)]
struct GenWorldObject {
    object: ObjectZ,
    gen_world: GenWorldZ,
}

pub fn fuel_fmt_extract_gen_world_z(header: &[u8], data: &[u8], output_path: &Path) -> Result<()> {
    let json_path = output_path.join("object.json");
    let mut output_file = File::create(json_path)?;

    let object = match ObjectZ::parse(&header) {
        Ok((_, h)) => h,
        Err(error) => panic!("{}", error),
    };

    let gen_world = match GenWorldZ::parse(&data) {
        Ok((_, h)) => h,
        Err(error) => panic!("{}", error),
    };

    let object = GenWorldObject { object, gen_world };

    output_file.write(serde_json::to_string_pretty(&object)?.as_bytes())?;

    Ok(())
}
