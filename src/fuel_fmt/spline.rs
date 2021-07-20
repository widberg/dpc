use std::io::Result;
use std::io::Write;
use std::path::Path;

use nom_derive::{NomLE, Parse};
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::{FixedVec, ObjectZ, PascalArray, Vec3f};
use crate::File;

#[derive(Serialize, Deserialize, NomLE)]
struct SplineZUnknown1 {
    data: FixedVec<u8, 240>,
}

#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
struct SplineZ {
    unknown0s: PascalArray<Vec3f>,
    unknown1s: PascalArray<SplineZUnknown1>,
    unknown2: f32,
    unknown3: f32,
    unknown4: f32,
    unknown5: f32,
    unknown6: f32,
}

#[derive(Serialize, Deserialize)]
struct SplineObject {
    object: ObjectZ,
    spline: SplineZ,
}

pub fn fuel_fmt_extract_spline_z(header: &[u8], data: &[u8], output_path: &Path) -> Result<()> {
    let json_path = output_path.join("object.json");
    let mut output_file = File::create(json_path)?;

    let object = match ObjectZ::parse(&header) {
        Ok((_, h)) => h,
        Err(error) => panic!("{}", error),
    };

    let spline = match SplineZ::parse(&data) {
        Ok((_, h)) => h,
        Err(error) => panic!("{}", error),
    };

    let object = SplineObject { object, spline };

    output_file.write(serde_json::to_string_pretty(&object)?.as_bytes())?;

    Ok(())
}
