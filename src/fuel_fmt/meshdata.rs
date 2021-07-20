use std::io::Result;
use std::io::Write;
use std::path::Path;

use nom_derive::{NomLE, Parse};
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::ResourceObjectZ;
use crate::File;

#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
struct MeshDataZ {
    not_traffic_tm_or_p_moto: u32,
    zero0: u32,
    zero1: u32,
    zero2: u32,
    zero3: u32,
}

#[derive(Serialize, Deserialize)]
struct MeshDataObject {
    resource_object: ResourceObjectZ,
    mesh_data: MeshDataZ,
}

pub fn fuel_fmt_extract_mesh_data_z(header: &[u8], data: &[u8], output_path: &Path) -> Result<()> {
    let json_path = output_path.join("object.json");
    let mut output_file = File::create(json_path)?;

    let resource_object = match ResourceObjectZ::parse(&header) {
        Ok((_, h)) => h,
        Err(error) => panic!("{}", error),
    };

    let mesh_data = match MeshDataZ::parse(&data) {
        Ok((_, h)) => h,
        Err(error) => panic!("{}", error),
    };

    let object = MeshDataObject {
        resource_object,
        mesh_data,
    };

    output_file.write(serde_json::to_string_pretty(&object)?.as_bytes())?;

    Ok(())
}
