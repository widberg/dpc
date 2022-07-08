use binwrite::BinWrite;
use nom_derive::NomLE;
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::{FUELObjectFormat, HasReferences, ResourceObjectZ, Vec2f, Vec3f};

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct WarpZ {
    material_crc32: u32,
    #[nom(Count(8))]
    vertices: Vec<Vec3f>,
    u24: f32,
    u25: f32,
    radius: f32,
    #[nom(Count(4))]
    texcoords: Vec<Vec2f>,
}

impl HasReferences for WarpZ {
    fn hard_links(&self) -> Vec<u32> {
        vec![]
    }

    fn soft_links(&self) -> Vec<u32> {
        vec![]
    }
}

pub type WarpObjectFormat = FUELObjectFormat<ResourceObjectZ, WarpZ>;
