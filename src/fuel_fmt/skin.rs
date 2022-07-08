use binwrite::BinWrite;
use nom_derive::NomLE;
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::{FUELObjectFormat, HasReferences, ObjectZ, PascalArray};

static mut SKIN_DATA_COUNT: u32 = 0;

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct SkinZSkinSubsection {
    vertex_group_crc32: u32,
    unknown_crc320: u32,
    unknown_crc321: u32,
    unknown_crc322: u32,
    #[nom(Count(unsafe { SKIN_DATA_COUNT as usize }))]
    data: Vec<u32>,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct SkinZ {
    mesh_crc32s: PascalArray<u32>,
    u0: u32,
    u1: u32,
    u2: u32,
    u3: u32,
    #[nom(Verify(*one_and_a_half == 1.5))]
    one_and_a_half: f32,
    #[nom(PostExec(unsafe { SKIN_DATA_COUNT = data_count }))]
    #[nom(Verify(*data_count == 21))]
    data_count: u32,
    skin_sections: PascalArray<PascalArray<SkinZSkinSubsection>>,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct SkinZAlt {
    mesh_crc32s: PascalArray<u32>,
    u0: u32,
    u1: u32,
    u2: u8,
    one_and_a_half: f32,
    #[nom(PostExec(unsafe { SKIN_DATA_COUNT = data_count }))]
    data_count: u32,
    skin_sections: PascalArray<PascalArray<SkinZSkinSubsection>>,
}

impl HasReferences for SkinZ {
    fn hard_links(&self) -> Vec<u32> {
        vec![]
    }

    fn soft_links(&self) -> Vec<u32> {
        vec![]
    }
}

impl HasReferences for SkinZAlt {
    fn hard_links(&self) -> Vec<u32> {
        vec![]
    }

    fn soft_links(&self) -> Vec<u32> {
        vec![]
    }
}

pub type SkinObjectFormat = FUELObjectFormat<ObjectZ, SkinZ>;
pub type SkinObjectFormatAlt = FUELObjectFormat<ObjectZ, SkinZAlt>;
