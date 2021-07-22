use nom_derive::NomLE;
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::{FixedVec, FUELObjectFormat, ObjectZ, PascalArray, Vec3f};

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
pub struct SplineGraphZ {
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

pub type SplineGraphObjectFormat = FUELObjectFormat<ObjectZ, SplineGraphZ>;
