use binwrite::BinWrite;
use nom_derive::NomLE;
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::{FUELObjectFormat, HasReferences, PascalArray, ResourceObjectZ};

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct FontsZCharacter {
    id: u32,
    material_index: u32,
    point: f32,
    height: f32,
    y: f32,
    x: f32,
    width: f32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct FontsZ {
    characters: PascalArray<FontsZCharacter>,
    material_crc32s: PascalArray<u32>,
}

impl HasReferences for FontsZ {
    fn hard_links(&self) -> Vec<u32> {
        let mut v = Vec::new();
        v.append(&mut self.material_crc32s.data.clone());
        v
    }

    fn soft_links(&self) -> Vec<u32> {
        vec![]
    }
}

pub type FontsObjectFormat = FUELObjectFormat<ResourceObjectZ, FontsZ>;
