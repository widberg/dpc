use binwrite::BinWrite;
use nom_derive::NomLE;
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::{FUELObjectFormat, PascalArray, ResourceObjectZ};

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct ParticlesDataZ {
    equals257: u32,
    position_x: f32,
    position_y: f32,
    position_z: f32,
    velocity_x: f32,
    velocity_y: f32,
    velocity_z: f32,
    shorts: PascalArray<u16>,
    zero: u32,
}

pub type ParticlesDataObjectFormat = FUELObjectFormat<ResourceObjectZ, ParticlesDataZ>;
