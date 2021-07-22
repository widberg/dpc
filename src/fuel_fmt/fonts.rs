use nom_derive::NomLE;
use serde::{Deserialize, Serialize};
use binwrite::BinWrite;

use crate::fuel_fmt::common::{FUELObjectFormat, PascalArray, ResourceObjectZ};

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

pub type FontsObjectFormat = FUELObjectFormat<ResourceObjectZ, FontsZ>;
