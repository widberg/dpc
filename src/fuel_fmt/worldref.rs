use binwrite::BinWrite;
use nom_derive::NomLE;
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::{FUELObjectFormat, Mat4f, ObjectZ, PascalArray, Vec3f, PascalStringNULL};

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

pub type WorldRefObjectFormat = FUELObjectFormat<ObjectZ, WorldRefZ>;
