use binwrite::BinWrite;
use nom_derive::NomLE;
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::{
    FUELObjectFormat, HasReferences, Mat4f, ObjectZ, PascalArray, PascalStringNULL, Vec3f,
};

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct UUIDPair {
    uuid0: u32,
    uuid1: u32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct WorldRefZ {
    node_crc32: u32,
    warp_crc32: u32,
    game_obj_crc32: u32,
    unused14: u32,
    gen_world_crc32: u32,
    node_crc321: u32,
    unused17s: PascalArray<u32>,
    unuseds: PascalArray<u8>,
    mats: PascalArray<Mat4f>,
    point_a: Vec3f,
    point_b: Vec3f,
    uuid_pairs: PascalArray<UUIDPair>,
    init_script: PascalStringNULL,
    node_crc32s: PascalArray<u32>,
    zero: u32,
}

impl HasReferences for WorldRefZ {
    fn hard_links(&self) -> Vec<u32> {
        vec![]
    }

    fn soft_links(&self) -> Vec<u32> {
        let mut v = Vec::new();
        if self.node_crc32 != 0 { v.push(self.node_crc32) }
        if self.warp_crc32 != 0 { v.push(self.warp_crc32) }
        if self.game_obj_crc32 != 0 { v.push(self.game_obj_crc32) }
        if self.unused14 != 0 { v.push(self.unused14) }
        if self.gen_world_crc32 != 0 { v.push(self.gen_world_crc32) }
        if self.node_crc321 != 0 { v.push(self.node_crc321) }
        v.append(&mut self.node_crc32s.data.clone());
        v
    }
}

pub type WorldRefObjectFormat = FUELObjectFormat<ObjectZ, WorldRefZ>;
