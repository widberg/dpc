use nom_derive::NomLE;
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::{FUELObjectFormat, Mat4f, ObjectZ, PascalArray};

#[derive(Serialize, Deserialize, NomLE)]
struct WorldRefZUnknown7 {
    unknown0: u32,
    unknown1: u32,
}

#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct WorldRefZ {
    unknown11: u32,
    unknown12: u32,
    unknown13: u32,
    unknown14: u32,
    unknown15: u32,
    unknown16: u32,
    unknown17s: PascalArray<u32>,
    unknowns: PascalArray<u8>,
    mats: PascalArray<Mat4f>,
    unknown1: u32,
    unknown2: u32,
    unknown3: u32,
    unknown4: u32,
    unknown5: u32,
    unknown6: u32,
    unknown7s: PascalArray<WorldRefZUnknown7>,
    unknown8s: PascalArray<u8>,
    unknown9s: PascalArray<u32>,
    zero: u32,
}

pub type WorldRefObjectFormat = FUELObjectFormat<ObjectZ, WorldRefZ>;
