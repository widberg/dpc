use binwrite::BinWrite;
use nom_derive::NomLE;
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::{FUELObjectFormat, HasReferences, ResourceObjectZ};

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct MeshDataZ {
    not_traffic_tm_or_p_moto: u32,
    zero0: u32,
    zero1: u32,
    zero2: u32,
    zero3: u32,
}

impl HasReferences for MeshDataZ {
    fn hard_links(&self) -> Vec<u32> {
        vec![]
    }

    fn soft_links(&self) -> Vec<u32> {
        vec![]
    }
}

pub type MeshDataObjectFormat = FUELObjectFormat<ResourceObjectZ, MeshDataZ>;
