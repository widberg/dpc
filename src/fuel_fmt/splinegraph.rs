use binwrite::BinWrite;
use nom_derive::NomLE;
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::{
    FUELObjectFormat, FixedVec, HasReferences, ObjectZ, PascalArray, Vec3f,
};

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct SplineGraphZUnknown {
    data: FixedVec<u8, 60>,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct SplineGraphZUnknown1 {
    unknowns: FixedVec<SplineGraphZUnknown, 4>,
}

#[derive(BinWrite)]
#[binwrite(little)]
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

impl HasReferences for SplineGraphZ {
    fn hard_links(&self) -> Vec<u32> {
        vec![]
    }

    fn soft_links(&self) -> Vec<u32> {
        vec![]
    }
}

pub type SplineGraphObjectFormat = FUELObjectFormat<ObjectZ, SplineGraphZ>;
