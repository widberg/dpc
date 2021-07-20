pub use nom::number::complete::*;
pub use nom_derive::NomLE;
use nom_derive::Parse;
pub use serde::{Deserialize, Deserializer, Serialize, Serializer};

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
pub struct Vec4f {
    x: f32,
    z: f32,
    y: f32,
    w: f32,
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
    data: Vec<f32>,
}

#[derive(Serialize, Deserialize, NomLE)]
pub struct Quat {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

#[derive(NomLE)]
pub struct PascalArray<T> {
    #[nom(LengthCount(le_u32))]
    data: Vec<T>,
}

impl<T> PascalArray<T> {
    pub fn len(self: &Self) -> usize {
        self.data.len()
    }
}

impl<T> Serialize for PascalArray<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.data.serialize(serializer)
    }
}

impl<'de, T> Deserialize<'de> for PascalArray<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(PascalArray {
            data: Vec::deserialize(deserializer)?,
        })
    }
}

#[derive(NomLE)]
pub struct FixedVec<T, const U: usize> {
    #[nom(Count(U))]
    data: Vec<T>,
}

impl<T, const U: usize> Serialize for FixedVec<T, U>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.data.serialize(serializer)
    }
}

impl<'de, T, const U: usize> Deserialize<'de> for FixedVec<T, U>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(FixedVec {
            data: Vec::deserialize(deserializer)?,
        })
    }
}

#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct ObjectZ {
    friendly_name_crc32: u32,
    crc32_or_zero: u32,
    #[nom(Cond = "i.len() != 90", Count = "crc32_or_zero as usize + 1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    crc32s: Option<Vec<u32>>,
    rot: Quat,
    transform: Mat4f,
    unknown2: f32,
    unknown0: f32,
    unknown1: u16,
}
