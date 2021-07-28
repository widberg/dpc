use binwrite::BinWrite;
use nom_derive::NomLE;
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::{
    write_option, FUELObjectFormat, FixedVec, ResourceObjectZ, Vec3f, Vec4f,
};

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct MaterialZ {
    color: Vec4f,
    emission: Vec3f,
    unknown0: i32,
    vertex_shader_constant_fs: FixedVec<u32, 26>,
    diffuse_bitmap_crc32: u32,
    unknown_bitmap_crc320: u32,
    metal_bitmap_crc32: u32,
    unknown_bitmap_crc321: u32,
    grey_bitmap_crc32: u32,
    normal_bitmap_crc32: u32,
    dirt_bitmap_crc32: u32,
    unknown_bitmap_crc322: u32,
    unknown_bitmap_crc323: u32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct MaterialZAlt {
    color: Vec4f,
    emission: Vec3f,
    unknown0: i32,
    vertex_shader_constant_fs: FixedVec<u32, 28>,
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    #[allow(dead_code)]
    #[binwrite(ignore)]
    opt: u8,
    #[binwrite(postprocessor(|x: Vec<u8>| -> (u8, Vec<u8>) { if x.len() != 0 { (1u8, x) } else { (0u8, x) } }))]
    #[nom(Cond(opt != 0))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[binwrite(with(write_option))]
    unknown_crc320: Option<u32>,
    #[nom(Cond(opt != 0))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[binwrite(with(write_option))]
    unknown_crc321: Option<u32>,
    bitmap_crc32s: FixedVec<u32, 6>,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct MaterialZAltAlt {
    color: Vec4f,
    emission: Vec3f,
    unknown0: i32,
    vertex_shader_constant_fs: FixedVec<u32, 31>,
    #[binwrite(ignore)]
    opt: u8,
    #[binwrite(postprocessor(|x: Vec<u8>| -> (u8, Vec<u8>) { if x.len() != 0 { (1u8, x) } else { (0u8, x) } }))]
    #[nom(Cond(opt != 0))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[binwrite(with(write_option))]
    unknown_crc320: Option<u32>,
    #[nom(Cond(opt != 0))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[binwrite(with(write_option))]
    unknown_crc321: Option<u32>,
    bitmap_crc32s: FixedVec<u32, 6>,
}

pub type MaterialObjectFormat = FUELObjectFormat<ResourceObjectZ, MaterialZ>;
pub type MaterialObjectFormatAlt = FUELObjectFormat<ResourceObjectZ, MaterialZAlt>;
pub type MaterialObjectFormatAltAlt = FUELObjectFormat<ResourceObjectZ, MaterialZAltAlt>;
