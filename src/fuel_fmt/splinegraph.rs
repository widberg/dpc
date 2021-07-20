use std::io::Result;
use std::io::Write;
use std::path::Path;

use nom_derive::{NomLE, Parse};
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::{FixedVec, ObjectZ, PascalArray, Vec3f};
use crate::File;

#[derive(Serialize, Deserialize, NomLE)]
struct SplineGraphZUnknown {
    data: FixedVec<u8, 60>,
}

#[derive(Serialize, Deserialize, NomLE)]
struct SplineGraphZUnknown1 {
    unknowns: FixedVec<SplineGraphZUnknown, 4>,
}

#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
struct SplineGraphZ {
    unknown0s: PascalArray<Vec3f>,
    unknown1s: PascalArray<SplineGraphZUnknown1>,
    unknown2: f32,
    unknown3: f32,
    unknown4: f32,
    unknown5: f32,
    unknown6: f32,
    unknown7s: PascalArray<u32>,
    unknown8s: PascalArray<PascalArray<u8>>,
    unknown9s: PascalArray<PascalArray<u8>>,
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
