use binwrite::BinWrite;
use nom_derive::NomLE;
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::{FUELObjectFormat, FixedVec, Mat4f, PascalArray, Quat, Vec3f, Vec4f};

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct MeshZUnknown0 {
    unknown0: u32,
    unknown1: u32,
    unknown2: u32,
    unknown3: u32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct MeshZUnknown1 {
    unknown0: u32,
    unknown1: u32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct MeshZUnknown2 {
    unknown0s: PascalArray<u16>,
    unknown1: u32,
    unknown2: u32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct MeshZUnknown4 {
    unknown0s: PascalArray<MeshZUnknown1>,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct MeshZUnknown6 {
    unknown0: u32,
    unknown1: u32,
    unknown2: u32,
    unknown3: u32,
    unknown4: u32,
    unknown5: u32,
    unknown6: u32,
    unknown7: u32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct MeshZUnknown7 {
    unknown0: u16,
    unknown1: u16,
    unknown2: u16,
    unknown3: u16,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct MeshZSubMesh {
    vertex_count: u32,
    vertex_size: u32,
    vertex_group_crc32: u32,
    #[nom(Count(vertex_count as usize * vertex_size as usize))]
    data: Vec<u8>,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct MeshZIndices {
    index_count: u32,
    vertex_group_crc32: u32,
    #[nom(Count(index_count))]
    data: Vec<u16>,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct MeshZUnknown11 {
    unknown0: u32,
    unknown1: u32,
    unknown2: u32,
    unknown3: u32,
    unknown4: u32,
    unknown5: u32,
    unknown6: u32,
    unknown7: u32,
    unknown8: u32,
    unknown9: u32,
    unknown10: u32,
    unknown11: u32,
    unknown12: u32,
    unknown13: u32,
    unknown14: u32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct MeshZUnknown13Unknown1 {
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
struct MeshZUnknown13 {
    unknown0s: FixedVec<u32, 12>,
    unknown1s: PascalArray<MeshZUnknown13Unknown1>,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct MeshZUnknown16 {
    unknown0: u32,
    unknown1: u32,
    unknown2: u32,
    unknown3: u32,
    unknown4: u32,
    unknown5: u32,
    unknown6: u32,
    unknown7: u32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct MeshZPair {
    first: u16,
    second: u16,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct MeshZUnknown15 {
    unknown0: u32,
    unknown1: u32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct MeshZUnknown14 {
    name: PascalArray<u8>,
    unknown1: u32,
    unknown2: u16,
    unknown4s: PascalArray<u16>,
    unknown15s: PascalArray<MeshZUnknown15>,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct MeshZUnknown12 {
    u0: u16,
    u1: u32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct MeshZ {
    vecs: PascalArray<Vec3f>,
    unknown0s: PascalArray<MeshZUnknown0>,
    unknown1s: PascalArray<MeshZUnknown1>,
    vertices1: PascalArray<Vec3f>,
    unknown2s: PascalArray<MeshZUnknown2>,
    // if (someHeaderValue)
    // {
    //     PascalArray<std::uint32_t> unknown3s;
    // }
    unknown4s: PascalArray<MeshZUnknown4>,
    material_crc32s: PascalArray<u32>,
    unknown6s: PascalArray<MeshZUnknown6>,
    unknown7s: PascalArray<MeshZUnknown7>,
    unknown8s: PascalArray<MeshZUnknown6>,
    sub_meshes: PascalArray<MeshZSubMesh>,
    indices: PascalArray<MeshZIndices>,
    unknown11s: PascalArray<MeshZUnknown11>,
    unknown13s: PascalArray<MeshZUnknown13>,
    unknown16s: PascalArray<MeshZUnknown16>,
    pairs: PascalArray<MeshZPair>,
    unknown18s: PascalArray<u16>,
    unknown14s: PascalArray<MeshZUnknown14>,
    unknown12s: PascalArray<MeshZUnknown12>,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct MeshZAlt {
    vecs: PascalArray<Vec3f>,
    unknown0s: PascalArray<MeshZUnknown0>,
    unknown1s: PascalArray<MeshZUnknown1>,
    vertices1: PascalArray<Vec3f>,
    unknown2s: PascalArray<MeshZUnknown2>,
    // if (someHeaderValue)
    // {
    //     PascalArray<std::uint32_t> unknown3s;
    // }
    unknown4s: PascalArray<MeshZUnknown4>,
    material_crc32s: PascalArray<u32>,
    unknown6s: PascalArray<MeshZUnknown6>,
    unknown7s: PascalArray<MeshZUnknown7>,
    unknown8s: PascalArray<MeshZUnknown6>,
    sub_meshes: PascalArray<MeshZSubMesh>,
    indices: PascalArray<MeshZIndices>,
    unknown11s: PascalArray<MeshZUnknown11>,
    unknown13s: PascalArray<MeshZUnknown13>,
    unknown12s: PascalArray<MeshZUnknown12>,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct MeshZAltAlt {
    vecs: PascalArray<Vec3f>,
    unknown0s: PascalArray<MeshZUnknown0>,
    unknown1s: PascalArray<MeshZUnknown1>,
    vertices1: PascalArray<Vec3f>,
    unknown2s: PascalArray<MeshZUnknown2>,
    // if (someHeaderValue)
    // {
    //     PascalArray<std::uint32_t> unknown3s;
    // }
    unknown4s: PascalArray<MeshZUnknown4>,
    material_crc32s: PascalArray<u32>,
    unknown6s: PascalArray<MeshZUnknown6>,
    unknown7s: PascalArray<MeshZUnknown7>,
    unknown8s: PascalArray<MeshZUnknown6>,
    sub_meshes: PascalArray<MeshZSubMesh>,
    indices: PascalArray<MeshZIndices>,
    unknown11s: PascalArray<MeshZUnknown11>,
    unknown13s: PascalArray<MeshZUnknown13>,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct MeshZAltAltAltUnknown11 {
    unknown0: u32,
    unknown1: u32,
    unknown2: u32,
    unknown3: u32,
    unknown4: u32,
    unknown5: u32,
    unknown6: u32,
    unknown7: u32,
    unknown8: u32,
    unknown9: u32,
    unknown10: u32,
    unknown11: u32,
    unknown12: u32,
    unknown13: u32,
    unknown14: u32,
    unknown15: u32,
    unknown16: u32,
    unknown17: u32,
    unknown18: u32,
    unknown19: u32,
    unknown20: u32,
    unknown21: u32,
    unknown22: u32,
    unknown23: u32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct MeshZAltAltAlt {
    vecs: PascalArray<Vec3f>,
    unknown0s: PascalArray<MeshZUnknown0>,
    material_crc32s0: PascalArray<u32>,
    unknown1s: PascalArray<MeshZUnknown1>,
    vertices1: PascalArray<Vec3f>,
    unknown2s: PascalArray<MeshZUnknown2>,
    // if (someHeaderValue)
    // {
    //     PascalArray<std::uint32_t> unknown3s;
    // }
    unknown4s: PascalArray<MeshZUnknown4>,
    material_crc32s1: PascalArray<u32>,
    unknown6s: PascalArray<MeshZUnknown6>,
    unknown7s: PascalArray<MeshZUnknown7>,
    unknown8s: PascalArray<MeshZUnknown6>,
    sub_meshes: PascalArray<MeshZSubMesh>,
    indices: PascalArray<MeshZIndices>,
    unknown11s: PascalArray<MeshZAltAltAltUnknown11>,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct MeshZHeaderUnknown3 {
    unknown0: u32,
    unknown1: u32,
    unknown2: u32,
    unknown3: u32,
    unknown4: u32,
    unknown5: u32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct MeshZHeaderUnknown4 {
    unknown0s: FixedVec<u8, 64>,
    unknown1: u32,
    unknown2: u32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct MeshZHeader {
    friendly_name_crc32: u32,
    crc32_or_zero: u32,
    rot: Quat,
    transform: Mat4f,
    unknown3: f32,
    unknown4: f32,
    unknown5: u16,
    crc32s: PascalArray<u32>,
    unknown0: u32,
    unknown1: u32,
    unknown2: u32,
    unknown3s: PascalArray<MeshZHeaderUnknown3>,
    unknown4s: PascalArray<MeshZHeaderUnknown4>,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct MeshZHeaderAlt {
    friendly_name_crc32: u32,
    crc32_or_zero: u32,
    rot: Quat,
    transform: Mat4f,
    unknown3: f32,
    unknown4: f32,
    unknown5: u16,
    crc32s: PascalArray<u32>,
    unknown0: u32,
    unknown1: u32,
    unknown2: u32,
    unknown3s: PascalArray<MeshZHeaderUnknown3>,
    unknown4s: PascalArray<MeshZHeaderUnknown4>,
    zeros: FixedVec<u32, 4>,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct MeshZHeaderAltAltUnknown10 {
    unknown0: u32,
    unknown1s: Vec3f,
    unknown2: u32,
    unknown3: u32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct MeshZHeaderAltAltUnknown4 {
    unknown0: u32,
    unknown1: u16,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct MeshZHeaderAltAltUnknown5 {
    unknown0: u32,
    unknown1: u32,
    unknown2: u32,
    unknown3: u32,
    unknown4: u32,
    unknown5: u32,
    unknown6: u32,
    unknown7: u32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct MeshZHeaderAltAltUnknown8 {
    name: PascalArray<u8>,
    unknown0: u32,
    unknown1flag: u16,
    unknown1s: PascalArray<u16>,
    unknown2s: PascalArray<Vec4f>,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct MeshZHeaderAltAlt {
    friendly_name_crc32: u32,
    crc32s: PascalArray<u32>,
    rot: Quat,
    transform: Mat4f,
    unknown2: f32,
    unknown0: f32,
    unknown1: u16,
    unknown3: Vec4f,
    unknown4: u32,
    unknown5: u32,
    unknown6: u32,
    unknown7: u32,
    unknown10s: PascalArray<MeshZHeaderAltAltUnknown10>,
    unknown8: u32,
    unknown9: u32,
    unknown4s: PascalArray<MeshZHeaderAltAltUnknown4>,
    unknown5s: PascalArray<MeshZHeaderAltAltUnknown5>,
    unknown6s: PascalArray<u32>,
    unknown7s: PascalArray<u16>,
    unknown8s: PascalArray<MeshZHeaderAltAltUnknown8>,
}

pub type MeshObjectFormat = FUELObjectFormat<MeshZHeader, MeshZ>;
pub type MeshObjectFormatAlt = FUELObjectFormat<MeshZHeaderAlt, MeshZAlt>;
pub type MeshObjectFormatAltAlt = FUELObjectFormat<MeshZHeaderAltAlt, MeshZAltAlt>;
pub type MeshObjectFormatAltAltAlt = FUELObjectFormat<MeshZHeaderAltAlt, MeshZAltAltAlt>;
