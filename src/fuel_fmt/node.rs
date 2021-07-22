use nom_derive::NomLE;
use serde::{Deserialize, Serialize};
use binwrite::BinWrite;

use crate::fuel_fmt::common::{FixedVec, FUELObjectFormat, Mat4f, ResourceObjectZ};

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct NodeZ {
    parent_crc32: u32,
    some_node_crc320: u32,
    some_node_crc321: u32,
    some_node_crc322: u32,
    some_crc320: u32,
    some_crc321: u32,
    unknown6: u32,
    unknown7: u32,
    unknown8: u32,
    unknown9: f32,
    unknown10s: FixedVec<u8, 32>,
    mat0: Mat4f,
    unknown11s: FixedVec<u16, 17>,
    mat1: Mat4f,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct NodeZAlt {
    parent_crc32: u32,
    some_node_crc320: u32,
    some_node_crc321: u32,
    some_node_crc322: u32,
    some_crc320: u32,
    some_crc321: u32,
    some_crc322: u32,
    some_crc323: u32,
    some_crc324: u32,
    mat0: Mat4f,
    unknown0s: FixedVec<u8, 208>,
    mat1: Mat4f,
    unknown2: u32,
    unknown3: u32,
    unknown4: u16,
    unknown5: u32,
    unknown6: u32,
}

pub type NodeObjectFormat = FUELObjectFormat<ResourceObjectZ, NodeZ>;
pub type NodeObjectFormatAlt = FUELObjectFormat<ResourceObjectZ, NodeZAlt>;
