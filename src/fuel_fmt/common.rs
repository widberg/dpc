pub use nom_derive::{NomLE};
pub use serde::{Serialize, Deserialize};
pub use nom::number::complete::*;

#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct ResourceObjectZ {
    friendly_name_crc32: u32,
    #[nom(Cond = "i.len() != 0")]
    #[nom(LengthCount = "le_u32")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crc32s: Option<Vec<u32>>,
}

#[derive(Serialize, Deserialize, NomLE)]
pub struct Vec3f {
    x: f32,
    z: f32,
    y: f32,
}

#[derive(Serialize, Deserialize, NomLE)]
pub struct Vec3i32 {
    x: i32,
    z: i32,
    y: i32,
}

#[derive(Serialize, Deserialize, NomLE)]
pub struct Vec2f {
    x: f32,
    y: f32,
}

#[derive(Serialize, Deserialize, NomLE)]
pub struct Mat4f {
    #[nom(Count(16))]
    data: Vec<u32>,
}

#[derive(Serialize, Deserialize, NomLE)]
pub struct Quad {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct ObjectZ {
    friendly_name_crc32: u32,
    crc32_or_zero: u32,
    #[nom(Cond = "i.len() != 90", Count = "crc32_or_zero as usize + 1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    crc32s: Option<Vec<u32>>,
    rot: Quad,
    transform: Mat4f,
    unknown2: f32,
    unknown0: f32,
    unknown1: u16,
}
