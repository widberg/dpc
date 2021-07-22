use nom_derive::NomLE;
use serde::{Deserialize, Serialize};
use binwrite::BinWrite;

use crate::fuel_fmt::common::{FUELObjectFormat, ResourceObjectZ, Vec3f, Vec3i32};

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct LightDataZ {
    unknown0: u32,
    color: Vec3f,
    unknown1: Vec3f,
    unknown2: Vec3i32,
    unknown_flag: u32,
    unknown3: Vec3f,
}

pub type LightDataObjectFormat = FUELObjectFormat<ResourceObjectZ, LightDataZ>;
