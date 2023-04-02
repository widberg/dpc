use binwrite::BinWrite;
use nom_derive::NomLE;
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::{FUELObjectFormat, FixedVec, HasReferences, Mat4f, ResourceObjectZ, Vec3f, Quat, SphereZ, Color, Rect};

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct NodeZ {
    parent_crc32: u32,
    head_child_crc32: u32,
    prev_node_crc32: u32,
    next_node_crc32: u32,
    lod_crc32: u32,
    lod_data_crc32: u32,
    user_define_crc32: u32,
    unknown7: u32,
    unknown8: u32,
    unknown9: u32,
    rotation: Quat,
    translation: Vec3f,
    flags: u32,
    rotation2: Quat,
    scale: f32,
    scale2: f32,
    reciprocal_scale: f32,
    unknown10: f32,
    color: Color,
    sphere: SphereZ,
    display_seeds_rect: Rect,
    collide_seeds_rect: Rect,
    negative_four: i16,
    world_transform: Mat4f,
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

impl HasReferences for NodeZ {
    fn hard_links(&self) -> Vec<u32> {
        vec![]
    }

    fn soft_links(&self) -> Vec<u32> {
        let mut v = Vec::new();
        if self.parent_crc32 != 0 { v.push(self.parent_crc32) }
        if self.head_child_crc32 != 0 { v.push(self.head_child_crc32) }
        if self.prev_node_crc32 != 0 { v.push(self.prev_node_crc32) }
        if self.next_node_crc32 != 0 { v.push(self.next_node_crc32) }
        if self.lod_crc32 != 0 { v.push(self.lod_crc32) }
        if self.lod_data_crc32 != 0 { v.push(self.lod_data_crc32) }
        if self.user_define_crc32 != 0 { v.push(self.user_define_crc32) }
        if self.unknown7 != 0 { v.push(self.unknown7) }
        if self.unknown8 != 0 { v.push(self.unknown8) }
        if self.unknown9 != 0 { v.push(self.unknown9) }
        v
    }
}

impl HasReferences for NodeZAlt {
    fn hard_links(&self) -> Vec<u32> {
        vec![]
    }

    fn soft_links(&self) -> Vec<u32> {
        vec![]
    }
}

pub type NodeObjectFormat = FUELObjectFormat<ResourceObjectZ, NodeZ>;
pub type NodeObjectFormatAlt = FUELObjectFormat<ResourceObjectZ, NodeZAlt>;
