use binwrite::BinWrite;
use nom_derive::NomLE;
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::{
    FUELObjectFormat, HasReferences, Mat4f, PascalArray, ResourceObjectZ,
};

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct WorldZUnknown2 {
    placeholder0: u32,
    placeholder1: u32,
    index: u32,
    placeholder2: u32,
    unknown4: u32,
    zero: u32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct WorldZ {
    node_crc32: u32,
    warp_crc32: u32,
    game_obj_crc32: u32,
    unused14: u32,
    gen_world_crc32: u32,
    node_crc321: u32,
    unused17s: PascalArray<u32>,
    unuseds: PascalArray<u8>,
    unknown0: Mat4f,
    indices0: PascalArray<u32>,
    unknown2s: PascalArray<WorldZUnknown2>,
    unknown3: Mat4f,
    indices1: PascalArray<u32>,
    unknown5s: PascalArray<WorldZUnknown2>,
    unused6s: PascalArray<u32>,
    unused7s: PascalArray<u32>,
    unused8s: PascalArray<u32>,
    unused9s: PascalArray<u32>,
    unused10s: PascalArray<u32>,
    spline_graph_crc32: PascalArray<u32>,
    unused12s: PascalArray<u32>,
    material_anim_crc32: PascalArray<u32>,
}

impl HasReferences for WorldZ {
    fn hard_links(&self) -> Vec<u32> {
        vec![]
    }

    fn soft_links(&self) -> Vec<u32> {
        vec![]
    }
}

pub type WorldObjectFormat = FUELObjectFormat<ResourceObjectZ, WorldZ>;
