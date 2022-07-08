use binwrite::BinWrite;
use nom_derive::NomLE;
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::{FUELObjectFormat, FixedVec, HasReferences, ObjectZ};

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct OmniZ {
    data: FixedVec<u32, 48>,
    crc32s: FixedVec<u32, 2>,
}

impl HasReferences for OmniZ {
    fn hard_links(&self) -> Vec<u32> {
        vec![]
    }

    fn soft_links(&self) -> Vec<u32> {
        vec![]
    }
}

pub type OmniObjectFormat = FUELObjectFormat<ObjectZ, OmniZ>;
