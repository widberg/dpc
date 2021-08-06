use binwrite::BinWrite;
use nom_derive::NomLE;
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::{FUELObjectFormat, FixedVec, Mat4f, ObjectZ, PascalArray, Vec2f, PascalStringNULL};

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct Category {
    name: PascalStringNULL,
    node_crc32s_arrays: PascalArray<PascalArray<u32>>,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct GenWorldZUnknown8 {
    unknown0: u32,
    data: FixedVec<u8, 127>,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct GenWorldZUnknown10 {
    unknown0: u32,
    unknown1s: FixedVec<u32, 8>,
    unknown2: u32,
    unknown3: u32,
    unknown4: u32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct CoordsLineSegment {
    coords_index_a: u32,
    coords_index_b: u32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct Region {
    name: FixedVec<u8, 32>,
    coords_line_segments_indices: PascalArray<u32>,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct GenWorldZ {
    node_crc32: u32,
    user_define_crc32: u32,
    gw_road_crc32: u32,
    binary_crc32s: PascalArray<u32>,
    bitmap_crc32s: PascalArray<u32>,
    material_crc32s: PascalArray<u32>,
    unknown6: u32,
    categories: PascalArray<Category>,
    unknown8s: PascalArray<GenWorldZUnknown8>,
    mats: PascalArray<Mat4f>,
    unknown10s: PascalArray<GenWorldZUnknown10>,
    coords: PascalArray<Vec2f>,
    coords_line_segments: PascalArray<CoordsLineSegment>,
    regions: PascalArray<Region>,
}

pub type GenWorldObjectFormat = FUELObjectFormat<ObjectZ, GenWorldZ>;
