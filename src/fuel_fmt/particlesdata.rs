use std::io::Result;
use std::io::Write;
use std::path::Path;

use nom_derive::{NomLE, Parse};
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::{PascalArray, ResourceObjectZ};
use crate::File;

#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
struct ParticlesDataZ {
    equals257: u32,
    position_x: f32,
    position_y: f32,
    position_z: f32,
    velocity_x: f32,
    velocity_y: f32,
    velocity_z: f32,
    shorts: PascalArray<u16>,
    zero: u32,
}

#[derive(Serialize, Deserialize)]
struct ParticlesDataObject {
    resource_object: ResourceObjectZ,
    particles_data: ParticlesDataZ,
}

pub fn fuel_fmt_extract_particles_data_z(
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

    let particles_data = match ParticlesDataZ::parse(&data) {
        Ok((_, h)) => h,
        Err(error) => panic!("{}", error),
    };

    let object = ParticlesDataObject {
        resource_object,
        particles_data,
    };

    output_file.write(serde_json::to_string_pretty(&object)?.as_bytes())?;

    Ok(())
}
