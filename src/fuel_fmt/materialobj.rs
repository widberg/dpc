use binwrite::BinWrite;
use nom_derive::NomLE;
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::{FUELObjectFormat, PascalArray, ResourceObjectZ};

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

pub type MaterialObjObjectFormat = FUELObjectFormat<ResourceObjectZ, MaterialObjZ>;
