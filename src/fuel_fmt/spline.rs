use binwrite::BinWrite;
use nom_derive::NomLE;
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::{
    FUELObjectFormat, FixedVec, HasReferences, ObjectZ, PascalArray, Vec3f,
};

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct SplineZSubsection {
    point1: Vec3f,
    point2: Vec3f,
    length: f32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct SplineZSection {
    p1: u16,
    p2: u16,
    p1_t: u16,
    p2_t: u16,
    unknown0: u32,
    length: f32,
    spline_subsections: FixedVec<SplineZSubsection, 8>,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct SplineZ {
    vertices: PascalArray<Vec3f>,
    spline_sections: PascalArray<SplineZSection>,
    unknown2: f32,
    unknown3: f32,
    unknown4: f32,
    unknown5: f32,
    length: f32,
}

impl HasReferences for SplineZ {
    fn hard_links(&self) -> Vec<u32> {
        vec![]
    }

    fn soft_links(&self) -> Vec<u32> {
        vec![]
    }
}

pub type SplineObjectFormat = FUELObjectFormat<ObjectZ, SplineZ>;
