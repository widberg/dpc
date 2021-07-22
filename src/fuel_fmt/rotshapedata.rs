use nom_derive::NomLE;
use serde::{Deserialize, Serialize};
use binwrite::BinWrite;

use crate::fuel_fmt::common::{FUELObjectFormat, PascalArray, ResourceObjectZ};

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

pub type RotShapeDataObjectFormat = FUELObjectFormat<ResourceObjectZ, RotShapeDataZ>;
