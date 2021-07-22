use nom_derive::NomLE;
use serde::{Deserialize, Serialize};
use binwrite::BinWrite;

use crate::fuel_fmt::common::{FUELObjectFormat, ResourceObjectZ};

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct SurfaceDatasZ {
    one: u32,
}

pub type SurfaceDatasObjectFormat = FUELObjectFormat<ResourceObjectZ, SurfaceDatasZ>;
