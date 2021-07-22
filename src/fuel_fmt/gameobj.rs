use nom::*;
use nom::number::complete::*;
use nom_derive::NomLE;
use serde::{Deserialize, Serialize};
use binwrite::BinWrite;

use crate::fuel_fmt::common::{FUELObjectFormat, PascalArray, ResourceObjectZ};

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct GameObjZChild {
    #[nom(
        Map = "|x: Vec<u8>| String::from_utf8_lossy(&x[0..x.len() - 1]).to_string()",
        Parse = "|i| length_count!(i, le_u32, le_u8)"
    )]
    string: String,
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
