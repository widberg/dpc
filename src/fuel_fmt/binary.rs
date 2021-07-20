use std::io::Result;
use std::io::Write;
use std::path::Path;

use nom_derive::Parse;
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::ResourceObjectZ;
use crate::File;

#[derive(Serialize, Deserialize)]
struct BinaryObject {
    resource_object: ResourceObjectZ,
}

pub fn fuel_fmt_extract_binary_z(header: &[u8], data: &[u8], output_path: &Path) -> Result<()> {
    let json_path = output_path.join("object.json");
    let mut output_file = File::create(json_path)?;

    let bin_path = output_path.join("data.bin");
    let mut output_bin_file = File::create(bin_path)?;

    let resource_object = match ResourceObjectZ::parse(&header) {
        Ok((_, h)) => h,
        Err(error) => panic!("{}", error),
    };

    let object = BinaryObject { resource_object };

    output_file.write(serde_json::to_string_pretty(&object)?.as_bytes())?;

    output_bin_file.write(&data)?;

    Ok(())
}
