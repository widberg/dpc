use std::io::Result;
use std::io::Write;
use std::path::Path;

use nom::number::complete::*;
use nom::*;
use nom_derive::{NomLE, Parse};
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::ResourceObjectZ;
use crate::File;

#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
struct UserDefineZ {
    #[nom(
        Map = "|x: Vec<u8>| String::from_utf8_lossy(&x[..]).to_string()",
        Parse = "|i| length_count!(i, le_u32, le_u8)"
    )]
    data: String,
}

#[derive(Serialize, Deserialize)]
struct UserDefineObject {
    resource_object: ResourceObjectZ,
    user_define: UserDefineZ,
}

pub fn fuel_fmt_extract_user_define_z(
    header: &[u8],
    data: &[u8],
    output_path: &Path,
) -> Result<()> {
    let json_path = output_path.join("object.json");
    let mut output_file = File::create(json_path)?;

    let resource_object = match ResourceObjectZ::parse(&header) {
        Ok((_, h)) => h,
        Err(error) => panic!("{}", error),
    };

    let user_define = match UserDefineZ::parse(&data) {
        Ok((_, h)) => h,
        Err(error) => panic!("{}", error),
    };

    let object = UserDefineObject {
        resource_object,
        user_define,
    };

    output_file.write(serde_json::to_string_pretty(&object)?.as_bytes())?;

    Ok(())
}
