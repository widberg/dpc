use binwrite::BinWrite;
use nom_derive::NomLE;
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::{
    FUELObjectFormat, FixedVec, HasReferences, Mat4f, ObjectZ, PascalArray, PascalStringNULL, Quat,
    Vec2f, Vec3f,
};

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
    zero: u32,
    mat: Mat4f,
    quat: Quat,
    vec: Vec3f,
    unknown1: f32,
    unknown3: i32,
    unknown5: i32,
    unknown6: i32,
    unknown7: i32,
    unknown8: i32,
    unknown9: i32,
    unknown4: i16,
    unknown10: i32,
    unknown2: i8,
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

impl HasReferences for GenWorldZ {
    fn hard_links(&self) -> Vec<u32> {
        vec![]
    }

    fn soft_links(&self) -> Vec<u32> {
        vec![]
    }
}

pub type GenWorldObjectFormat = FUELObjectFormat<ObjectZ, GenWorldZ>;
