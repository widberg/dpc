use binwrite::BinWrite;
use nom_derive::NomLE;
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::{
    FUELObjectFormat, HasReferences, Mat4f, PascalArray, Quat, ResourceObjectZ, Vec3f, Vec3i32,
};

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct SkelZBone {
    user_define_crc32: u32,
    quat: Quat,
    vec0: Vec3f,
    bone_flags: u32,
    vec1: Vec3f,
    child_bone_begin: u32,
    vec2: Vec3f,
    some_mat_pro0: u32,
    vec3: Vec3f,
    some_mat_pro1: u32,
    vec4: Vec3f,
    some_mat_pro2: u32,
    quat1: Quat,
    vec5: Vec3i32,
    parent_bone_ptr: u32,
    vec6: Vec3i32,
    some_bone_ptr: u32,
    vec7: Vec3i32,
    child_bone_ptr: u32,
    transformation: Mat4f,
    parent_index: i32,
    child_bones_index0: i32,
    child_bones_index1: i32,
    some_bone_index: i32,
    bone_name: u32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct SkelZUnknown4 {
    unknown0: u32,
    unknown1: u32,
    unknown2: u32,
    unknown3: u32,
    unknown4: u32,
    unknown5: u32,
    unknown6: u32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct SkelZUnknown2 {
    mat: Mat4f,
    unknown0: u32,
    unknown1: u32,
    unknown2: u32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct SkelZ {
    u0: u32,
    u1: f32,
    u2: f32,
    u3: f32,
    u4: f32,
    bones: PascalArray<SkelZBone>,
    material_crc32s: PascalArray<u32>,
    mesh_data_crc32s: PascalArray<u32>,
    unknown5s: PascalArray<PascalArray<u32>>,
    unknown3: PascalArray<u32>,
    unknown4s: PascalArray<SkelZUnknown4>,
    unknown1s: PascalArray<SkelZUnknown4>,
    unknown2s: PascalArray<SkelZUnknown2>,
}

impl HasReferences for SkelZ {
    fn hard_links(&self) -> Vec<u32> {
        vec![]
    }

    fn soft_links(&self) -> Vec<u32> {
        vec![]
    }
}

pub type SkelObjectFormat = FUELObjectFormat<ResourceObjectZ, SkelZ>;
