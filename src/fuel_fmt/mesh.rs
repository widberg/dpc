use std::io::Write;
use binwrite::{BinWrite, WriterOption};
use nom::{count, IResult};
use nom_derive::NomLE;
use nom_derive::Parse;
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::{FUELObjectFormat, FixedVec, HasReferences, Mat4f, PascalArray, Quat, Vec3f, Vec4f, Vec2f, FadeDistances, RangeBeginEnd, RangeBeginSize, PascalString, NumeratorFloat, Vec3, VertexVectorComponent, DynSphere, DynBox};

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct Unused0 {
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
struct Strip {
    strip_vertices_indices: PascalArray<u16>,
    material_name: u32,
    tri_order: u32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct Unused4 {
    unknown0s: PascalArray<MeshZUnknown1>,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct CollisionAABB {
    min: Vec3f,
    collision_aabb_range: RangeBeginEnd,
    max: Vec3f,
    collision_faces_range: RangeBeginSize,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct CollisionFace {
    short_vec_weirds_indices: FixedVec<u16, 3>,
    surface_type: u16,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct VertexLayoutPosition {
    position: Vec3f,
}

type VertexVector3u8 = Vec3<VertexVectorComponent>;

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct VertexLayoutNoBlend {
    position: Vec3f,
    tangent: VertexVector3u8,
    pad0: u8,
    normal: VertexVector3u8,
    pad1: u8,
    uv: Vec2f,
    luv: Vec2f,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct VertexBlendIndex {
    index: f32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct VertexLayout1Blend {
    position: Vec3f,
    tangent: VertexVector3u8,
    pad0: u8,
    normal: VertexVector3u8,
    pad1: u8,
    uv: Vec2f,
    blend_index: VertexBlendIndex,
    pad2: FixedVec<i32, 3>,
    blend_weight: f32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct VertexLayout4Blend {
    position: Vec3f,
    tangent: VertexVector3u8,
    pad0: u8,
    normal: VertexVector3u8,
    pad1: u8,
    uv: Vec2f,
    blend_indies: FixedVec<VertexBlendIndex, 4>,
    blend_weights: FixedVec<f32, 4>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum VertexBufferData {
    VertexLayout4BlendCase(Vec<VertexLayout4Blend>),
    VertexLayout1BlendCase(Vec<VertexLayout1Blend>),
    VertexLayoutNoBlendCase(Vec<VertexLayoutNoBlend>),
    VertexLayoutPositionCase(Vec<VertexLayoutPosition>),
}

impl VertexBufferData {
    fn parse(i: &[u8], vertex_size: u32, vertex_count: usize) -> IResult<&[u8], VertexBufferData> {
        match vertex_size {
            60 => {
                let parse_result = count!(i, VertexLayout4Blend::parse, vertex_count)?;
                Ok((parse_result.0, VertexBufferData::VertexLayout4BlendCase(parse_result.1)))
            }
            48 => {
                let parse_result = count!(i, VertexLayout1Blend::parse, vertex_count)?;
                Ok((parse_result.0, VertexBufferData::VertexLayout1BlendCase(parse_result.1)))
            }
            36 => {
                let parse_result = count!(i, VertexLayoutNoBlend::parse, vertex_count)?;
                Ok((parse_result.0, VertexBufferData::VertexLayoutNoBlendCase(parse_result.1)))
            }
            12 => {
                let parse_result = count!(i, VertexLayoutPosition::parse, vertex_count)?;
                Ok((parse_result.0, VertexBufferData::VertexLayoutPositionCase(parse_result.1)))
            }
            _ => { panic!("Invalid vertex size") }
        }
    }
}

impl BinWrite for VertexBufferData {
    fn write<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        match self {
            VertexBufferData::VertexLayout4BlendCase(data) => { data.write(writer) }
            VertexBufferData::VertexLayout1BlendCase(data) => { data.write(writer) }
            VertexBufferData::VertexLayoutNoBlendCase(data) => { data.write(writer) }
            VertexBufferData::VertexLayoutPositionCase(data) => { data.write(writer) }
        }
    }

    fn write_options<W: Write>(&self, writer: &mut W, options: &WriterOption) -> std::io::Result<()> {
        match self {
            VertexBufferData::VertexLayout4BlendCase(data) => { data.write_options(writer, options) }
            VertexBufferData::VertexLayout1BlendCase(data) => { data.write_options(writer, options) }
            VertexBufferData::VertexLayoutNoBlendCase(data) => { data.write_options(writer, options) }
            VertexBufferData::VertexLayoutPositionCase(data) => { data.write_options(writer, options) }
        }
    }
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[serde(from = "VertexBufferExtShadow")]
struct VertexBufferExt {
    #[serde(skip)]
    vertex_count: u32,
    #[serde(skip)]
    vertex_size: u32,
    vertex_buffer_id: u32,
    #[nom(Parse="{ |i| VertexBufferData::parse(i, vertex_size, vertex_count as usize) }")]
    vertices: VertexBufferData,
}

#[derive(Deserialize)]
struct VertexBufferExtShadow {
    vertex_buffer_id: u32,
    vertices: VertexBufferData,
}

impl From<VertexBufferExtShadow> for VertexBufferExt {
    fn from(shadow: VertexBufferExtShadow) -> Self {
        let (vertex_count, vertex_size) = match &shadow.vertices {
            VertexBufferData::VertexLayout4BlendCase(data) => (data.len(), 60),
            VertexBufferData::VertexLayout1BlendCase(data) => (data.len(), 48),
            VertexBufferData::VertexLayoutNoBlendCase(data) => (data.len(), 36),
            VertexBufferData::VertexLayoutPositionCase(data) => (data.len(), 12),
        };
        VertexBufferExt {
            vertex_count: vertex_count as u32,
            vertex_size,
            vertex_buffer_id: shadow.vertex_buffer_id,
            vertices: shadow.vertices,
        }
    }
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[serde(from = "IndexBufferExtShadow")]
struct IndexBufferExt {
    #[serde(skip)]
    index_count: u32,
    index_buffer_id: u32,
    #[nom(Count(index_count))]
    indices: Vec<u16>,
}

#[derive(Deserialize)]
struct IndexBufferExtShadow {
    index_buffer_id: u32,
    indices: Vec<u16>,
}

impl From<IndexBufferExtShadow> for IndexBufferExt {
    fn from(shadow: IndexBufferExtShadow) -> Self {
        IndexBufferExt {
            index_count: shadow.indices.len() as u32,
            index_buffer_id: shadow.index_buffer_id,
            indices: shadow.indices,
        }
    }
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct Quad {
    vertices: FixedVec<Vec3f, 4>,
    normal: Vec3f,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct MeshZVertexGroupUnused1 {
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
struct VertexGroup {
    vertex_buffer_index: u32,
    index_buffer_index: u32,
    quad_range: RangeBeginSize<u32>,
    flags: u32,
    vertex_buffer_range: RangeBeginEnd,
    vertex_count: u32,
    index_buffer_index_begin: u32,
    face_count: u32,
    zero: u32,
    vertex_buffer_range_begin_or_zero: u32,
    vertex_size: u16,
    material_index: i16,
    unuseds1: PascalArray<MeshZVertexGroupUnused1>,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct AABBMorphTrigger {
    min: Vec3f,
    aabb_morph_triggers_range: RangeBeginEnd,
    max: Vec3f,
    map_index_range: RangeBeginSize,
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
struct DisplacementVector {
    displacement: Vec3<NumeratorFloat<i16, 1024>>,
    displacement_vectors_self_index: u16,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct MorphTargetDesc {
    name: PascalString,
    base_vertex_buffer_id: u32,
    displacement_vertex_buffer_index: u16,
    displacement_vectors_indicies: PascalArray<u16>,
    displacement_vectors: PascalArray<DisplacementVector>,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct Morpher {
    aabb_morph_triggers: PascalArray<AABBMorphTrigger>,
    map: PascalArray<MeshZPair>,
    displacement_vectors_indices: PascalArray<u16>,
    morphs: PascalArray<MorphTargetDesc>,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct MeshBuffers {
    vertex_buffers: PascalArray<VertexBufferExt>,
    index_buffers: PascalArray<IndexBufferExt>,
    quads: PascalArray<Quad>,
    vertex_groups: PascalArray<VertexGroup>,
    morpher: Morpher,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct MeshZUnknown12 {
    u0: u16,
    u1: u16,
    u2: u16,
}

type ShortVecWeird = Vec3<NumeratorFloat<i16, 1024>>;

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct MeshZ {
    strip_vertices: PascalArray<Vec3f>,
    unused0s: PascalArray<Unused0>,
    texcoords: PascalArray<Vec2f>,
    normals: PascalArray<Vec3f>,
    strips: PascalArray<Strip>,
    // if (someHeaderValue)
    // {
    //     PascalArray<std::uint32_t> unknown3s;
    // }
    unused4s: PascalArray<Unused4>,
    material_crc32s: PascalArray<u32>,
    collision_aabbs: PascalArray<CollisionAABB>,
    collision_faces: PascalArray<CollisionFace>,
    unused8s: PascalArray<CollisionAABB>,
    mesh_buffers: MeshBuffers,
    short_vec_weirds: PascalArray<ShortVecWeird>,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct MeshZAlt {
    vecs: PascalArray<Vec3f>,
    unknown0s: PascalArray<Unused0>,
    unknown1s: PascalArray<MeshZUnknown1>,
    vertices1: PascalArray<Vec3f>,
    unknown2s: PascalArray<Strip>,
    // if (someHeaderValue)
    // {
    //     PascalArray<std::uint32_t> unknown3s;
    // }
    unknown4s: PascalArray<Unused4>,
    material_crc32s: PascalArray<u32>,
    unknown6s: PascalArray<CollisionAABB>,
    unknown7s: PascalArray<CollisionFace>,
    unknown8s: PascalArray<CollisionAABB>,
    sub_meshes: PascalArray<VertexBufferExt>,
    indices: PascalArray<IndexBufferExt>,
    unknown11s: PascalArray<Quad>,
    unknown13s: PascalArray<VertexGroup>,
    unknown12s: PascalArray<MeshZUnknown12>,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct MeshZAltAlt {
    vecs: PascalArray<Vec3f>,
    unknown0s: PascalArray<Unused0>,
    unknown1s: PascalArray<MeshZUnknown1>,
    vertices1: PascalArray<Vec3f>,
    unknown2s: PascalArray<Strip>,
    // if (someHeaderValue)
    // {
    //     PascalArray<std::uint32_t> unknown3s;
    // }
    unknown4s: PascalArray<Unused4>,
    material_crc32s: PascalArray<u32>,
    unknown6s: PascalArray<CollisionAABB>,
    unknown7s: PascalArray<CollisionFace>,
    unknown8s: PascalArray<CollisionAABB>,
    sub_meshes: PascalArray<VertexBufferExt>,
    indices: PascalArray<IndexBufferExt>,
    unknown11s: PascalArray<Quad>,
    unknown13s: PascalArray<VertexGroup>,
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
    unknown0s: PascalArray<Unused0>,
    material_crc32s0: PascalArray<u32>,
    unknown1s: PascalArray<MeshZUnknown1>,
    vertices1: PascalArray<Vec3f>,
    unknown2s: PascalArray<Strip>,
    // if (someHeaderValue)
    // {
    //     PascalArray<std::uint32_t> unknown3s;
    // }
    unknown4s: PascalArray<Unused4>,
    material_crc32s1: PascalArray<u32>,
    unknown6s: PascalArray<CollisionAABB>,
    unknown7s: PascalArray<CollisionFace>,
    unknown8s: PascalArray<CollisionAABB>,
    sub_meshes: PascalArray<VertexBufferExt>,
    indices: PascalArray<IndexBufferExt>,
    unknown11s: PascalArray<MeshZAltAltAltUnknown11>,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct MeshZHeader {
    link_name: u32,
    data_name: u32,
    rot: Quat,
    transform: Mat4f,
    radius: f32,
    flags: u32,
    typ: u16,
    crc32s: PascalArray<u32>,
    fade: FadeDistances,
    dyn_spheres: PascalArray<DynSphere>,
    dyn_boxes: PascalArray<DynBox>,
}

impl HasReferences for MeshZHeader {
    fn hard_links(&self) -> Vec<u32> {
        vec![]
    }

    fn soft_links(&self) -> Vec<u32> {
        let mut crc32s = self.crc32s.data.clone();
        crc32s.push(self.data_name);
        crc32s
    }
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
    unknown3s: PascalArray<DynSphere>,
    unknown4s: PascalArray<DynBox>,
    zeros: FixedVec<u32, 4>,
}

impl HasReferences for MeshZHeaderAlt {
    fn hard_links(&self) -> Vec<u32> {
        vec![]
    }

    fn soft_links(&self) -> Vec<u32> {
        let mut crc32s = self.crc32s.data.clone();
        crc32s.push(self.crc32_or_zero);
        crc32s
    }
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

impl HasReferences for MeshZHeaderAltAlt {
    fn hard_links(&self) -> Vec<u32> {
        vec![]
    }

    fn soft_links(&self) -> Vec<u32> {
        self.crc32s.data.clone()
    }
}

impl HasReferences for MeshZ {
    fn hard_links(&self) -> Vec<u32> {
        self.material_crc32s.data.clone()
    }

    fn soft_links(&self) -> Vec<u32> {
        vec![]
    }
}

impl HasReferences for MeshZAlt {
    fn hard_links(&self) -> Vec<u32> {
        self.material_crc32s.data.clone()
    }

    fn soft_links(&self) -> Vec<u32> {
        vec![]
    }
}

impl HasReferences for MeshZAltAlt {
    fn hard_links(&self) -> Vec<u32> {
        self.material_crc32s.data.clone()
    }

    fn soft_links(&self) -> Vec<u32> {
        vec![]
    }
}

impl HasReferences for MeshZAltAltAlt {
    fn hard_links(&self) -> Vec<u32> {
        [
            &self.material_crc32s0.data[..],
            &self.material_crc32s1.data[..],
        ]
            .concat()
    }

    fn soft_links(&self) -> Vec<u32> {
        vec![]
    }
}

pub type MeshObjectFormat = FUELObjectFormat<MeshZHeader, MeshZ>;
pub type MeshObjectFormatAlt = FUELObjectFormat<MeshZHeaderAlt, MeshZAlt>;
pub type MeshObjectFormatAltAlt = FUELObjectFormat<MeshZHeaderAltAlt, MeshZAltAlt>;
pub type MeshObjectFormatAltAltAlt = FUELObjectFormat<MeshZHeaderAltAlt, MeshZAltAltAlt>;
