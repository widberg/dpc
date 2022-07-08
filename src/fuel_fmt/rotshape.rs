use binwrite::BinWrite;
use nom_derive::NomLE;
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::{
    FUELObjectFormat, HasReferences, ObjectZ, PascalArray, Vec2f, Vec3f,
};

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct RotShapeZ {
    vertices: PascalArray<Vec3f>,
    unknown1: f32,
    ints: PascalArray<u32>,
    sizes: PascalArray<Vec3f>,
    texcoords: PascalArray<Vec2f>,
    material_crc32s: PascalArray<u32>,
    scale: f32,
    billboard_mode: u16,
}

impl HasReferences for RotShapeZ {
    fn hard_links(&self) -> Vec<u32> {
        vec![]
    }

    fn soft_links(&self) -> Vec<u32> {
        vec![]
    }
}

pub type RotShapeObjectFormat = FUELObjectFormat<ObjectZ, RotShapeZ>;
