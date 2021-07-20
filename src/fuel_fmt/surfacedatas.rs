use std::io::Result;
use std::io::Write;
use std::path::Path;

use nom_derive::{NomLE, Parse};
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::ResourceObjectZ;
use crate::File;

#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
struct SurfaceDatasZ {
    one: u32,
}

#[derive(Serialize, Deserialize)]
struct SurfaceDatasObject {
    resource_object: ResourceObjectZ,
    surface_datas: SurfaceDatasZ,
}

pub fn fuel_fmt_extract_surface_datas_z(
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

    let surface_datas = match SurfaceDatasZ::parse(&data) {
        Ok((_, h)) => h,
        Err(error) => panic!("{}", error),
    };

    let object = SurfaceDatasObject {
        resource_object,
        surface_datas,
    };

    output_file.write(serde_json::to_string_pretty(&object)?.as_bytes())?;

    Ok(())
}
