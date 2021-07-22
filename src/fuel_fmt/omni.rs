use binwrite::BinWrite;
use nom_derive::NomLE;
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::{FUELObjectFormat, FixedVec, ObjectZ};

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct OmniZ {
    data: FixedVec<u32, 48>,
    crc32s: FixedVec<u32, 2>,
}

pub type OmniObjectFormat = FUELObjectFormat<ObjectZ, OmniZ>;
