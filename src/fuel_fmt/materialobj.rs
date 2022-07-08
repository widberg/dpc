use binwrite::BinWrite;
use nom_derive::NomLE;
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::{FUELObjectFormat, HasReferences, PascalArray, ResourceObjectZ};

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct MaterialObjZEntry {
    array_name_crc32: u32,
    material_anim_crc32s: PascalArray<u32>,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct MaterialObjZ {
    entries: PascalArray<MaterialObjZEntry>,
}

impl HasReferences for MaterialObjZ {
    fn hard_links(&self) -> Vec<u32> {
        vec![]
    }

    fn soft_links(&self) -> Vec<u32> {
        vec![]
    }
}

pub type MaterialObjObjectFormat = FUELObjectFormat<ResourceObjectZ, MaterialObjZ>;
