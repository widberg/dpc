use std::io::Result;
use std::io::Write;
use std::path::Path;

use nom::number::complete::*;
use nom_derive::{NomLE, Parse};
use serde::{Deserialize, Serialize};

use crate::File;
use crate::fuel_fmt::common::ResourceObjectZ;

#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
struct LodDataZ {
    unknown_byte0: u8,
    unknown_byte1: u8,
    zero_byte0: u8,
    zero_byte1: u8,
    #[nom(LengthCount(le_u32))]
    crc32s: Vec<u32>,
    zero0: u32,
    #[serde(skip_serializing)]
    #[allow(dead_code)]
    opt: u8,
    #[nom(Cond = "opt != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[nom(Count(24))]
    padding: Option<Vec<u8>>,
    #[nom(Cond = "opt != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    u1: Option<u32>,
    #[nom(Cond = "opt != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    zero1: Option<u32>,
    #[nom(Cond = "opt != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    u2: Option<u32>,
    #[nom(Cond = "opt != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    zero2: Option<u32>,
    #[nom(Cond = "opt != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    zero3: Option<u32>,
    #[nom(Cond = "opt != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    zero4: Option<u32>,
    #[nom(Cond = "opt != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    scale_x: Option<f32>,
    #[nom(Cond = "opt != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    scale_y: Option<f32>,
    #[nom(Cond = "opt != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    scale_z: Option<f32>,
    #[nom(Cond = "opt != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    zero5: Option<u32>,
    #[nom(Cond = "opt != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    zero6: Option<u32>,
    #[nom(Cond = "opt != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    zero7: Option<u32>,
    #[nom(Cond = "opt != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    u6: Option<u32>,
    #[nom(Cond = "opt != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    zero8: Option<u32>,
    #[nom(Cond = "opt != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    zero9: Option<u32>,
    #[nom(Cond = "opt != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    zero10: Option<u32>,
    #[nom(Cond = "opt != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    zero11: Option<u32>,
}

#[derive(Serialize, Deserialize)]
struct LodDataObject {
    resource_object: ResourceObjectZ,
    lod_data: LodDataZ,
}

pub fn fuel_fmt_extract_lod_data_z(header: &[u8], data: &[u8], output_path: &Path) -> Result<()> {
    let json_path = output_path.join("object.json");
    let mut output_file = File::create(json_path)?;

    let resource_object = match ResourceObjectZ::parse(&header) {
        Ok((_, h)) => h,
        Err(error) => panic!("{}", error),
    };

    let lod_data = match LodDataZ::parse(&data) {
        Ok((_, h)) => h,
        Err(error) => panic!("{}", error),
    };

    let object = LodDataObject {
        resource_object,
        lod_data,
    };

    output_file.write(serde_json::to_string_pretty(&object)?.as_bytes())?;

    Ok(())
}
