use nom_derive::NomLE;
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::{FUELObjectFormat, FixedVec, ObjectZ, PascalArray, Vec3f};

#[derive(Serialize, Deserialize, NomLE)]
struct SplineZUnknown1 {
    data: FixedVec<u8, 240>,
}

#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct SplineZ {
    unknown0s: PascalArray<Vec3f>,
    unknown1s: PascalArray<SplineZUnknown1>,
    unknown2: f32,
    unknown3: f32,
    unknown4: f32,
    unknown5: f32,
    unknown6: f32,
}

pub type SplineObjectFormat = FUELObjectFormat<ObjectZ, SplineZ>;
