use std::io::Result;
use std::io::Write;
use std::path::Path;

use nom::number::complete::*;
use nom_derive::{NomLE, Parse};
use serde::{Deserialize, Serialize};

use crate::File;
use crate::fuel_fmt::common::{Mat4f, ObjectZ};

#[derive(Serialize, Deserialize, NomLE)]
struct ParticlesZUnknown1 {
    unknown0: u32,
    unknown1: u32,
    unknown2: u32,
}

#[derive(Serialize, Deserialize, NomLE)]
struct ParticlesZUnknown2
{
    unknown0: u32,
    unknown1: u32,
    unknown2: u32,
    unknown3: u32,
    unknown4: u32,
}

#[derive(Serialize, Deserialize, NomLE)]
struct ParticlesZUnknown4
{
    unknown0: u32,
    unknown1: u32,
}

#[derive(Serialize, Deserialize, NomLE)]
struct ParticlesZUnknown5
{
    unknown0: u32,
    unknown1: u32,
    unknown2: u32,
    unknown3: u32,
}

#[derive(Serialize, Deserialize, NomLE)]
struct ParticlesZUnknown0 {
    #[nom(Count(19))]
    data: Vec<u32>,
    unknown1flag: u16,
    #[nom(LengthCount(le_u32))]
    unknown1s: Vec<ParticlesZUnknown1>,

    unknown2flag: u16,
    #[nom(LengthCount(le_u32))]
    unknown2s: Vec<ParticlesZUnknown2>,

    unknown3flag: u16,
    #[nom(LengthCount(le_u32))]
    unknown3s: Vec<ParticlesZUnknown2>,

    unknown4flag: u16,
    #[nom(LengthCount(le_u32))]
    unknown4s: Vec<ParticlesZUnknown4>,

    unknown5flag: u16,
    #[nom(LengthCount(le_u32))]
    unknown5s: Vec<ParticlesZUnknown5>,

    unknown6flag: u16,
    #[nom(LengthCount(le_u32))]
    unknown6s: Vec<ParticlesZUnknown5>,
    unknown7flag: u16,
    #[nom(LengthCount(le_u32))]
    unknown7s: Vec<ParticlesZUnknown4>,
    unknown8: u32,
}

#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
struct ParticlesZ {
    #[nom(LengthCount(le_u32))]
    unknown0s: Vec<ParticlesZUnknown0>,
    #[nom(LengthCount(le_u32))]
    mats: Vec<Mat4f>,
    unknown2: u32,
    unknown3: u16,
}

#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
struct ParticlesZAlt {
    #[nom(Count(i.len()))]
    data: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
struct ParticlesObject {
    object: ObjectZ,
    particles: ParticlesZ,
}

#[derive(Serialize, Deserialize)]
struct ParticlesObjectAlt {
    object: ObjectZ,
    particles: ParticlesZAlt,
}

pub fn fuel_fmt_extract_particles_z(header: &[u8], data: &[u8], output_path: &Path) -> Result<()> {
    let json_path = output_path.join("object.json");
    let mut output_file = File::create(json_path)?;

    let object = match ObjectZ::parse(&header) {
        Ok((_, h)) => h,
        Err(error) => panic!("{}", error),
    };

    let particles = match ParticlesZ::parse(&data) {
        Ok((_, h)) => h,
        Err(_) => match ParticlesZAlt::parse(&data) {
            Ok((_, particles)) => {
                let object = ParticlesObjectAlt {
                    object,
                    particles,
                };

                output_file.write(serde_json::to_string_pretty(&object)?.as_bytes())?;

                return Ok(());
            },
            Err(error) => panic!("{}", error),
        },
    };

    let object = ParticlesObject {
        object,
        particles,
    };

    output_file.write(serde_json::to_string_pretty(&object)?.as_bytes())?;

    Ok(())
}
