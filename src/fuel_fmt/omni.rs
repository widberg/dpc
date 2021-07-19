use std::io::Result;
use std::io::Write;
use std::path::Path;

use nom_derive::{NomLE, Parse};
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::ObjectZ;
use crate::File;

#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
struct OmniZ {
    #[nom(Count(48))]
    data: Vec<u32>,
    #[nom(Count(2))]
    crc32s: Vec<u32>,
}

#[derive(Serialize, Deserialize)]
struct OmniObject {
    object: ObjectZ,
    omni: OmniZ,
}

pub fn fuel_fmt_extract_omni_z(header: &[u8], data: &[u8], output_path: &Path) -> Result<()> {
    let json_path = output_path.join("object.json");
    let mut output_file = File::create(json_path)?;

    let object = match ObjectZ::parse(&header) {
        Ok((_, h)) => h,
        Err(error) => panic!("{}", error),
    };

    let omni = match OmniZ::parse(&data) {
        Ok((_, h)) => h,
        Err(error) => panic!("{}", error),
    };

    let object = OmniObject { object, omni };

    output_file.write(serde_json::to_string_pretty(&object)?.as_bytes())?;

    Ok(())
}
