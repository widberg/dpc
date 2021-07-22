use binwrite::BinWrite;
use nom_derive::NomLE;
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::{FUELObjectFormat, ResourceObjectZ, Vec2f, Vec3f};

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

pub type WarpObjectFormat = FUELObjectFormat<ResourceObjectZ, WarpZ>;
