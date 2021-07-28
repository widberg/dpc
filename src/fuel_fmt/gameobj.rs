use binwrite::BinWrite;
use nom_derive::NomLE;
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::{FUELObjectFormat, PascalArray, PascalStringNULL, ResourceObjectZ};

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct GameObjZChild {
    string: PascalStringNULL,
    is_in_world: u32,
    crc32s: PascalArray<u32>,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct GameObjZ {
    children: PascalArray<GameObjZChild>,
}

pub type GameObjObjectFormat = FUELObjectFormat<ResourceObjectZ, GameObjZ>;
