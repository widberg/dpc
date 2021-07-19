use std::io::Result;
use std::io::Write;
use std::path::Path;

use nom::number::complete::*;
use nom_derive::{NomLE, Parse};
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::{ObjectZ, Vec3f};
use crate::File;

#[derive(Serialize, Deserialize, NomLE)]
struct SplineGraphZUnknown {
    #[nom(Count(60))]
    data: Vec<u8>,
}

#[derive(Serialize, Deserialize, NomLE)]
struct SplineGraphZUnknown1 {
    #[nom(Count(4))]
    unknowns: Vec<SplineGraphZUnknown>,
}

#[derive(Serialize, Deserialize, NomLE)]
struct PascalArrayu8 {
    #[nom(LengthCount(le_u32))]
    data: Vec<u8>,
}

#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
struct SplineGraphZ {
    #[nom(LengthCount(le_u32))]
    unknown0s: Vec<Vec3f>,
    #[nom(LengthCount(le_u32))]
    unknown1s: Vec<SplineGraphZUnknown1>,
    unknown2: f32,
    unknown3: f32,
    unknown4: f32,
    unknown5: f32,
    unknown6: f32,
    #[nom(LengthCount(le_u32))]
    unknown7s: Vec<u32>,
    #[nom(LengthCount(le_u32))]
    unknown8s: Vec<PascalArrayu8>,
    #[nom(LengthCount(le_u32))]
    unknown9s: Vec<PascalArrayu8>,
}

#[derive(Serialize, Deserialize)]
struct SplineGraphObject {
    object: ObjectZ,
    spline_graph: SplineGraphZ,
}

pub fn fuel_fmt_extract_spline_graph_z(
    header: &[u8],
    data: &[u8],
    output_path: &Path,
) -> Result<()> {
    let json_path = output_path.join("object.json");
    let mut output_file = File::create(json_path)?;

    let object = match ObjectZ::parse(&header) {
        Ok((_, h)) => h,
        Err(error) => panic!("{}", error),
    };

    let spline_graph = match SplineGraphZ::parse(&data) {
        Ok((_, h)) => h,
        Err(error) => panic!("{}", error),
    };

    let object = SplineGraphObject {
        object,
        spline_graph,
    };

    output_file.write(serde_json::to_string_pretty(&object)?.as_bytes())?;

    Ok(())
}
