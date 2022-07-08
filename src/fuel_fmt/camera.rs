use binwrite::BinWrite;
use nom_derive::NomLE;
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::{FUELObjectFormat, HasReferences, ObjectZ};

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct CameraZ {
    angle_of_view: f32,
    zero: f32,
    node_crc32: u32,
}

impl HasReferences for CameraZ {
    fn hard_links(&self) -> Vec<u32> {
        vec![]
    }

    fn soft_links(&self) -> Vec<u32> {
        vec![]
    }
}

pub type CameraObjectFormat = FUELObjectFormat<ObjectZ, CameraZ>;
