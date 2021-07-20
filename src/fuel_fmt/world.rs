use nom_derive::NomLE;
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::{FUELObjectFormat, Mat4f, PascalArray, ResourceObjectZ};

#[derive(Serialize, Deserialize, NomLE)]
struct WorldZUnknown2 {
    unknown0: u32,
    unknown1: u32,
    unknown2: u32,
    unknown3: u32,
    unknown4: u32,
    unknown5: u32,
}

#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct WorldZ {
    unknown11: u32,
    unknown12: u32,
    unknown13: u32,
    unknown14: u32,
    unknown15: u32,
    unknown16: u32,
    unknown17s: PascalArray<u32>,
    unknowns: PascalArray<u8>,
    unknown0: Mat4f,
    unknown1s: PascalArray<u32>,
    unknown2s: PascalArray<WorldZUnknown2>,
    unknown3: Mat4f,
    unknown4s: PascalArray<u32>,
    unknown5s: PascalArray<WorldZUnknown2>,
    unknown6s: PascalArray<u32>,
    unknown7s: PascalArray<u32>,
    unknown8s: PascalArray<u32>,
    unknown9s: PascalArray<u32>,
    unknown10s: PascalArray<u32>,
    unknown11s: PascalArray<u32>,
    unknown12s: PascalArray<u32>,
    unknown13s: PascalArray<u32>,
}

pub type WorldObjectFormat = FUELObjectFormat<ResourceObjectZ, WorldZ>;
