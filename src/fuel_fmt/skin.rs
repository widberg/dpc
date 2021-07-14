use nom_derive::{NomLE, Parse};
use serde::{Serialize, Deserialize};
use std::path::Path;
use crate::File;
use std::io::Result;
use std::io::Write;
use nom::number::complete::*;

use crate::fuel_fmt::common::{ObjectZ};

static mut SKIN_DATA_COUNT: u32 = 0;

#[derive(Serialize, Deserialize, NomLE)]
struct SkinZSkinSubsection {
    vertex_group_crc32: u32,
    unknown_crc320: u32,
    unknown_crc321: u32,
    unknown_crc322: u32,
    #[nom(Count(unsafe { SKIN_DATA_COUNT as usize }))]
    data: Vec<u32>,
}

#[derive(Serialize, Deserialize, NomLE)]
struct SkinZSkinSection {
    #[nom(LengthCount(le_u32))]
    skin_subsections: Vec<SkinZSkinSubsection>,
}

#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
struct SkinZ {
    #[nom(LengthCount(le_u32))]
    mesh_crc32s: Vec<u32>,
    u0: u32,
    u1: u32,
    u2: u32,
    u3: u32,
    one_and_a_half: f32,
    #[nom(PostExec(unsafe { SKIN_DATA_COUNT = data_count }))]
    data_count: u32,
    #[nom(LengthCount(le_u32))]
    skin_sections: Vec<SkinZSkinSection>,
}

#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
struct SkinZAlt {
    #[nom(Count(i.len()))]
    data: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
struct SkinObject {
    object: ObjectZ,
    skin: SkinZ,
}

#[derive(Serialize, Deserialize)]
struct SkinObjectAlt {
    object: ObjectZ,
    skin: SkinZAlt,
}

pub fn fuel_fmt_extract_skin_z(header: &[u8], data: &[u8], output_path: &Path) -> Result<()> {
    let json_path = output_path.join("object.json");
    let mut output_file = File::create(json_path)?;

    let object = match ObjectZ::parse(&header) {
        Ok((_, h)) => h,
        Err(error) => panic!("{}", error),
    };

    let skin = match SkinZ::parse(&data) {
        Ok((_, h)) => h,
        Err(_) => match SkinZAlt::parse(&data) {
            Ok((_, skin)) => {
                let object = SkinObjectAlt {
                    object,
                    skin,
                };

                output_file.write(serde_json::to_string_pretty(&object)?.as_bytes())?;

                return Ok(());
            },
            Err(error) => panic!("{}", error),
        },
    };

    let object = SkinObject {
        object,
        skin,
    };

    output_file.write(serde_json::to_string_pretty(&object)?.as_bytes())?;

    Ok(())
}
