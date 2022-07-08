use binwrite::BinWrite;
use nom_derive::NomLE;
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::{FUELObjectFormat, HasReferences, PascalArray, ResourceObjectZ};

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct RotShapeDataZ {
    one: u32,
    shorts: PascalArray<u16>,
    #[nom(Map = "|x: &[u8]| x.to_vec()", Take = "shorts.len() * 28")]
    padding: Vec<u8>,
}

impl HasReferences for RotShapeDataZ {
    fn hard_links(&self) -> Vec<u32> {
        vec![]
    }

    fn soft_links(&self) -> Vec<u32> {
        vec![]
    }
}

pub type RotShapeDataObjectFormat = FUELObjectFormat<ResourceObjectZ, RotShapeDataZ>;
