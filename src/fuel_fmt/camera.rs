use binwrite::BinWrite;
use nom_derive::NomLE;
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::{FUELObjectFormat, ObjectZ};

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct CameraZ {
    angle_of_view: f32,
    zero: f32,
    node_crc32: u32,
}

pub type CameraObjectFormat = FUELObjectFormat<ObjectZ, CameraZ>;
