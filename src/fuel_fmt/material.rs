use std::io::Result;
use std::io::Write;
use std::path::Path;

use nom_derive::{NomLE, Parse};
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::{ResourceObjectZ, Vec3f, Vec4f};
use crate::File;

#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
struct MaterialZ {
    color: Vec4f,
    emission: Vec3f,
    unknown0: i32,
    #[nom(Count = "26")]
    vertex_shader_constant_fs: Vec<f32>,
    diffuse_bitmap_crc32: u32,
    unknown_bitmap_crc320: u32,
    metal_bitmap_crc32: u32,
    unknown_bitmap_crc321: u32,
    grey_bitmap_crc32: u32,
    normal_bitmap_crc32: u32,
    dirt_bitmap_crc32: u32,
    unknown_bitmap_crc322: u32,
    unknown_bitmap_crc323: u32,
}

#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
struct MaterialZAlt {
    color: Vec4f,
    emission: Vec3f,
    unknown0: i32,
    #[nom(Count = "28")]
    vertex_shader_constant_fs: Vec<f32>,
    #[serde(skip_serializing)]
    #[allow(dead_code)]
    opt: u8,
    #[nom(Cond(opt != 0))]
    #[serde(skip_serializing_if = "Option::is_none")]
    unknown_crc320: Option<u32>,
    #[nom(Cond(opt != 0))]
    #[serde(skip_serializing_if = "Option::is_none")]
    unknown_crc321: Option<u32>,
    #[nom(Count = "6")]
    bitmap_crc32s: Vec<u32>,
}

#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
struct MaterialZAltAlt {
    color: Vec4f,
    emission: Vec3f,
    unknown0: i32,
    #[nom(Count = "31")]
    vertex_shader_constant_fs: Vec<f32>,
    opt: u8,
    #[nom(Count = "6")]
    bitmap_crc32s: Vec<u32>,
}

#[derive(Serialize, Deserialize)]
struct MaterialObject {
    resource_object: ResourceObjectZ,
    material: MaterialZ,
}

#[derive(Serialize, Deserialize)]
struct MaterialObjectAlt {
    resource_object: ResourceObjectZ,
    material: MaterialZAlt,
}

#[derive(Serialize, Deserialize)]
struct MaterialObjectAltAlt {
    resource_object: ResourceObjectZ,
    material: MaterialZAltAlt,
}

pub fn fuel_fmt_extract_material_z(header: &[u8], data: &[u8], output_path: &Path) -> Result<()> {
    let json_path = output_path.join("object.json");
    let mut output_file = File::create(json_path)?;

    let resource_object = match ResourceObjectZ::parse(&header) {
        Ok((_, h)) => h,
        Err(error) => panic!("{}", error),
    };

    if data.len() == 172 {
        let material = match MaterialZ::parse(&data) {
            Ok((_, h)) => h,
            Err(error) => panic!("{}", error),
        };

        let object = MaterialObject {
            resource_object,
            material,
        };

        output_file.write(serde_json::to_string_pretty(&object)?.as_bytes())?;
    } else if data.len() == 177 {
        let material = match MaterialZAlt::parse(&data) {
            Ok((_, h)) => h,
            Err(error) => panic!("{}", error),
        };

        let object = MaterialObjectAlt {
            resource_object,
            material,
        };

        output_file.write(serde_json::to_string_pretty(&object)?.as_bytes())?;
    } else if data.len() == 181 {
        let material = match MaterialZAltAlt::parse(&data) {
            Ok((_, h)) => h,
            Err(error) => panic!("{}", error),
        };

        let object = MaterialObjectAltAlt {
            resource_object,
            material,
        };

        output_file.write(serde_json::to_string_pretty(&object)?.as_bytes())?;
    } else {
        panic!("bad data length");
    }

    Ok(())
}
