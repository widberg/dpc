use std::io::Result;
use std::io::Write;
use std::path::Path;

use nom::number::complete::*;
use nom::*;
use nom_derive::{NomLE, Parse};
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::{PascalArray, ResourceObjectZ};
use crate::File;

#[derive(Serialize, Deserialize, NomLE)]
struct GameObjZChild {
    #[nom(
        Map = "|x: Vec<u8>| String::from_utf8_lossy(&x[0..x.len() - 1]).to_string()",
        Parse = "|i| length_count!(i, le_u32, le_u8)"
    )]
    string: String,
    is_in_world: u32,
    crc32s: PascalArray<u32>,
}

#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
struct GameObjZ {
    children: PascalArray<GameObjZChild>,
}

#[derive(Serialize, Deserialize)]
struct GameObjObject {
    resource_object: ResourceObjectZ,
    game_obj: GameObjZ,
}

pub fn fuel_fmt_extract_game_obj_z(header: &[u8], data: &[u8], output_path: &Path) -> Result<()> {
    let json_path = output_path.join("object.json");
    let mut output_file = File::create(json_path)?;

    let resource_object = match ResourceObjectZ::parse(&header) {
        Ok((_, h)) => h,
        Err(error) => panic!("{}", error),
    };

    let game_obj = match GameObjZ::parse(&data) {
        Ok((_, h)) => h,
        Err(error) => panic!("{}", error),
    };

    let object = GameObjObject {
        resource_object,
        game_obj,
    };

    output_file.write(serde_json::to_string_pretty(&object)?.as_bytes())?;

    Ok(())
}
